use crate::model::{run_pipeline, SimulationInput, SimulationOutput};
use crate::presets::FramePreset;
use chimera_core::diagnostics::{DiagnosticsBundle, ModulationAudio, SymbolDecision};
use js_sys::ArrayBuffer;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use rustfft::num_complex::Complex32;
use rustfft::FftPlanner;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::console;
use web_sys::{
    AudioBuffer, AudioBufferSourceNode, AudioContext, AudioContextState, Document, GainNode,
    HtmlCanvasElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement,
};
use yew::events::Event;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(SimulationInput::default);
    let output = use_state(|| None::<SimulationOutput>);
    let is_running = use_state(|| false);

    let on_submit = {
        let output = output.clone();
        let input = input.clone();
        let is_running = is_running.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_running.set(true);
            let result = run_pipeline((*input).clone());
            output.set(Some(result));
            is_running.set(false);
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
    let is_running_now = *is_running;

    let results_view = if let Some(output) = current_output.as_ref() {
        let diagnostics = output.diagnostics.clone();
        let modulation_audio = diagnostics.modulation_audio.clone();
        let symbol_decisions = diagnostics.demodulation.symbol_decisions.clone();
        let encoder_logs = diagnostics.encoding_logs.clone();
        let decoder_logs = diagnostics.decoding_logs.clone();

        html! {
            <section class="results-grid">
                <StatsPanel input={current_input.clone()} output={output.clone()} />
                <ConstellationChart diagnostics={diagnostics.clone()} />
        <AudioPanel audio={modulation_audio} />
                <SymbolDecisionPanel decisions={symbol_decisions} />
                <LogsPanel title={AttrValue::from("Encoder Trace")} entries={encoder_logs} />
                <LogsPanel title={AttrValue::from("Decoder Trace")} entries={decoder_logs} />
            </section>
        }
    } else {
        html! {
            <section class="empty-state panel">
                <h2>{"Ready to explore Chimera"}</h2>
                <p>{"Configure a preset and press Run Simulation to watch the transmitter/receiver chain light up. The dashboard will update to show constellation, audio, error statistics, and raw decoder evidence."}</p>
                <ul>
                    <li>{"Pick a frame preset to load recommended plaintext, SNR, and sample rate."}</li>
                    <li>{"Adjust SNR to experience how the LDPC decoder handles tougher channels."}</li>
                    <li>{"Upload an audio clip later to fold it through the Chimera channel with non-linear mixing."}</li>
                </ul>
            </section>
        }
    };

    html! {
        <div class="app-shell">
            <header class="hero">
                <h1>{"Chimera Modulation Lab"}</h1>
                <p>{"Experiment with Chimera's LDPC-backed telemetry link and see every stage of the TX/RX chain, from plaintext bits to recovered payload."}</p>
            </header>

            <section class="layout-grid">
                <form class="control-panel" onsubmit={on_submit}>
                    <div class="field">
                        <label for="plaintext">{"Plaintext"}</label>
                        <textarea
                            id="plaintext"
                            value={current_input.plaintext.clone()}
                            oninput={on_plaintext_change}
                        />
                    </div>

                    <div class="field-row">
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
                    </div>

                    <div class="field-group">
                        <div class="field">
                            <label for="snr">{"SNR (dB)"}</label>
                            <input
                                id="snr"
                                type="number"
                                step="0.5"
                                value={current_input.snr_db.to_string()}
                                oninput={on_snr_change}
                            />
                        </div>
                        <div class="field">
                            <label for="sample-rate">{"Sample rate (Hz)"}</label>
                            <input
                                id="sample-rate"
                                type="number"
                                min="1"
                                step="1"
                                value={current_input.sample_rate.to_string()}
                                oninput={on_sample_rate_change}
                            />
                        </div>
                    </div>

                    <button class="primary" type="submit" disabled={is_running_now}>
                        {if is_running_now { "Simulating…" } else { "Run Simulation" }}
                    </button>
                </form>

                <PipelineVisualizer input={current_input.clone()} output={current_output.clone()} />
            </section>

            {results_view}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PipelineVisualizerProps {
    pub input: SimulationInput,
    pub output: Option<SimulationOutput>,
}

#[function_component(PipelineVisualizer)]
pub fn pipeline_visualizer(props: &PipelineVisualizerProps) -> Html {
    let preset = props.input.preset;
    let protocol = preset.protocol_config();
    let simulation_defaults = preset.simulation_config();
    let ldpc = preset.ldpc_config();

    let plaintext_len = props.input.plaintext.chars().count();
    let mut preview: String = props.input.plaintext.chars().take(96).collect();
    if preview.len() < props.input.plaintext.len() {
        preview.push_str("…");
    }

    let output = props.output.as_ref();
    let recovered_preview = output
        .map(|o| {
            let mut text: String = o.report.recovered_message.chars().take(96).collect();
            if text.len() < o.report.recovered_message.len() {
                text.push_str("…");
            }
            text
        })
        .unwrap_or_else(|| "Awaiting first run…".into());

    let symbol_decisions = output
        .map(|o| o.diagnostics.demodulation.symbol_decisions.len())
        .unwrap_or(0);
    let pre_fec = output
        .map(|o| format!("{:.6}", o.report.pre_fec_ber))
        .unwrap_or_else(|| "—".into());
    let post_fec = output
        .map(|o| format!("{:.6}", o.report.post_fec_ber))
        .unwrap_or_else(|| "—".into());
    let carrier_info = output
        .and_then(|o| o.diagnostics.modulation_audio.as_ref())
        .map(|audio| format!("{:.1} Hz", audio.carrier_freq_hz))
        .unwrap_or_else(|| "Pending".into());

    html! {
        <section class="pipeline panel">
            <div class="panel-header">
                <h2>{"TX → Channel → RX pipeline"}</h2>
                <span class="pipeline-tag">{preset.display_name()}</span>
            </div>
            <div class="pipeline-track">
                <div class="pipeline-stage">
                    <span class="stage-label">{"Input"}</span>
                    <h3>{"Plaintext"}</h3>
                    <ul>
                        <li>{format!("{} characters", plaintext_len)}</li>
                        <li class="muted">{preview}</li>
                    </ul>
                </div>
                <div class="stage-connector" aria-hidden="true"></div>
                <div class="pipeline-stage">
                    <span class="stage-label">{"TX"}</span>
                    <h3>{"Frame synthesis"}</h3>
                    <ul>
                        <li>{format!("QPSK symbol rate: {} sym/s", protocol.qpsk_symbol_rate)}</li>
                        <li>{format!("Bandwidth: {:.1} Hz", protocol.qpsk_bandwidth_hz)}</li>
                        <li>{format!("Frame layout: {} total symbols", protocol.frame_layout.total_symbols)}</li>
                        <li>{format!("LDPC (d_v={}, d_c={})", ldpc.dv, ldpc.dc)}</li>
                    </ul>
                </div>
                <div class="stage-connector" aria-hidden="true"></div>
                <div class="pipeline-stage">
                    <span class="stage-label">{"Channel"}</span>
                    <h3>{"AWGN + Raman mix"}</h3>
                    <ul>
                        <li>{format!("Configured SNR: {:.1} dB", props.input.snr_db)}</li>
                        <li>{format!("Sample rate: {} Hz", props.input.sample_rate)}</li>
                        <li>{format!("Carrier: {}", carrier_info)}</li>
                        <li class="muted">{format!("Preset baseline: {:.1} dB · {} Hz", simulation_defaults.snr_db, simulation_defaults.sample_rate)}</li>
                    </ul>
                </div>
                <div class="stage-connector" aria-hidden="true"></div>
                <div class="pipeline-stage">
                    <span class="stage-label">{"RX"}</span>
                    <h3>{"LDPC decode"}</h3>
                    <ul>
                        <li>{format!("Symbol decisions: {}", symbol_decisions)}</li>
                        <li>{format!("Pre-FEC BER: {}", pre_fec)}</li>
                        <li>{format!("Post-FEC BER: {}", post_fec)}</li>
                        <li class="muted">{recovered_preview}</li>
                    </ul>
                </div>
            </div>
        </section>
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

fn play_samples(
    ctx: &AudioContext,
    samples: &[f32],
    sample_rate: f32,
    gain_value: f32,
) -> Result<AudioBufferSourceNode, JsValue> {
    if samples.is_empty() {
        return Err(JsValue::from_str("No audio samples to play"));
    }

    let buffer = ctx.create_buffer(1, samples.len() as u32, sample_rate)?;
    let mut data = samples.to_vec();
    buffer.copy_to_channel(&mut data, 0)?;

    let source = ctx.create_buffer_source()?;
    source.set_buffer(Some(&buffer));

    let gain_node: GainNode = ctx.create_gain()?;
    gain_node.gain().set_value(gain_value);

    source.connect_with_audio_node(&gain_node)?;
    gain_node.connect_with_audio_node(&ctx.destination())?;
    source.start()?;
    Ok(source)
}

fn stop_source(source: &AudioBufferSourceNode) {
    #[allow(deprecated)]
    {
        let _ = source.stop();
    }
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

fn compute_magnitude_spectrum(samples: &[f32], sample_rate: f32) -> Vec<(f32, f32)> {
    const MAX_POINTS: usize = 4096;
    if samples.is_empty() {
        return Vec::new();
    }

    let len = samples.len().min(MAX_POINTS);
    let fft_len = len.next_power_of_two().max(64).min(MAX_POINTS);
    let mut buffer: Vec<Complex32> = samples
        .iter()
        .take(fft_len)
        .map(|&v| Complex32::new(v, 0.0))
        .collect();
    buffer.resize(fft_len, Complex32::new(0.0, 0.0));

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_len);
    fft.process(&mut buffer);

    let scale = 1.0 / fft_len as f32;
    let half = fft_len / 2;
    buffer
        .iter()
        .take(half)
        .enumerate()
        .map(|(i, c)| {
            let freq = sample_rate * i as f32 / fft_len as f32;
            let magnitude = (c.norm() * scale).max(1.0e-9);
            let db = 20.0 * magnitude.log10();
            (freq, db)
        })
        .collect()
}

fn draw_spectrum(canvas: &HtmlCanvasElement, spectrum: &[(f32, f32)]) {
    if spectrum.is_empty() {
        if let Some(backend) = CanvasBackend::with_canvas_object(canvas.clone()) {
            let area = backend.into_drawing_area();
            let _ = area.fill(&RGBColor(8, 10, 18));
        }
        return;
    }

    let backend = if let Some(backend) = CanvasBackend::with_canvas_object(canvas.clone()) {
        backend
    } else {
        console::error_1(&JsValue::from_str("Canvas backend unavailable"));
        return;
    };

    let drawing_area = backend.into_drawing_area();
    if drawing_area.fill(&RGBColor(8, 10, 18)).is_err() {
        return;
    }

    let max_freq = spectrum.last().map(|(f, _)| *f).unwrap_or(1.0).max(1.0);
    let (min_db, max_db) = spectrum
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), (_, db)| {
            (min.min(*db), max.max(*db))
        });
    let lower_db = min_db.floor().max(-120.0);
    let upper_db = max_db.ceil().min(6.0).max(lower_db + 6.0);

    let mut chart = match ChartBuilder::on(&drawing_area)
        .margin(12)
        .set_label_area_size(LabelAreaPosition::Left, 48)
        .set_label_area_size(LabelAreaPosition::Bottom, 48)
        .build_cartesian_2d(0f32..max_freq, lower_db..upper_db)
    {
        Ok(chart) => chart,
        Err(err) => {
            console::error_1(&JsValue::from_str(&format!("Chart build error: {err}")));
            return;
        }
    };

    let label_font = ("Inter", 14.0).into_font();

    chart
        .configure_mesh()
        .bold_line_style(&RGBAColor(60, 70, 100, 0.8))
        .light_line_style(&RGBAColor(46, 54, 80, 0.6))
        .axis_style(&RGBAColor(180, 192, 234, 0.6))
        .label_style(label_font.clone())
        .x_desc("Frequency (Hz)")
        .y_desc("Magnitude (dB)")
        .draw()
        .ok();

    let path: Vec<(f32, f32)> = spectrum.to_vec();
    chart
        .draw_series(std::iter::once(PathElement::new(
            path,
            &RGBColor(91, 200, 255),
        )))
        .ok();
}

#[derive(Properties, PartialEq)]
pub struct SpectrumPanelProps {
    pub spectrum: Vec<(f32, f32)>,
    pub sample_rate: f32,
}

#[function_component(SpectrumPanel)]
pub fn spectrum_panel(props: &SpectrumPanelProps) -> Html {
    let canvas_ref = use_node_ref();
    {
        let canvas_ref = canvas_ref.clone();
        let spectrum = props.spectrum.clone();
        use_effect_with(spectrum, move |data| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_spectrum(&canvas, data.as_slice());
            }
            || ()
        });
    }

    html! {
        <div class="spectrum-panel panel">
            <div class="panel-header">
                <h2>{"Frequency spectrum"}</h2>
                <span class="tag">{format!("Nyquist {:.0} Hz", props.sample_rate / 2.0)}</span>
            </div>
            <canvas ref={canvas_ref} width="512" height="240" />
            {
                if props.spectrum.is_empty() {
                    html! { <p class="muted">{"Blend at least one source to reveal the spectrum."}</p> }
                } else {
                    Html::default()
                }
            }
        </div>
    }
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
    <div class="stats-panel panel">
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
    <div class="constellation-panel panel">
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
    let include_carrier = use_state(|| true);
    let include_noise = use_state(|| true);
    let include_external = use_state(|| false);
    let external_mix = use_state(|| 0.5_f32);
    let master_level = use_state(|| 0.8_f32);
    let status = use_state(|| AttrValue::from("Run the pipeline to generate audio"));
    let active_source = use_state(|| None::<AudioBufferSourceNode>);

    {
        let status = status.clone();
        let external_audio = external_audio.clone();
        let active_source = active_source.clone();
        use_effect_with(props.audio.clone(), move |audio| {
            if audio.is_some() {
                status.set(AttrValue::from("Ready to play modulation audio"));
            } else {
                status.set(AttrValue::from("Run the pipeline to generate audio"));
                external_audio.set(None);
            }

            if let Some(source) = (*active_source).clone() {
                stop_source(&source);
                active_source.set(None);
            }

            || ()
        });
    }

    let audio_opt = props.audio.clone();
    if audio_opt.is_none() {
        return html! {
            <div class="audio-panel panel">
                <div class="panel-header">
                    <h2>{"Modulated Audio"}</h2>
                    <span class="tag">{"Pipeline idle"}</span>
                </div>
                <p class="muted">{"Run a simulation to generate the channel audio diagnostics."}</p>
            </div>
        };
    }

    let audio = audio_opt.unwrap();
    let mut mix = vec![0.0; audio.noisy.len()];
    let mut sources_active = 0usize;

    if *include_carrier {
        sources_active += 1;
        for (dst, &sample) in mix.iter_mut().zip(audio.clean.iter()) {
            *dst += sample;
        }
    }

    if *include_noise {
        sources_active += 1;
        for (dst, (&noisy, &clean)) in mix
            .iter_mut()
            .zip(audio.noisy.iter().zip(audio.clean.iter()))
        {
            *dst += noisy - clean;
        }
    }

    let external_loaded = (*external_audio).clone();
    if *include_external {
        if let Some(external) = external_loaded.as_ref() {
            sources_active += 1;
            let resampled = resample_external(
                &external.samples,
                external.sample_rate,
                audio.sample_rate as f32,
                audio.noisy.len(),
            );
            let vocoded = nonlinear_mix(&audio.noisy, &resampled, *external_mix);
            for (dst, sample) in mix.iter_mut().zip(vocoded.iter()) {
                *dst += *sample;
            }
        }
    }

    if sources_active > 0 {
        normalize_samples(&mut mix);
    }

    let mix_samples = Rc::new(mix);
    let spectrum = if sources_active > 0 {
        compute_magnitude_spectrum(mix_samples.as_ref(), audio.sample_rate as f32)
    } else {
        Vec::new()
    };
    let carrier_summary = format!(
        "Carrier {:.1} Hz · Sample rate {} Hz",
        audio.carrier_freq_hz, audio.sample_rate
    );
    let external_percent = format!("{:.0}%", *external_mix * 100.0);
    let master_percent = format!("{:.0}%", *master_level * 100.0);
    let playing = (*active_source).is_some();
    let can_play = sources_active > 0;
    let has_external_audio = external_loaded.is_some();
    let status_message = (*status).clone();

    let on_toggle_carrier = {
        let include_carrier = include_carrier.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                include_carrier.set(target.checked());
            }
        })
    };

    let on_toggle_noise = {
        let include_noise = include_noise.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                include_noise.set(target.checked());
            }
        })
    };

    let on_toggle_external = {
        let include_external = include_external.clone();
        let status = status.clone();
        let external_audio = external_audio.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let checked = target.checked();
                include_external.set(checked);
                if checked && (*external_audio).is_none() {
                    status.set(AttrValue::from("Upload a Raman feed to enable the blend"));
                }
            }
        })
    };

    let on_external_mix = {
        let external_mix = external_mix.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = target.value().parse::<f32>() {
                    external_mix.set(value.clamp(0.0, 1.0));
                }
            }
        })
    };

    let on_master_level = {
        let master_level = master_level.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = target.value().parse::<f32>() {
                    master_level.set(value.clamp(0.0, 1.0));
                }
            }
        })
    };

    let on_audio_upload = {
        let context = context.clone();
        let external_audio = external_audio.clone();
        let status = status.clone();
        let include_external = include_external.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Some(files) = target.files() {
                    if let Some(file) = files.get(0) {
                        let file_name = file.name();
                        let context = context.clone();
                        let external_audio = external_audio.clone();
                        let status = status.clone();
                        let include_external = include_external.clone();
                        spawn_local(async move {
                            status.set(AttrValue::from("Decoding uploaded Raman feed…"));
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
                                                                        include_external.set(true);
                                                                        status.set(format!(
                                                                            "Loaded Raman feed: {} ({:.1} kHz)",
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

    let play_mix = {
        let context = context.clone();
        let status = status.clone();
        let mix_samples = mix_samples.clone();
        let master_level = master_level.clone();
        let active_source = active_source.clone();
        let include_external = include_external.clone();
        let external_audio = external_audio.clone();
        let audio = audio.clone();
        Callback::from(move |_| {
            if mix_samples.iter().all(|v| v.abs() < 1.0e-5) {
                status.set(AttrValue::from(
                    "Enable at least one source before playback",
                ));
                return;
            }

            if *include_external && (*external_audio).is_none() {
                status.set(AttrValue::from("Upload a Raman feed to include in the mix"));
                return;
            }

            let mix_for_playback = mix_samples.clone();
            let context = context.clone();
            let status = status.clone();
            let active_source = active_source.clone();
            let gain = *master_level;
            let audio = audio.clone();

            spawn_local(async move {
                status.set(AttrValue::from("Starting playback…"));
                match ensure_audio_context(&context) {
                    Ok(ctx) => {
                        resume_context(&ctx).await;
                        if let Some(existing) = (*active_source).clone() {
                            stop_source(&existing);
                        }

                        match play_samples(
                            &ctx,
                            mix_for_playback.as_ref(),
                            audio.sample_rate as f32,
                            gain,
                        ) {
                            Ok(source) => {
                                active_source.set(Some(source));
                                status.set(AttrValue::from("Playback active"));
                            }
                            Err(err) => {
                                console::error_1(&err);

                                status.set(AttrValue::from("Unable to play audio"));
                            }
                        }
                    }
                    Err(err) => {
                        console::error_1(&err);

                        status.set(AttrValue::from("Audio context unavailable"));
                    }
                }
            });
        })
    };

    let stop_playback = {
        let active_source = active_source.clone();
        let status = status.clone();
        Callback::from(move |_| {
            if let Some(source) = (*active_source).clone() {
                stop_source(&source);
                status.set(AttrValue::from("Playback stopped"));
            } else {
                status.set(AttrValue::from("No playback active"));
            }
            active_source.set(None);
        })
    };

    html! {
        <div class="audio-panel panel">
            <div class="panel-header">
                <h2>{"Modulated Audio"}</h2>
                <span class="tag">{carrier_summary}</span>
            </div>
            <p class="muted">{"Blend the transmitter carrier, channel noise, and an optional Raman feed to hear Chimera's signal path."}</p>

            <div class="mixer-grid">
                <div class="toggle-group">
                    <label class="toggle">
                        <input type="checkbox" checked={*include_carrier} onchange={on_toggle_carrier} />
                        <span>
                            <strong>{"Carrier"}</strong>
                            <small>{"Clean TX waveform"}</small>
                        </span>
                    </label>
                    <label class="toggle">
                        <input type="checkbox" checked={*include_noise} onchange={on_toggle_noise} />
                        <span>
                            <strong>{"Channel noise"}</strong>
                            <small>{"AWGN impairment"}</small>
                        </span>
                    </label>
                    <label class="toggle">
                        <input type="checkbox" checked={*include_external && has_external_audio} onchange={on_toggle_external} disabled={!has_external_audio} />
                        <span>
                            <strong>{"Raman feed"}</strong>
                            <small>{"Uploaded audio"}</small>
                        </span>
                    </label>
                </div>

                <div class="fader-group">
                    <label>
                        <span>{"Raman blend"}</span>
                        <input
                            type="range"
                            min="0"
                            max="1"
                            step="0.05"
                            value={format!("{:.2}", *external_mix)}
                            oninput={on_external_mix}
                            disabled={!has_external_audio}
                        />
                        <span class="value">{external_percent.clone()}</span>
                    </label>
                    <label>
                        <span>{"Master level"}</span>
                        <input
                            type="range"
                            min="0"
                            max="1"
                            step="0.05"
                            value={format!("{:.2}", *master_level)}
                            oninput={on_master_level}
                        />
                        <span class="value">{master_percent.clone()}</span>
                    </label>
                    <div class="transport">
                        <button class="primary" type="button" onclick={play_mix.clone()} disabled={!can_play}>
                            { if playing { "Restart mix" } else { "Play mix" } }
                        </button>
                        <button class="ghost" type="button" onclick={stop_playback} disabled={!playing}>{"Stop"}</button>
                    </div>
                </div>
            </div>

            <label class="audio-upload">
                <span>{"Upload Raman audio (mp3/wav)"}</span>
                <input type="file" accept="audio/*" onchange={on_audio_upload} />
            </label>

            {
                if let Some(external) = external_loaded {
                    html! { <p class="muted">{format!("Loaded {} ({:.1} kHz)", external.name, external.sample_rate / 1000.0)}</p> }
                } else {
                    Html::default()
                }
            }

            <p class="audio-status">{status_message}</p>

            <SpectrumPanel spectrum={spectrum} sample_rate={audio.sample_rate as f32} />
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
    <div class="symbol-decisions-panel panel">
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
    <div class="logs-panel panel">
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
