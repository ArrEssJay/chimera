use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::diagnostics::{DiagnosticsBundle, SimulationReport};
use chimera_core::run_simulation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationInput {
    pub plaintext: String,
    pub snr_db: f64,
    pub sample_rate: usize,
}

impl Default for SimulationInput {
    fn default() -> Self {
        let sim = SimulationConfig::default();
        Self {
            plaintext: sim.plaintext_source,
            snr_db: sim.snr_db,
            sample_rate: sim.sample_rate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
}

pub fn run_pipeline(input: SimulationInput) -> SimulationOutput {
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = input.plaintext;
    sim.snr_db = input.snr_db;
    sim.sample_rate = input.sample_rate;

    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    let output = run_simulation(&sim, &protocol, &ldpc);
    SimulationOutput {
        report: output.report,
        diagnostics: output.diagnostics,
    }
}
