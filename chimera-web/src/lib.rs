use wasm_bindgen::prelude::*;

pub mod model;
pub mod presets;
pub mod streaming_wasm;

pub use model::{run_pipeline, SimulationInput, SimulationOutput};
pub use presets::{FramePreset, PresetBundle};
pub use streaming_wasm::{WASMStreamingDSP, WASMStreamOutput};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    // UI is now handled by React - no need to mount the old Rust UI
    Ok(())
}
