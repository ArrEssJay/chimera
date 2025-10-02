use crate::model::{run_pipeline, SimulationInput, SimulationOutput as PipelineOutput};
use crate::presets::FramePreset;
use plotters::prelude::*;
use plotters::style::colors::TRANSPARENT;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, HtmlCanvasElement, HtmlElement, HtmlInputElement, HtmlSelectElement,
    HtmlTextAreaElement};
use yew::prelude::*;
use yew::TargetCast;


#[function_component(App)]
fn app(_props: &AppProps) -> Html {
    let sim_config = use_state(SimulationConfig::default);
    let protocol_config = use_state(ProtocolConfig::default);
    let ldpc_config = use_state(LDPCConfig::default);
    let sim_output = use_state(|| None);
    let is_running = use_state(|| false);

    let on_run = {
        let sim_config = sim_config.clone();
        let protocol_config = protocol_config.clone();
        let ldpc_config = ldpc_config.clone();
        let sim_output = sim_output.clone();
        let is_running = is_running.clone();
        Callback::from(move |_mouse_event: MouseEvent| {
            is_running.set(true);
            let sim_config = (*sim_config).clone();
            let protocol_config = (*protocol_config).clone();
            let ldpc_config = (*ldpc_config).clone();
            let sim_output = sim_output.clone();
            let is_running = is_running.clone();
            spawn_local(async move {
                let output =
                    chimera_core::run_simulation(&sim_config, &protocol_config, &ldpc_config);
                sim_output.set(Some(output));
                is_running.set(false);
            });
        })
    };

    let (tx_i, tx_q, rx_i, rx_q) = if let Some(output) = &*sim_output {
        // TODO: Transmitted symbols are not currently part of the diagnostics bundle.
        // We should add them to see the "clean" constellation. For now, it will be empty.
        let tx_i: Vec<f64> = vec![];
        let tx_q: Vec<f64> = vec![];

        let rx_i = output.diagnostics.demodulation.received_symbols_i.clone();
        let rx_q = output.diagnostics.demodulation.received_symbols_q.clone();

        (tx_i, tx_q, rx_i, rx_q)
    } else {
        (vec![], vec![], vec![], vec![])
    };

    html! {
        <main>
            <div class="main-grid">
                <div class="controls">
                    <button onclick={on_run} disabled={*is_running}>
                        { if *is_running { "Running..." } else { "Run" } }
                    </button>
                </div>
                <div class="node-graph">
                    <div class="node-column">
                        <div class="node">
                            <h3>{"Input"}</h3>
                            <p>{"Plaintext, parameters"}</p>
                        </div>
                    </div>
                    <div class="node-column">
                        <div class="node">
                            <h3>{"Encoder"}</h3>
                            <p>{"FEC, framing"}</p>
                        </div>
                        <div class="node">
                            <h3>{"Modulator"}</h3>
                            <ConstellationChart title="TX Symbols" i_samples={tx_i} q_samples={tx_q} />
                        </div>
                    </div>
                    <div class="node-column">
                        <div class="node">
                            <h3>{"Channel"}</h3>
                            <p>{"AWGN, impairments"}</p>
                        </div>
                    </div>
                    <div class="node-column">
                        <div class="node">
                            <h3>{"Demodulator"}</h3>
                            <ConstellationChart title="RX Symbols" i_samples={rx_i} q_samples={rx_q} />
                        </div>
                        <div class="node">
                            <h3>{"Decoder"}</h3>
                            <p>{"FEC, de-framing"}</p>
                        </div>
                    </div>
                    <div class="node-column">
                        <div class="node">
                            <h3>{"Output"}</h3>
                            <p>{"Recovered text, stats"}</p>
                        </div>
                    </div>
                </div>
            </div>
        </main>
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
            <canvas ref={canvas_ref} width="240" height="240" />
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

    // Let CSS handle the background color
    root.fill(&TRANSPARENT).unwrap_or_else(|e| {
        web_sys::console::error_1(&format!("Failed to fill chart background: {:?}", e).into());
    });

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut chart = ChartBuilder::on(&root)
            .caption(
                title,
                ("Inter", 16, &RGBColor(200, 200, 200)), // Use a neutral text color
            )
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
            // Use a consistent color from the theme palette via a simple hue rotation
            let color = HSLColor(phase / 360.0, 0.9, 0.7);
            Circle::new((i, q), 2, color.filled())
        }))?;
        Ok(())
    })();

    if let Err(e) = result {
        web_sys::console::error_1(&format!("Failed to draw constellation chart: {:?}", e).into());
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
