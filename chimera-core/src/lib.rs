//! Chimera core library
//!
//! This crate provides a Rust-native implementation of the Raman Whisper
//! modulation and decoding pipeline. The modules are organized to mirror
//! the former Python reference implementation, but expose a fully typed
//! and testable API.

pub mod config;
pub mod decoder;
pub mod diagnostics;
pub mod encoder;
pub mod ldpc;
pub mod utils;

use config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use decoder::demodulate_and_decode;
use diagnostics::{DiagnosticsBundle, SimulationReport};
use encoder::generate_modulated_signal;
use ldpc::LDPCSuite;

/// High-level handle returned by `run_simulation`.
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
pub fn run_simulation(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    let encoding = generate_modulated_signal(sim, protocol, &ldpc_suite.matrices);
    let demodulation = demodulate_and_decode(&encoding, &ldpc_suite.matrices, sim, protocol);

    let diagnostics = DiagnosticsBundle {
        encoding_logs: encoding.logs.clone(),
        decoding_logs: demodulation.logs.clone(),
        demodulation: demodulation.diagnostics.clone(),
    };

    SimulationOutput {
        report: demodulation.report.clone(),
        diagnostics,
        ldpc: ldpc_suite,
    }
}
