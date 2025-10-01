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
use diagnostics::{DiagnosticsBundle, SimulationReport};
use ldpc::LDPCSuite;

/// High-level handle returned by `run_simulation`.
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
pub fn run_simulation(
    _sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);

    // TODO: call encoder / decoder pipeline once implemented.
    SimulationOutput {
        report: SimulationReport::default(),
        diagnostics: DiagnosticsBundle::default(),
        ldpc: ldpc_suite,
    }
}
