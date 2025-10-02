use wasm_bindgen::prelude::*;

pub mod model;
mod ui;

pub use model::{run_pipeline, SimulationInput, SimulationOutput};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let _ = wasm_logger::init(wasm_logger::Config::default());
    ui::mount_app();
    Ok(())
}
