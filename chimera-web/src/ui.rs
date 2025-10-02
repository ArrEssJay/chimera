use crate::model::{run_pipeline, SimulationInput, SimulationOutput as PipelineOutput};
use crate::presets::FramePreset;
use plotters::prelude::*;
use plotters::style::colors::TRANSPARENT;
use plotters_canvas::CanvasBackend;
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

    let current_input = (*simulation).clone();
    let preset_bundle = current_input.preset.bundle();
    let frame_layout = preset_bundle.protocol.frame_layout;

    let on_preset_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: Event| {
            if let Some(select) = event.target_dyn_into::<web_sys::HtmlSelectElement>() {
                if let Some(preset) = FramePreset::from_key(&select.value()) {
                    let defaults = preset.simulation_config();
                    let mut next = (*simulation).clone();
                    next.preset = preset;
                    next.sample_rate = defaults.sample_rate;
                    next.plaintext = defaults.plaintext_source;
                    next.snr_db = defaults.snr_db;
                    simulation.set(next);
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

    let on_sample_rate_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<usize>() {
                    let mut next = (*simulation).clone();
                    next.sample_rate = value;
                    simulation.set(next);
                }
            }
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

    let (rx_i, rx_q, timing_error, freq_offset, encoding_logs, decoding_logs) =
        if let Some(out) = pipeline_output {
            (
                out.diagnostics.demodulation.received_symbols_i.clone(),
                out.diagnostics.demodulation.received_symbols_q.clone(),
                out.diagnostics.demodulation.timing_error.clone(),
                out.diagnostics.demodulation.nco_freq_offset.clone(),
                out.diagnostics.encoding_logs.clone(),
                out.diagnostics.decoding_logs.clone(),
            )
        } else {
            (vec![], vec![], vec![], vec![], vec![], vec![])
        };

    let report = pipeline_output.map(|out| out.report.clone());
    let modulation_audio = pipeline_output.and_then(|out| out.diagnostics.modulation_audio.clone());

    let recovered_message = report
        .as_ref()
        .map(|rep| rep.recovered_message.clone())
        .unwrap_or_default();
    let plaintext_len = current_input.plaintext.chars().count();

    html! {
        <main>
            <div class="main-grid">
                <section class="panel controls-panel">
                    <header class="panel-header">
                        <div>
                            <h1>{"Simulation Controls"}</h1>
                            <p class="muted">{"Configure presets and channel parameters, then run the full DSP pipeline."}</p>
                        </div>
                        <button class="primary" onclick={on_run.clone()} disabled={*is_running}>
                            { if *is_running { "Running…" } else { "Run Simulation" } }
                        </button>
                    </header>

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
                            </label>
                            <label class="field">
                                <span>{"Sample Rate (Hz)"}</span>
                                <input type="number" min="8000" step="1000" value={current_input.sample_rate.to_string()} oninput={on_sample_rate_change} />
                            </label>
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
                            html! {
                                <p class="muted">{"Run the simulation to populate telemetry."}</p>
                            }
                        }
                    }
                </section>

                <section class="pipeline-section">
                    <div class="node-graph">
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Input"}</h3>
                                <p>{format!("Plaintext: {} chars", plaintext_len)}</p>
                                <p>{format!("Sample rate: {} Hz", current_input.sample_rate)}</p>
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
                                <h3>{"Modulator"}</h3>
                                <ConstellationChart title="TX Symbols" i_samples={Vec::new()} q_samples={Vec::new()} />
                                <p class="muted">{"Transmitter diagnostics pending."}</p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Channel"}</h3>
                                <p>{format!("Carrier: {:.1} Hz", preset_bundle.protocol.carrier_freq_hz)}</p>
                                <p>{format!("QPSK rate: {} sym/s", preset_bundle.protocol.qpsk_symbol_rate)}</p>
                                <p>{format!("Frame max: {}", preset_bundle.protocol.max_frames)}</p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Demodulator"}</h3>
                                <ConstellationChart title="RX Symbols" i_samples={rx_i.clone()} q_samples={rx_q.clone()} />
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

                <section class="panel diagnostics-panel">
                    <header>
                        <h2>{"Diagnostics"}</h2>
                        <p class="muted">{"Analyzer outputs from the demodulation loop."}</p>
                    </header>
                    <div class="chart-grid">
                        <LineChart title="Timing Error" values={timing_error} />
                        <LineChart title="NCO Frequency Offset" values={freq_offset} />
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
        html! { <p class="muted">{"No log entries yet."}</p> }
    } else {
        let content = entries.join("\n");
        html! { <pre class="log-viewer">{content}</pre> }
    }
}

#[derive(Properties, PartialEq)]
pub struct ConstellationProps {
    pub title: AttrValue,
    pub i_samples: Vec<f64>,
    pub q_samples: Vec<f64>,
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

        use_effect_with(
            (i_samples.clone(), q_samples.clone()),
            move |(i_samples, q_samples)| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_constellation(&canvas, i_samples, q_samples, &title);
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

        use_effect_with(values.clone(), move |values| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_line_chart(&canvas, values, &title);
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

        let symbols = symbols_i
            .iter()
            .zip(symbols_q.iter())
            .map(|(&i, &q)| (i, q));

        chart.draw_series(symbols.map(|(i, q)| {
            let phase = (q.atan2(i) * 180.0 / std::f64::consts::PI) as f64;
            let color = HSLColor(phase / 360.0, 0.9, 0.6);
            Circle::new((i, q), 2, color.filled())
        }))?;
        Ok(())
    })();

    if let Err(e) = result {
        web_sys::console::error_1(&format!("Failed to draw constellation chart: {:?}", e).into());
    }
}

fn draw_line_chart(canvas: &HtmlCanvasElement, values: &[f64], title: &str) {
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

        let points: Vec<(f64, f64)> = values
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as f64, v))
            .collect();
        chart.draw_series(std::iter::once(PathElement::new(
            points,
            &RGBColor(94, 214, 255),
        )))?;

        Ok(())
    })();

    if let Err(e) = result {
        web_sys::console::error_1(&format!("Failed to draw line chart: {:?}", e).into());
    }
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
