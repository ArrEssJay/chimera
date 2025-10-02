use crate::model::{run_pipeline, SimulationInput, SimulationOutput};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
yew::function_component;
yew::html;
yew::use_state;
yew::AttrValue;
yew::Callback;
yew::Html;
yew::Properties;

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(SimulationInput::default);
    let output = use_state(|| run_pipeline((*input).clone()));

    let on_submit = {
        let input = input.clone();
        let output = output.clone();
        Callback::from(move |_e: yew::events::SubmitEvent| {
            output.set(run_pipeline((*input).clone()));
        })
    };

    let on_plaintext_change = {
        let input = input.clone();
        Callback::from(move |e: yew::events::InputEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                let mut updated = (*input).clone();
                updated.plaintext = target.value();
                input.set(updated);
            }
        })
    };

    let current_output = (*output).clone();

    yew::html! {
        <div class="app-container">
            <h1>{"Chimera Visualizer"}</h1>
            <form onsubmit={on_submit}>
                <label for="plaintext">{"Plaintext"}</label>
                <textarea id="plaintext" value={AttrValue::from(input.plaintext.clone())}
                    oninput={on_plaintext_change} />
                <button type="submit">{"Run Simulation"}</button>
            </form>
            <div class="dashboard">
                <StatsPanel output={current_output.clone()} />
                <ConstellationChart diagnostics={current_output.diagnostics.clone()} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatsPanelProps {
    pub output: SimulationOutput,
}

#[function_component(StatsPanel)]
pub fn stats_panel(props: &StatsPanelProps) -> Html {
    let report = &props.output.report;
    yew::html! {
        <div class="stats-panel">
            <h2>{"Simulation Results"}</h2>
            <ul>
                <li>{format!("Pre-FEC errors: {} (BER {:.6})", report.pre_fec_errors, report.pre_fec_ber)}</li>
                <li>{format!("Post-FEC errors: {} (BER {:.6})", report.post_fec_errors, report.post_fec_ber)}</li>
                <li>{format!("Recovered message: {}", report.recovered_message)}</li>
            </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ConstellationProps {
    pub diagnostics: crate::chimera_core::diagnostics::DiagnosticsBundle,
}

#[function_component(ConstellationChart)]
pub fn constellation_chart(props: &ConstellationProps) -> Html {
    let canvas_ref = yew::use_node_ref();
    {
        let diagnostics = props.diagnostics.clone();
        let canvas_ref = canvas_ref.clone();
        yew::use_effect_with_deps(
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_constellation(&canvas, &diagnostics);
                }
                || ()
            },
            (),
        );
    }

    yew::html! {
        <div class="constellation-panel">
            <h2>{"Constellation"}</h2>
            <canvas ref={canvas_ref} width="320" height="320" />
        </div>
    }
}

fn draw_constellation(canvas: &HtmlCanvasElement, diagnostics: &crate::chimera_core::diagnostics::DiagnosticsBundle) {
    if let Some(mut backend) = CanvasBackend::with_canvas_object(canvas.clone()) {
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let area = root.margin(10, 10, 10, 10);
        let mut chart = ChartBuilder::on(&area)
            .caption("Received Constellation", ("sans-serif", 20))
            .build_cartesian_2d(-1.5..1.5, -1.5..1.5)
            .unwrap();

        chart.configure_mesh().x_labels(5).y_labels(5).draw().unwrap();

        let symbols = diagnostics.demodulation.received_symbols_i
            .iter()
            .zip(&diagnostics.demodulation.received_symbols_q)
            .map(|(&i, &q)| (i, q));

        chart.draw_series(symbols.map(|(i, q)| Circle::new((i, q), 3, RED.filled()))).unwrap();
    }
}

pub fn mount_app() {
    yew::Renderer::<App>::new().render();
}
