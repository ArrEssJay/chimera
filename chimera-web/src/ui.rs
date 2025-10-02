use crate::model::{
    run_pipeline, SimulationInput, SimulationOutput as PipelineOutput, FIXED_SAMPLE_RATE,
};
use crate::presets::FramePreset;
use chimera_core::diagnostics::{FrameDescriptor, SymbolDecision};
use gloo_file::callbacks::{read_as_data_url, FileReader};
use gloo_file::Blob;
use gloo_timers::callback::Timeout;
use plotters::prelude::*;
use plotters::style::colors::TRANSPARENT;
use plotters::style::RGBAColor;
use plotters_canvas::CanvasBackend;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FftPlanner;
use std::f64::consts::FRAC_1_SQRT_2;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, Event, HtmlCanvasElement, HtmlElement};
use yew::events::InputEvent;
use yew::prelude::*;
use yew::TargetCast;

#[function_component(App)]
pub fn app() -> Html {
    let simulation = use_state(SimulationInput::default);
    let output = use_state(|| None::<PipelineOutput>);
    let is_running = use_state(|| false);
    let external_audio_name = use_state(|| None::<String>);
    let reader_handle = use_mut_ref(|| None::<FileReader>);

    let current_input = (*simulation).clone();
    let preset_bundle = current_input.preset.bundle();
    let frame_layout = preset_bundle.protocol.frame_layout;

    {
        let output_state = output.clone();
        let running_state = is_running.clone();
        use_effect_with((*simulation).clone(), move |input| {
            let maybe_timeout = if *running_state {
                None
            } else {
                let snapshot = input.clone();
                let output_handle = output_state.clone();
                let running_handle = running_state.clone();
                Some(Timeout::new(300, move || {
                    if *running_handle {
                        return;
                    }

                    running_handle.set(true);
                    let output_handle = output_handle.clone();
                    let running_handle_inner = running_handle.clone();
                    let sim_to_run = snapshot.clone();

                    spawn_local(async move {
                        let result = run_pipeline(sim_to_run);
                        output_handle.set(Some(result));
                        running_handle_inner.set(false);
                    });
                }))
            };

            move || {
                if let Some(timeout) = maybe_timeout {
                    timeout.cancel();
                }
            }
        });
    }

    let on_preset_change = {
        let simulation = simulation.clone();
        let external_audio_name = external_audio_name.clone();
        Callback::from(move |event: Event| {
            if let Some(select) = event.target_dyn_into::<web_sys::HtmlSelectElement>() {
                if let Some(preset) = FramePreset::from_key(&select.value()) {
                    let defaults = preset.simulation_config();
                    let mut next = (*simulation).clone();
                    next.preset = preset;
                    next.plaintext = defaults.plaintext_source;
                    next.snr_db = defaults.snr_db;
                    simulation.set(next);
                    external_audio_name.set(None);
                }
            }
        })
    };

    let on_plaintext_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(textarea) = event.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                let mut next = (*simulation).clone();
                next.plaintext = textarea.value();
                simulation.set(next);
            }
        })
    };

    let on_snr_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    let mut next = (*simulation).clone();
                    next.snr_db = value;
                    simulation.set(next);
                }
            }
        })
    };

    let on_audio_upload = {
        let simulation = simulation.clone();
        let reader_handle = reader_handle.clone();
        let external_audio_name = external_audio_name.clone();
        Callback::from(move |event: Event| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let file_name = file.name();
                        if file.size() > 512_000.0 {
                            web_sys::console::warn_1(
                                &format!(
                                    "Audio file '{}' is larger than 512 KB; skipping upload.",
                                    file_name
                                )
                                .into(),
                            );
                        } else {
                            let blob = Blob::from(file);
                            let simulation_handle = simulation.clone();
                            let audio_name_handle = external_audio_name.clone();
                            let name_clone = file_name.clone();
                            let handle = read_as_data_url(&blob, move |result| {
                                if let Ok(data_url) = result {
                                    if let Some(encoded) = data_url.split(',').nth(1) {
                                        let mut next = (*simulation_handle).clone();
                                        let payload_name = name_clone.clone();
                                        let payload =
                                            format!("AUDIO:{}:{}", payload_name.as_str(), encoded);
                                        next.plaintext = payload;
                                        simulation_handle.set(next);
                                        audio_name_handle.set(Some(payload_name));
                                    }
                                }
                            });
                            *reader_handle.borrow_mut() = Some(handle);
                        }
                    }
                }
                input.set_value("");
            }
        })
    };

    let on_audio_clear = {
        let simulation = simulation.clone();
        let external_audio_name = external_audio_name.clone();
        Callback::from(move |_event: MouseEvent| {
            let mut next = (*simulation).clone();
            let defaults = next.preset.simulation_config();
            next.plaintext = defaults.plaintext_source;
            simulation.set(next);
            external_audio_name.set(None);
        })
    };

    let on_run = {
        let simulation_handle = simulation.clone();
        let output_handle = output.clone();
        let running_handle = is_running.clone();
        Callback::from(move |_event: MouseEvent| {
            if *running_handle {
                return;
            }
            running_handle.set(true);
            let input = (*simulation_handle).clone();
            let output_state = output_handle.clone();
            let running_state = running_handle.clone();
            spawn_local(async move {
                let result = run_pipeline(input);
                output_state.set(Some(result));
                running_state.set(false);
            });
        })
    };

    let pipeline_output = (*output).as_ref();

    let (
        tx_i,
        tx_q,
        rx_i,
        rx_q,
        clean_baseband,
        noisy_baseband,
        tx_bits,
        symbol_decisions,
        timing_error,
        freq_offset,
        encoding_logs,
        decoding_logs,
        frames,
    ) = if let Some(out) = pipeline_output {
        let diag = &out.diagnostics;
        (
            diag.tx_symbols_i.clone(),
            diag.tx_symbols_q.clone(),
            diag.demodulation.received_symbols_i.clone(),
            diag.demodulation.received_symbols_q.clone(),
            diag.clean_baseband.clone(),
            diag.noisy_baseband.clone(),
            diag.tx_bits.clone(),
            diag.demodulation.symbol_decisions.clone(),
            diag.demodulation.timing_error.clone(),
            diag.demodulation.nco_freq_offset.clone(),
            diag.encoding_logs.clone(),
            diag.decoding_logs.clone(),
            diag.frames.clone(),
        )
    } else {
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::<SymbolDecision>::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::<FrameDescriptor>::new(),
        )
    };

    let psd_clean = compute_psd(&clean_baseband, FIXED_SAMPLE_RATE);
    let psd_noisy = compute_psd(&noisy_baseband, FIXED_SAMPLE_RATE);
    let ber_trend = compute_ber_trend(&tx_bits, &symbol_decisions);

    let report = pipeline_output.map(|out| out.report.clone());
    let modulation_audio = pipeline_output.and_then(|out| out.diagnostics.modulation_audio.clone());

    let recovered_message = report
        .as_ref()
        .map(|rep| rep.recovered_message.clone())
        .unwrap_or_default();
    let plaintext_len = current_input.plaintext.chars().count();
    let audio_file_name = (*external_audio_name).clone();

    html! {
        <main>
            <div class="main-grid">
                <section class="panel controls-panel">
                    <header class="panel-header">
                        <div>
                            <h1>{"Simulation Controls"}</h1>
                            <p class="muted">{"Configure presets and channel parameters; the dashboard re-runs automatically after edits."}</p>
                        </div>
                        <div class="run-controls">
                            {
                                if *is_running {
                                    html! { <span class="badge badge-live">{"Updating…"}</span> }
                                } else {
                                    html! { <span class="badge badge-live idle">{"Live preview"}</span> }
                                }
                            }
                            <button class="primary" onclick={on_run.clone()} disabled={*is_running}>
                                { if *is_running { "Running…" } else { "Run Now" } }
                            </button>
                        </div>
                    </header>

                    <p class="control-hint">{"Changes trigger an auto-preview after 300 ms. Use \"Run Now\" to force an immediate rebuild."}</p>

                    <div class="control-grid">
                        <label class="field">
                            <span>{"Preset"}</span>
                            <select value={current_input.preset.key()} onchange={on_preset_change}>
                                { for FramePreset::all().iter().map(|preset| {
                                    let key = preset.key();
                                    let name = preset.display_name();
                                    html! { <option value={key} selected={current_input.preset == *preset}>{name}</option> }
                                }) }
                            </select>
                            <p class="muted">{current_input.preset.description()}</p>
                        </label>

                        <label class="field">
                            <span>{"Plaintext"}</span>
                            <textarea value={current_input.plaintext.clone()} oninput={on_plaintext_change} />
                            <p class="muted">{format!("{} chars", plaintext_len)}</p>
                        </label>

                        <div class="field-inline">
                            <label class="field">
                                <span>{"Eb/N₀ (dB)"}</span>
                                    <input type="range" min="-10" max="20" step="0.5" value={current_input.snr_db.to_string()} oninput={on_snr_change.clone()} />
                                <input type="number" value={format!("{:.2}", current_input.snr_db)} oninput={on_snr_change} />
                                <p class="muted small">{"Eb/N₀ expresses per-bit energy against noise density. Higher values generally reduce pre-FEC BER under AWGN."}</p>
                            </label>
                            <div class="field locked">
                                <span>{"Sample Rate"}</span>
                                <div class="locked-field">
                                    <span class="value">{format!("{} Hz", FIXED_SAMPLE_RATE)}</span>
                                    <span class="tag compact">{"Locked"}</span>
                                </div>
                                <p class="muted small">{"All presets render at 48 kHz for deterministic diagnostics."}</p>
                            </div>
                        </div>

                        <div class="field">
                            <span>{"External Audio Payload"}</span>
                            <input type="file" accept="audio/*" onchange={on_audio_upload} />
                            <div class="audio-actions">
                                {
                                    if let Some(name) = audio_file_name {
                                        html! {
                                            <>
                                                <span class="muted small">{format!("Attached: {} (embedded as base64)", name)}</span>
                                                <button class="ghost" type="button" onclick={on_audio_clear.clone()}>{"Remove audio"}</button>
                                            </>
                                        }
                                    } else {
                                        html! { <span class="muted small">{"Attach a small WAV/MP3 to embed directly into the framed payload."}</span> }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </section>

                <section class="panel telemetry-panel">
                    <header>
                        <h2>{"Frame Telemetry"}</h2>
                        <p class="muted">{"Live metrics from the most recent pipeline execution."}</p>
                    </header>
                    {
                        if let Some(ref report) = report {
                            html! {
                                <div class="metrics-grid">
                                    <div class="metric">
                                        <span class="label">{"Pre-FEC BER"}</span>
                                        <span class="value">{format_sci(report.pre_fec_ber)}</span>
                                        <span class="detail">{format!("{} symbol errors", report.pre_fec_errors)}</span>
                                    </div>
                                    <div class="metric">
                                        <span class="label">{"Post-FEC BER"}</span>
                                        <span class="value">{format_sci(report.post_fec_ber)}</span>
                                        <span class="detail">{format!("{} residual errors", report.post_fec_errors)}</span>
                                    </div>
                                    <div class="metric">
                                        <span class="label">{"Recovered Message"}</span>
                                        <span class="value value-long">{&report.recovered_message}</span>
                                    </div>
                                    if let Some(ref audio) = modulation_audio {
                                        <div class="metric">
                                            <span class="label">{"Modulation Audio"}</span>
                                            <span class="value">{format!("{} Hz", audio.sample_rate)}</span>
                                            <span class="detail">{format!("Carrier {:.1} Hz", audio.carrier_freq_hz)}</span>
                                        </div>
                                    }
                                </div>
                            }
                        } else {
                            html! { <p class="muted">{"Auto-preview will populate telemetry after the first run."}</p> }
                        }
                    }
                </section>

                <section class="pipeline-section">
                    <div class="node-graph">
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Input"}</h3>
                                <p>{format!("Plaintext: {} chars", plaintext_len)}</p>
                                <p>{format!("Sample rate: {} Hz", FIXED_SAMPLE_RATE)}</p>
                                <p>{format!("Eb/N₀: {:.1} dB", current_input.snr_db)}</p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Encoder"}</h3>
                                <p>{format!("Total symbols: {}", frame_layout.total_symbols)}</p>
                                <p>{format!("Payload symbols: {}", frame_layout.data_payload_symbols)}</p>
                                <p>{format!("ECC symbols: {}", frame_layout.ecc_symbols)}</p>
                            </div>
                            <div class="node">
                                <h3>{"Transmitter"}</h3>
                                <ConstellationChart title="TX Symbols" i_samples={tx_i.clone()} q_samples={tx_q.clone()} variant={ConstellationVariant::Tx} />
                                <p class="muted small">{"Ideal QPSK constellation produced by the framing encoder."}</p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Channel"}</h3>
                                <p>{format!("Carrier: {:.1} Hz", preset_bundle.protocol.carrier_freq_hz)}</p>
                                <p>{format!("QPSK rate: {} sym/s", preset_bundle.protocol.qpsk_symbol_rate)}</p>
                                <p>{format!("Frame ceiling: {}", preset_bundle.protocol.max_frames)}</p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Receiver"}</h3>
                                <ConstellationChart title="RX Symbols" i_samples={rx_i.clone()} q_samples={rx_q.clone()} variant={ConstellationVariant::Rx} />
                                <p class="muted small">{"Recovered constellation after timing/frequency correction."}</p>
                            </div>
                            <div class="node">
                                <h3>{"Decoder"}</h3>
                                {
                                    if let Some(ref report) = report {
                                        html! {
                                            <>
                                                <p>{format!("Residual errors: {}", report.post_fec_errors)}</p>
                                                <p>{format!("Post-FEC BER: {}", format_sci(report.post_fec_ber))}</p>
                                            </>
                                        }
                                    } else {
                                        html! { <p class="muted">{"Awaiting run."}</p> }
                                    }
                                }
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Output"}</h3>
                                {
                                    if !recovered_message.is_empty() {
                                        html! { <p>{recovered_message.clone()}</p> }
                                    } else {
                                        html! { <p class="muted">{"Recovered text will appear here."}</p> }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </section>

                <section class="panel frame-panel">
                    <header>
                        <h2>{"Frame Inspector"}</h2>
                        <p class="muted">{"Decoded command flags with payload previews."}</p>
                    </header>
                    {
                        if frames.is_empty() {
                            html! { <p class="muted">{"Run the simulation to inspect frame descriptors."}</p> }
                        } else {
                            html! {
                                <div class="frame-table-wrapper">
                                    <table class="frame-table">
                                        <thead>
                                            <tr>
                                                <th>{"Index"}</th>
                                                <th>{"Label"}</th>
                                                <th>{"Opcode"}</th>
                                                <th>{"Command Word"}</th>
                                                <th>{"Payload Preview"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            { for frames.iter().map(|desc| {
                                                html! {
                                                    <tr>
                                                        <td>{format!("{}/{}", desc.frame_index + 1, desc.total_frames)}</td>
                                                        <td><span class="tag">{desc.frame_label.clone()}</span></td>
                                                        <td>{format_opcode_label(desc.command_opcode)}</td>
                                                        <td>{format_command_word_label(desc)}</td>
                                                        <td class="payload-cell">{desc.payload_preview.clone()}</td>
                                                    </tr>
                                                }
                                            }) }
                                        </tbody>
                                    </table>
                                </div>
                            }
                        }
                    }
                </section>

                <section class="panel diagnostics-panel">
                    <header>
                        <h2>{"Diagnostics"}</h2>
                        <p class="muted">{"Analyzer outputs from the demodulation loop."}</p>
                    </header>
                    <div class="chart-grid">
                        <LineChart title="Timing Error" values={timing_error.clone()} accent_rgb={Some((94, 214, 255))} />
                        <LineChart title="NCO Frequency Offset" values={freq_offset.clone()} accent_rgb={Some((255, 168, 112))} />
                        <LineChart title="Clean Signal PSD (dBFS)" values={psd_clean.clone()} accent_rgb={Some((126, 240, 180))} />
                        <LineChart title="Noisy Signal PSD (dBFS)" values={psd_noisy.clone()} accent_rgb={Some((255, 132, 220))} />
                        <LineChart title="Running BER" values={ber_trend.clone()} accent_rgb={Some((255, 238, 96))} />
                    </div>
                    <div class="log-columns">
                        <div class="log-pane">
                            <h3>{"Encoder Log"}</h3>
                            { render_log(&encoding_logs) }
                        </div>
                        <div class="log-pane">
                            <h3>{"Decoder Log"}</h3>
                            { render_log(&decoding_logs) }
                        </div>
                    </div>
                </section>
            </div>
        </main>
    }
}

fn render_log(entries: &[String]) -> Html {
    if entries.is_empty() {
        html! { <p class="muted">{"Diagnostics will appear after the auto-preview finishes."}</p> }
    } else {
        let content = entries.join("\n");
        html! { <pre class="log-viewer">{content}</pre> }
    }
}

#[derive(Clone, PartialEq)]
pub enum ConstellationVariant {
    Tx,
    Rx,
}

#[derive(Properties, PartialEq)]
pub struct ConstellationProps {
    pub title: AttrValue,
    pub i_samples: Vec<f64>,
    pub q_samples: Vec<f64>,
    #[prop_or(ConstellationVariant::Rx)]
    pub variant: ConstellationVariant,
}

#[function_component(ConstellationChart)]
pub fn constellation_chart(props: &ConstellationProps) -> Html {
    if props.i_samples.is_empty() || props.q_samples.is_empty() {
        return html! {
            <div class="constellation-panel panel">
                <div class="chart-empty">{"No constellation samples."}</div>
            </div>
        };
    }

    let canvas_ref = use_node_ref();
    {
        let canvas_ref = canvas_ref.clone();
        let i_samples = props.i_samples.clone();
        let q_samples = props.q_samples.clone();
        let title = props.title.clone();
        let variant = props.variant.clone();

        use_effect_with(
            (i_samples.clone(), q_samples.clone(), variant.clone()),
            move |(i_samples, q_samples, variant)| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_constellation(&canvas, i_samples, q_samples, &title, variant.clone());
                }
                || ()
            },
        );
    }

    html! {
        <div class="constellation-panel panel">
            <canvas ref={canvas_ref} width="260" height="260" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LineChartProps {
    pub title: AttrValue,
    pub values: Vec<f64>,
    #[prop_or(None)]
    pub accent_rgb: Option<(u8, u8, u8)>,
}

#[function_component(LineChart)]
fn line_chart(props: &LineChartProps) -> Html {
    if props.values.is_empty() {
        return html! {
            <div class="chart-panel panel">
                <div class="chart-empty">{"No samples available."}</div>
            </div>
        };
    }

    let canvas_ref = use_node_ref();
    {
        let canvas_ref = canvas_ref.clone();
        let values = props.values.clone();
        let title = props.title.clone();
        let accent = props.accent_rgb;

        use_effect_with((values.clone(), accent), move |(values, accent)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_line_chart(&canvas, values, &title, *accent);
            }
            || ()
        });
    }

    html! {
        <div class="chart-panel panel">
            <canvas ref={canvas_ref} width="320" height="220" />
        </div>
    }
}

fn draw_constellation(
    canvas: &HtmlCanvasElement,
    symbols_i: &[f64],
    symbols_q: &[f64],
    title: &str,
    variant: ConstellationVariant,
) {
    let backend = if let Some(backend) = CanvasBackend::with_canvas_object(canvas.clone()) {
        backend
    } else {
        web_sys::console::error_1(&"Failed to create canvas backend".into());
        return;
    };
    let root = backend.into_drawing_area();

    root.fill(&TRANSPARENT).unwrap_or_else(|e| {
        web_sys::console::error_1(&format!("Failed to fill chart background: {:?}", e).into());
    });

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("Inter", 16, &RGBColor(200, 200, 200)))
            .margin(5)
            .build_cartesian_2d(-1.5..1.5, -1.5..1.5)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .disable_axes()
            .draw()?;

        let (point_color, halo_color, radius) = match variant {
            ConstellationVariant::Tx => {
                (RGBColor(126, 240, 196), RGBAColor(126, 240, 196, 0.25), 6)
            }
            ConstellationVariant::Rx => {
                (RGBColor(255, 168, 250), RGBAColor(255, 168, 250, 0.25), 3)
            }
        };

        if matches!(variant, ConstellationVariant::Tx) {
            let reference = [
                (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            ];
            chart.draw_series(
                reference
                    .iter()
                    .map(|&(i, q)| Circle::new((i, q), radius + 2, halo_color.filled())),
            )?;
        }

        let symbols = symbols_i
            .iter()
            .zip(symbols_q.iter())
            .map(|(&i, &q)| (i, q));

        chart
            .draw_series(symbols.map(|(i, q)| Circle::new((i, q), radius, point_color.filled())))?;

        Ok(())
    })();

    if let Err(e) = result {
        web_sys::console::error_1(&format!("Failed to draw constellation chart: {:?}", e).into());
    }
}

