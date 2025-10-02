use crate::model::{run_pipeline, SimulationInput, SimulationOutput};
use crate::presets::FramePreset;
use chimera_core::diagnostics::{DiagnosticsBundle, ModulationAudio, SymbolDecision};
use js_sys::ArrayBuffer;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::console;
use web_sys::{
    AudioBuffer, AudioContext, AudioContextState, Document, HtmlCanvasElement, HtmlInputElement,
    HtmlSelectElement, HtmlTextAreaElement,
};
use yew::events::Event;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(SimulationInput::default);
    let output = use_state(|| run_pipeline((*input).clone()));

    let on_submit = {
        let output = output.clone();
        let input = input.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            output.set(run_pipeline((*input).clone()));
        })
    };

    let on_plaintext_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlTextAreaElement>() {
                let mut updated = (*input).clone();
                updated.plaintext = target.value();
                input.set(updated);
            }
        })
    };

    let on_snr_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = target.value().parse::<f64>() {
                    let mut updated = (*input).clone();
                    updated.snr_db = value;
                    input.set(updated);
                }
            }
        })
    };

    let on_sample_rate_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = target.value().parse::<usize>() {
                    let mut updated = (*input).clone();
                    updated.sample_rate = value;
                    input.set(updated);
                }
            }
        })
    };

    let on_preset_change = {
        let input = input.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                if let Some(preset) = FramePreset::from_key(&target.value()) {
                    let mut updated = (*input).clone();
                    let previous_defaults = updated.preset.simulation_config();
                    let defaults = preset.simulation_config();
                    if updated.plaintext == previous_defaults.plaintext_source {
                        updated.plaintext = defaults.plaintext_source.clone();
                    }
                    updated.preset = preset;
                    updated.snr_db = defaults.snr_db;
                    updated.sample_rate = defaults.sample_rate;
                    input.set(updated);
                }
            }
        })
    };

    let current_input = (*input).clone();
    let current_output = (*output).clone();
    let active_preset = current_input.preset;

    html! {
        <div class="app-container">
            <h1>{"Chimera Visualizer"}</h1>
            <form class="controls" onsubmit={on_submit}>
                <label for="plaintext">{"Plaintext"}</label>
                <textarea
                    id="plaintext"
                    value={current_input.plaintext.clone()}
                    oninput={on_plaintext_change}
                />

                <label for="preset">{"Frame preset"}</label>
                <select
                    id="preset"
                    value={AttrValue::from(active_preset.key())}
                    onchange={on_preset_change}
                >
                    {for FramePreset::all().iter().map(|preset| {
                        let label = preset.display_name();
                        let value = preset.key();
                        let selected = *preset == active_preset;
                        html! {
                            <option value={AttrValue::from(value)} selected={selected}>{label}</option>
                        }
                    })}
                </select>
                <p class="preset-description">{active_preset.description()}</p>

                <label for="snr">{"SNR (dB)"}</label>
                <input
                    id="snr"
                    type="number"
                    step="0.5"
                    value={current_input.snr_db.to_string()}
                    oninput={on_snr_change}
                />

                <label for="sample-rate">{"Sample rate (Hz)"}</label>
                <input
                    id="sample-rate"
                    type="number"
                    min="1"
                    step="1"
                    value={current_input.sample_rate.to_string()}
                    oninput={on_sample_rate_change}
                />

                <button type="submit">{"Run Simulation"}</button>
            </form>

            <div class="dashboard">
                <StatsPanel input={current_input.clone()} output={current_output.clone()} />
                <ConstellationChart diagnostics={current_output.diagnostics.clone()} />
                <AudioPanel audio={current_output.diagnostics.modulation_audio.clone()} />
                <SymbolDecisionPanel decisions={current_output.diagnostics.demodulation.symbol_decisions.clone()} />
                <LogsPanel title={AttrValue::from("Encoder Logs")} entries={current_output.diagnostics.encoding_logs.clone()} />
                <LogsPanel title={AttrValue::from("Decoder Logs")} entries={current_output.diagnostics.decoding_logs.clone()} />
            </div>
        </div>
    }
}

async fn resume_context(ctx: &AudioContext) {
    if ctx.state() == AudioContextState::Suspended {
        if let Ok(promise) = ctx.resume() {
            let _ = JsFuture::from(promise).await;
        }
    }
}

fn ensure_audio_context(
    handle: &UseStateHandle<Option<AudioContext>>,
) -> Result<AudioContext, JsValue> {
    if let Some(ctx) = handle.as_ref() {
        Ok(ctx.clone())
    } else {
        let ctx = AudioContext::new()?;
        handle.set(Some(ctx.clone()));
        Ok(ctx)
    }
}

