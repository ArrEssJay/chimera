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
            plaintext: defaults.plaintext_source,
            snr_db: defaults.snr_db,
            link_loss_db: defaults.link_loss_db,
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
    sim.plaintext_source = input.plaintext;
    sim.snr_db = input.snr_db;
    sim.link_loss_db = input.link_loss_db;
    sim.sample_rate = FIXED_SAMPLE_RATE;

    let output = run_simulation(&sim, &protocol, &ldpc);
    SimulationOutput {
        report: output.report,
        diagnostics: output.diagnostics,
    }
}