fn draw_line_chart(
    canvas: &HtmlCanvasElement,
    values: &[f64],
    title: &str,
    accent: Option<(u8, u8, u8)>,
) {
    let backend = if let Some(backend) = CanvasBackend::with_canvas_object(canvas.clone()) {
        backend
    } else {
        web_sys::console::error_1(&"Failed to create canvas backend".into());
        return;
    };

    let root = backend.into_drawing_area();
    root.fill(&TRANSPARENT).unwrap_or_else(|e| {
        web_sys::console::error_1(&format!("Failed to fill chart background: {:?}", e).into());
    });

    if values.is_empty() {
        return;
    }

    let y_min = values
        .iter()
        .cloned()
        .fold(f64::INFINITY, |acc, v| acc.min(v));
    let y_max = values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |acc, v| acc.max(v));

    let (y_lower, y_upper) = if (y_max - y_min).abs() < f64::EPSILON {
        (y_min - 1.0, y_max + 1.0)
    } else {
        (y_min, y_max)
    };

    let len = values.len();
    let x_upper = if len > 1 { (len - 1) as f64 } else { 1.0 };

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("Inter", 16, &RGBColor(200, 200, 200)))
            .margin(5)
            .build_cartesian_2d(0f64..x_upper, y_lower..y_upper)?;

        chart
            .configure_mesh()
            .bold_line_style(&RGBColor(40, 60, 90).mix(0.4))
            .light_line_style(&RGBColor(40, 60, 90).mix(0.2))
            .x_labels(5)
            .y_labels(5)
            .draw()?;

        let line_color = accent
            .map(|(r, g, b)| RGBColor(r, g, b))
            .unwrap_or_else(|| RGBColor(94, 214, 255));

        let points: Vec<(f64, f64)> = values
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as f64, v))
            .collect();
        let line_style = ShapeStyle::from(&line_color).stroke_width(2);
        chart.draw_series(std::iter::once(PathElement::new(points, line_style)))?;

        Ok(())
    })();

    if let Err(e) = result {
        web_sys::console::error_1(&format!("Failed to draw line chart: {:?}", e).into());
    }
}