fn play_samples(ctx: &AudioContext, samples: &[f32], sample_rate: f32) -> Result<(), JsValue> {
    if samples.is_empty() {
        return Ok(());
    }

    let buffer = ctx.create_buffer(1, samples.len() as u32, sample_rate)?;
    let mut data = samples.to_vec();
    buffer.copy_to_channel(&mut data, 0)?;

    let source = ctx.create_buffer_source()?;
    source.set_buffer(Some(&buffer));
    source.connect_with_audio_node(&ctx.destination())?;
    source.start()?;
    Ok(())
}

fn mixdown_audio_buffer(buffer: &AudioBuffer) -> Result<Vec<f32>, JsValue> {
    let channels = buffer.number_of_channels();
    let length = buffer.length() as usize;
    if length == 0 {
        return Ok(Vec::new());
    }

    let mut mixdown = vec![0.0_f32; length];
    for channel in 0..channels {
        let mut data = vec![0.0_f32; length];
        buffer.copy_from_channel(&mut data, channel as i32)?;
        for (idx, sample) in data.iter().enumerate() {
            mixdown[idx] += *sample;
        }
    }

    if channels > 0 {
        let scale = 1.0 / channels as f32;
        for sample in mixdown.iter_mut() {
            *sample *= scale;
        }
    }

    Ok(mixdown)
}

fn normalize_samples(samples: &mut [f32]) {
    let mut max_amp = 0.0_f32;
    for &value in samples.iter() {
        max_amp = max_amp.max(value.abs());
    }

    if max_amp > 1.0 {
        let scale = 0.98_f32 / max_amp;
        for sample in samples.iter_mut() {
            *sample *= scale;
        }
    }
}

fn resample_external(
    samples: &[f32],
    source_rate: f32,
    target_rate: f32,
    target_len: usize,
) -> Vec<f32> {
    if target_len == 0 {
        return Vec::new();
    }
    if samples.is_empty() {
        return vec![0.0; target_len];
    }
    if source_rate <= 0.0 || target_rate <= 0.0 {
        return samples.iter().cycle().take(target_len).copied().collect();
    }

    let ratio = source_rate / target_rate;
    let src_len = samples.len();
    let mut output = Vec::with_capacity(target_len);

    for i in 0..target_len {
        let src_pos = (i as f32 * ratio) % src_len as f32;
        let idx = src_pos.floor() as usize;
        let frac = src_pos - idx as f32;
        let next_idx = (idx + 1) % src_len;
        let sample = samples[idx] * (1.0 - frac) + samples[next_idx] * frac;
        output.push(sample);
    }

    output
}

fn nonlinear_mix(mod_signal: &[f32], carrier: &[f32], wet: f32) -> Vec<f32> {
    let mut output = Vec::with_capacity(mod_signal.len());
    let wet = wet.clamp(0.0, 1.0);
    let dry = 1.0 - wet;

    for (&mod_sample, &carrier_sample) in mod_signal.iter().zip(carrier.iter()) {
        let vocoded = (mod_sample * carrier_sample).tanh();
        output.push(wet * vocoded + dry * mod_sample);
    }

    output
}

#[derive(Properties, PartialEq)]
pub struct StatsPanelProps {
    pub input: SimulationInput,
    pub output: SimulationOutput,
}

