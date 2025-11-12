use crate::presets::{FramePreset, PresetBundle};
use chimera_core::diagnostics::{DiagnosticsBundle, SimulationReport};
use chimera_core::run_simulation;
use serde::{Deserialize, Serialize};

pub const FIXED_SAMPLE_RATE: usize = 48_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationInput {
    pub plaintext: String,
    pub snr_db: f64,
    pub link_loss_db: f64,
    pub preset: FramePreset,
}

impl Default for SimulationInput {
    fn default() -> Self {
        Self::with_preset(FramePreset::RamanWhisper)
    }
}

impl SimulationInput {
    pub fn with_preset(preset: FramePreset) -> Self {
        let defaults = preset.simulation_config();
        Self {
            plaintext: defaults.message,
            snr_db: 20.0, // Default SNR since it's no longer in simulation config
            link_loss_db: 0.0, // Default link loss
            preset,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
}

pub fn run_pipeline(input: SimulationInput) -> SimulationOutput {
    let PresetBundle {
        protocol,
        simulation,
        ldpc,
    } = input.preset.bundle();

    let mut sim = simulation;
    sim.message = input.plaintext;
    // Note: SNR and link_loss are now runtime params, passed separately to pipeline
    // For now, using the simulation directly which will use defaults

    let output = run_simulation(&sim, &protocol, &ldpc);
    SimulationOutput {
        report: output.report,
        diagnostics: output.diagnostics,
    }
}