fn decimate_series(series: &[f64], max_points: usize) -> Vec<f64> {
    if max_points == 0 || series.len() <= max_points {
        return series.to_vec();
    }

    let step = series.len() as f64 / max_points as f64;
    let mut result = Vec::with_capacity(max_points);
    let mut cursor = 0.0f64;

    while result.len() < max_points {
        let start = cursor.floor() as usize;
        if start >= series.len() {
            break;
        }
        let mut end = ((cursor + step).ceil() as usize).min(series.len());
        if end <= start {
            end = start + 1;
        }
        let slice = &series[start..end.min(series.len())];
        let avg = slice.iter().sum::<f64>() / slice.len() as f64;
        result.push(avg);
        cursor += step;
    }

    if result.is_empty() {
        series.to_vec()
    } else {
        result
    }
}

fn compute_psd(samples: &[f64], _sample_rate: usize) -> Vec<f64> {
    let sample_count = samples.len().min(16_384);
    if sample_count == 0 {
        return Vec::new();
    }

    let mut fft_len = sample_count.next_power_of_two();
    if fft_len < 1_024 {
        fft_len = 1_024;
    }
    if fft_len > 32_768 {
        fft_len = 32_768;
    }

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(fft_len);

    let mut buffer: Vec<Complex<f64>> = (0..fft_len)
        .map(|i| {
            if i < sample_count {
                Complex::new(samples[i], 0.0)
            } else {
                Complex::zero()
            }
        })
        .collect();

    fft.process(&mut buffer);

    let scale = 1.0 / fft_len as f64;
    let half = buffer.len() / 2;
    let mut spectrum: Vec<f64> = buffer
        .iter()
        .take(half)
        .map(|c| {
            let power = (c.norm_sqr() * scale).max(1e-12);
            10.0 * power.log10()
        })
        .collect();

    let max_db = spectrum.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if max_db.is_finite() {
        for v in &mut spectrum {
            *v -= max_db;
        }
    }

    decimate_series(&spectrum, 512)
}