#[function_component(StatsPanel)]
pub fn stats_panel(props: &StatsPanelProps) -> Html {
    let report = &props.output.report;
    let preset = props.input.preset;
    let protocol = preset.protocol_config();
    let layout = &protocol.frame_layout;
    let baseline_sim = preset.simulation_config();
    let symbol_decisions = props.output.diagnostics.demodulation.symbol_decisions.len();

    html! {
        <div class="stats-panel">
            <h2>{"Simulation Results"}</h2>
            <ul>
                <li>{format!("Preset: {}", preset.display_name())}</li>
                <li class="preset-description">{preset.description()}</li>
                <li>{format!(
                    "Frame layout: total {} (sync {} · target {} · command {} · data {} · ecc {})",
                    layout.total_symbols,
                    layout.sync_symbols,
                    layout.target_id_symbols,
                    layout.command_type_symbols,
                    layout.data_payload_symbols,
                    layout.ecc_symbols
                )}</li>
                <li>{format!(
                    "Symbol rate: {} sym/s · Bandwidth {:.1} Hz",
                    protocol.qpsk_symbol_rate,
                    protocol.qpsk_bandwidth_hz
                )}</li>
                <li>{format!(
                    "SNR configured: {:.1} dB (baseline {:.1} dB)",
                    props.input.snr_db,
                    baseline_sim.snr_db
                )}</li>
                <li>{format!(
                    "Sample rate: {} Hz (baseline {} Hz)",
                    props.input.sample_rate,
                    baseline_sim.sample_rate
                )}</li>
                <li>{format!("Symbol decisions captured: {}", symbol_decisions)}</li>
                <li>{format!("Pre-FEC errors: {} (BER {:.6})", report.pre_fec_errors, report.pre_fec_ber)}</li>
                <li>{format!("Post-FEC errors: {} (BER {:.6})", report.post_fec_errors, report.post_fec_ber)}</li>
                <li>{format!("Recovered message: {}", report.recovered_message)}</li>
            </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ConstellationProps {
    pub diagnostics: DiagnosticsBundle,
}

#[function_component(ConstellationChart)]
pub fn constellation_chart(props: &ConstellationProps) -> Html {
    let canvas_ref = use_node_ref();
    {
        let canvas_ref = canvas_ref.clone();
        let symbols_i = props.diagnostics.demodulation.received_symbols_i.clone();
        let symbols_q = props.diagnostics.demodulation.received_symbols_q.clone();

        use_effect_with(
            (symbols_i.clone(), symbols_q.clone()),
            move |(i_samples, q_samples): &(Vec<f64>, Vec<f64>)| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_constellation(&canvas, i_samples, q_samples);
                }
                || ()
            },
        );
    }

    html! {
        <div class="constellation-panel">
            <h2>{"Constellation"}</h2>
            <canvas ref={canvas_ref} width="320" height="320" />
        </div>
    }
}

fn draw_constellation(canvas: &HtmlCanvasElement, symbols_i: &[f64], symbols_q: &[f64]) {
    if let Some(backend) = CanvasBackend::with_canvas_object(canvas.clone()) {
        let root = backend.into_drawing_area();
        if root.fill(&WHITE).is_err() {
            return;
        }

        let area = root.margin(10, 10, 10, 10);
        if symbols_i.is_empty() || symbols_q.is_empty() {
            return;
        }

        if let Ok(mut chart) = ChartBuilder::on(&area)
            .caption("Received Constellation", ("sans-serif", 20))
            .build_cartesian_2d(-1.5..1.5, -1.5..1.5)
        {
            let _ = chart.configure_mesh().x_labels(5).y_labels(5).draw();

            let symbols = symbols_i
                .iter()
                .zip(symbols_q.iter())
                .map(|(&i, &q)| (i, q));

            let _ = chart.draw_series(symbols.map(|(i, q)| Circle::new((i, q), 3, RED.filled())));
        }
    }
}

#[derive(Clone, PartialEq)]
struct ExternalAudio {
    name: String,
    sample_rate: f32,
    samples: Vec<f32>,
}

#[derive(Properties, PartialEq)]
pub struct AudioPanelProps {
    pub audio: Option<ModulationAudio>,
}

