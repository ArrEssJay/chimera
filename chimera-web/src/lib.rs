use wasm_bindgen::prelude::*;

pub mod model;
pub mod presets;
mod ui;

pub use model::{run_pipeline, SimulationInput, SimulationOutput};
pub use presets::{FramePreset, PresetBundle};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    ui::mount_app();
    Ok(())
}
