use crate::model::{run_pipeline, SimulationInput, SimulationOutput};
use crate::presets::FramePreset;
use chimera_core::diagnostics::{DiagnosticsBundle, SymbolDecision};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::{
    Document, HtmlCanvasElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement,
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
                <SymbolDecisionPanel decisions={current_output.diagnostics.demodulation.symbol_decisions.clone()} />
                <LogsPanel title={AttrValue::from("Encoder Logs")} entries={current_output.diagnostics.encoding_logs.clone()} />
                <LogsPanel title={AttrValue::from("Decoder Logs")} entries={current_output.diagnostics.decoding_logs.clone()} />
            </div>
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
