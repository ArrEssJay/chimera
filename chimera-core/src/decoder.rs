//! Demodulation and decoding stage implementations.
use ndarray::Array1;
use num_complex::Complex64;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::diagnostics::{DemodulationDiagnostics, SimulationReport};
use crate::encoder::EncodingResult;
use crate::ldpc::LDPCMatrices;

pub struct DemodulationResult {
    pub demodulated_bitstream: Vec<u8>,
    pub decoded_bitstream: Vec<u8>,
    pub recovered_message: String,
    pub diagnostics: DemodulationDiagnostics,
    pub report: SimulationReport,
}

impl DemodulationResult {
    pub fn empty() -> Self {
        DemodulationResult {
            demodulated_bitstream: Vec::new(),
            decoded_bitstream: Vec::new(),
            recovered_message: String::new(),
            diagnostics: DemodulationDiagnostics::default(),
            report: SimulationReport::default(),
        }
    }
}

pub fn demodulate_and_decode(
    _encoding: &EncodingResult,
    _matrices: &LDPCMatrices,
    _sim: &SimulationConfig,
    _protocol: &ProtocolConfig,
) -> DemodulationResult {
    // TODO(TBD): implement Costas / Gardner loops, LDPC decode integration, etc.
    DemodulationResult::empty()
}

pub fn baseband_to_symbols(_signal: &Array1<f64>) -> Vec<Complex64> {
    unimplemented!("baseband_to_symbols not yet implemented")
}
