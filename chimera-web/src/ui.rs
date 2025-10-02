use crate::model::{run_pipeline, SimulationInput, SimulationOutput};
use chimera_core::diagnostics::DiagnosticsBundle;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::{Document, HtmlCanvasElement, HtmlInputElement, HtmlTextAreaElement};
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

    let current_input = (*input).clone();
    let current_output = (*output).clone();

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

    html! {
        <div class="stats-panel">
            <h2>{"Simulation Results"}</h2>
            <ul>
                <li>{format!("SNR configured: {:.1} dB", props.input.snr_db)}</li>
                <li>{format!("Sample rate: {} Hz", props.input.sample_rate)}</li>
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

            let symbols = symbols_i.iter().zip(symbols_q.iter()).map(|(&i, &q)| (i, q));

            let _ = chart.draw_series(symbols.map(|(i, q)| Circle::new((i, q), 3, RED.filled())));
        }
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