#[function_component(AudioPanel)]
pub fn audio_panel(props: &AudioPanelProps) -> Html {
    let context = use_state(|| None::<AudioContext>);
    let external_audio = use_state(|| None::<ExternalAudio>);
    let wet_mix = use_state(|| 0.5_f32);
    let status = use_state(|| AttrValue::from("Run the pipeline to generate audio"));

    {
        let status = status.clone();
        let external_audio = external_audio.clone();
        use_effect_with(
            props.audio.clone(),
            move |audio: &Option<ModulationAudio>| {
                if audio.is_some() {
                    status.set(AttrValue::from("Ready to play modulation audio"));
                } else {
                    status.set(AttrValue::from("Run the pipeline to generate audio"));
                    external_audio.set(None);
                }
            },
        );
    }

    let audio_opt = props.audio.clone();
    if audio_opt.is_none() {
        return html! {
            <div class="audio-panel">
                <h2>{"Modulated Audio"}</h2>
                <p>{"Audio playback is unavailable until a simulation has been run."}</p>
            </div>
        };
    }

    let audio = audio_opt.unwrap();
    let carrier_summary = format!(
        "Carrier {:.1} Hz · Sample rate {} Hz",
        audio.carrier_freq_hz, audio.sample_rate
    );
    let wet_value = *wet_mix;
    let wet_percent = format!("{:.0}%", wet_value * 100.0);
    let external_summary = (*external_audio).clone();
    let status_message = (*status).clone();

    let on_wet_change = {
        let wet_mix = wet_mix.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = target.value().parse::<f32>() {
                    wet_mix.set(value.clamp(0.0, 1.0));
                }
            }
        })
    };

    let on_audio_upload = {
        let context = context.clone();
        let external_audio = external_audio.clone();
        let status = status.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Some(files) = target.files() {
                    if let Some(file) = files.get(0) {
                        let file_name = file.name();
                        let context = context.clone();
                        let external_audio = external_audio.clone();
                        let status = status.clone();
                        spawn_local(async move {
                            status.set(AttrValue::from("Decoding uploaded audio…"));
                            match ensure_audio_context(&context) {
                                Ok(ctx) => {
                                    if let Ok(buffer) = JsFuture::from(file.array_buffer()).await {
                                        if let Ok(array_buffer) = buffer.dyn_into::<ArrayBuffer>() {
                                            match ctx.decode_audio_data(&array_buffer) {
                                                Ok(promise) => {
                                                    match JsFuture::from(promise).await {
                                                        Ok(decoded) => {
                                                            if let Ok(audio_buffer) =
                                                                decoded.dyn_into::<AudioBuffer>()
                                                            {
                                                                match mixdown_audio_buffer(
                                                                    &audio_buffer,
                                                                ) {
                                                                    Ok(mut samples) => {
                                                                        normalize_samples(
                                                                            &mut samples,
                                                                        );
                                                                        external_audio.set(Some(ExternalAudio {
                                                                            name: file_name.clone(),
                                                                            sample_rate: audio_buffer.sample_rate(),
                                                                            samples,
                                                                        }));
                                                                        status.set(format!(
                                                                            "Loaded {} ({:.1} kHz)",
                                                                            file_name,
                                                                            audio_buffer.sample_rate() / 1000.0
                                                                        ).into());
                                                                    }
                                                                    Err(err) => {
                                                                        console::error_1(&err);
                                                                        status.set("Failed to mix channels from uploaded audio".into());
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(err) => {
                                                            console::error_1(&err);
                                                            status.set(
                                                                "Unable to decode uploaded audio"
                                                                    .into(),
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    console::error_1(&err);
                                                    status
                                                        .set("Audio decode promise failed".into());
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    console::error_1(&err);
                                    status.set("Audio context unavailable".into());
                                }
                            }
                        });
                    }
                }
            }
        })
    };

    let play_clean = {
        let context = context.clone();
        let status = status.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            let context = context.clone();
            let status = status.clone();
            let audio = audio.clone();
            spawn_local(async move {
                status.set(AttrValue::from("Playing clean carrier…"));
                match ensure_audio_context(&context) {
                    Ok(ctx) => {
                        resume_context(&ctx).await;
                        if let Err(err) = play_samples(&ctx, &audio.clean, audio.sample_rate as f32)
                        {
                            console::error_1(&err);
                            status.set("Error playing clean carrier".into());
                        } else {
                            status.set("Clean carrier playback started".into());
                        }
                    }
                    Err(err) => {
                        console::error_1(&err);
                        status.set("Audio context unavailable".into());
                    }
                }
            });
        })
    };

    let play_noisy = {
        let context = context.clone();
        let status = status.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            let context = context.clone();
            let status = status.clone();
            let audio = audio.clone();
            spawn_local(async move {
                status.set(AttrValue::from("Playing noisy channel…"));
                match ensure_audio_context(&context) {
                    Ok(ctx) => {
                        resume_context(&ctx).await;
                        if let Err(err) = play_samples(&ctx, &audio.noisy, audio.sample_rate as f32)
                        {
                            console::error_1(&err);
                            status.set("Error playing noisy channel".into());
                        } else {
                            status.set("Noisy channel playback started".into());
                        }
                    }
                    Err(err) => {
                        console::error_1(&err);
                        status.set("Audio context unavailable".into());
                    }
                }
            });
        })
    };

    let play_mixed = {
        let context = context.clone();
        let status = status.clone();
        let external_audio_handle = external_audio.clone();
        let wet_mix = wet_mix.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            let maybe_external = (*external_audio_handle).clone();
            if maybe_external.is_none() {
                status.set("Upload an audio file to enable mixing".into());
                return;
            }

            let external = maybe_external.unwrap();
            let wet_value = *wet_mix;
            let context = context.clone();
            let status = status.clone();
            let audio = audio.clone();
            spawn_local(async move {
                status.set(AttrValue::from("Playing vocoder mix…"));
                match ensure_audio_context(&context) {
                    Ok(ctx) => {
                        resume_context(&ctx).await;
                        let resampled = resample_external(
                            &external.samples,
                            external.sample_rate,
                            audio.sample_rate as f32,
                            audio.noisy.len(),
                        );
                        let mut mixed = nonlinear_mix(&audio.noisy, &resampled, wet_value);
                        normalize_samples(&mut mixed);
                        if let Err(err) = play_samples(&ctx, &mixed, audio.sample_rate as f32) {
                            console::error_1(&err);
                            status.set("Error playing vocoder mix".into());
                        } else {
                            status.set(
                                format!(
                                    "Vocoder mix playback started (wet {}%)",
                                    (wet_value * 100.0).round() as i32
                                )
                                .into(),
                            );
                        }
                    }
                    Err(err) => {
                        console::error_1(&err);
                        status.set("Audio context unavailable".into());
                    }
                }
            });
        })
    };

    let has_external = external_summary.is_some();
    let external_display = external_summary.clone();

    html! {
        <div class="audio-panel">
            <h2>{"Modulated Audio"}</h2>
            <p>{carrier_summary}</p>
            <div class="audio-controls">
                <button type="button" onclick={play_clean.clone()}>{"Play Clean Carrier"}</button>
                <button type="button" onclick={play_noisy.clone()}>{"Play Noisy Channel"}</button>
                <button type="button" onclick={play_mixed.clone()} disabled={!has_external}>{"Play Vocoder Mix"}</button>
            </div>
            <label class="audio-upload">
                {"Upload audio (mp3/wav)"}
                <input type="file" accept="audio/*" onchange={on_audio_upload} />
            </label>
            <label class="audio-slider">
                {"Wet/Dry Mix"}
                <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.05"
                    value={format!("{:.2}", wet_value)}
                    oninput={on_wet_change}
                />
                <span>{wet_percent}</span>
            </label>
            {
                if let Some(external) = external_display {
                    html! { <p class="audio-status">{format!("External source: {} ({:.1} kHz)", external.name, external.sample_rate / 1000.0)}</p> }
                } else {
                    Html::default()
                }
            }
            <p class="audio-status">{status_message}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SymbolDecisionPanelProps {
    pub decisions: Vec<SymbolDecision>,
}

#[function_component(SymbolDecisionPanel)]
pub fn symbol_decision_panel(props: &SymbolDecisionPanelProps) -> Html {
    const MAX_PREVIEW: usize = 24;
    let total = props.decisions.len();
    let preview_len = total.min(MAX_PREVIEW);
    let summary_message = if total == 0 {
        String::from("No demodulated symbols captured.")
    } else if total > MAX_PREVIEW {
        format!("Showing first {} of {} symbols", preview_len, total)
    } else {
        format!("Showing {} symbols", total)
    };

    html! {
        <div class="symbol-decisions-panel">
            <h2>{"Symbol Decisions"}</h2>
            {
                if total == 0 {
                    html! { <p>{summary_message}</p> }
                } else {
                    html! {
                        <>
                            <table>
                                <thead>
                                    <tr>
                                        <th>{"Index"}</th>
                                        <th>{"Bits"}</th>
                                        <th>{"Ī"}</th>
                                        <th>{"Q̄"}</th>
                                        <th>{"Min Dist"}</th>
                                        <th>{"Soft(0)"}</th>
                                        <th>{"Soft(1)"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {for props.decisions.iter().take(MAX_PREVIEW).map(|decision| {
                                        let bits = decision.decided_bits;
                                        let bit_str = format!("{}{}", bits[0], bits[1]);
                                        html! {
                                            <tr>
                                                <td>{decision.index}</td>
                                                <td>{bit_str}</td>
                                                <td>{format!("{:+.3}", decision.average_i)}</td>
                                                <td>{format!("{:+.3}", decision.average_q)}</td>
                                                <td>{format!("{:.3}", decision.min_distance)}</td>
                                                <td>{format!("{:+.3}", decision.soft_metrics[0])}</td>
                                                <td>{format!("{:+.3}", decision.soft_metrics[1])}</td>
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                            <p class="symbol-decisions-note">{summary_message}</p>
                        </>
                    }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LogsPanelProps {
    pub title: AttrValue,
    pub entries: Vec<String>,
}

#[function_component(LogsPanel)]
pub fn logs_panel(props: &LogsPanelProps) -> Html {
    html! {
        <div class="logs-panel">
            <h2>{props.title.clone()}</h2>
            <ol>
                {for props.entries.iter().map(|entry| html! { <li>{entry}</li> })}
            </ol>
        </div>
    }
}

pub fn mount_app() {
    let window = web_sys::window().expect("window available");
    let document: Document = window.document().expect("document available");
    let root = document
        .get_element_by_id("chimera-root")
        .or_else(|| document.get_element_by_id("root"))
        .unwrap_or_else(|| document.body().expect("body element").into());

    yew::Renderer::<App>::with_root(root).render();
}