fn compute_ber_trend(tx_bits: &[u8], decisions: &[SymbolDecision]) -> Vec<f64> {
    let symbol_count = usize::min(decisions.len(), tx_bits.len() / 2);
    if symbol_count == 0 {
        return Vec::new();
    }

    let mut errors = 0usize;
    let mut trend = Vec::with_capacity(symbol_count);

    for idx in 0..symbol_count {
        let tx0 = tx_bits[idx * 2];
        let tx1 = tx_bits[idx * 2 + 1];
        let decided = decisions[idx].decided_bits;
        if decided[0] != tx0 {
            errors += 1;
        }
        if decided[1] != tx1 {
            errors += 1;
        }
        let total_bits = (idx + 1) * 2;
        trend.push(errors as f64 / total_bits as f64);
    }

    decimate_series(&trend, 512)
}

fn format_opcode_label(opcode: u32) -> String {
    let label = match opcode {
        0x0001 => "Baseline Telemetry",
        0x00F1 => "Burst Telemetry Control",
        0x0D11 => "Deep-Space Probe Command",
        _ => "Custom Opcode",
    };

    format!("{label} · 0x{opcode:04X}")
}

fn format_command_word_label(desc: &FrameDescriptor) -> String {
    let frame_number = desc.frame_index + 1;
    let total_frames = desc.total_frames;

    let sequence = if total_frames > 0 {
        format!("Frame {frame_number} of {total_frames}")
    } else {
        format!("Frame {frame_number}")
    };

    format!("{sequence} · 0x{:08X}", desc.command_value)
}

fn format_sci(value: f64) -> String {
    if value == 0.0 {
        "0".into()
    } else {
        format!("{:.3e}", value)
    }
}

pub fn mount_app() {
    let window = web_sys::window().expect("window available");
    let document: Document = window.document().expect("document available");
    let root = document
        .get_element_by_id("chimera-root")
        .or_else(|| {
            let body = document.body().expect("document should have a body");
            let element = document.create_element("div").unwrap();
            element.set_id("chimera-root");
            body.append_child(&element).unwrap();
            Some(element)
        })
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    yew::Renderer::<App>::with_root(root.into()).render();
}
