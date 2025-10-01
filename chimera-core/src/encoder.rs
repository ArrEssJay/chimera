//! Encoding and modulation stage.
use ndarray::Array1;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::ldpc::LDPCMatrices;
use crate::utils::LogCollector;

pub struct EncodingResult {
    pub noisy_signal: Array1<f64>,
    pub clean_signal: Array1<f64>,
    pub qpsk_bitstream: Vec<u8>,
    pub payload_bits: Vec<u8>,
    pub total_frames: usize,
    pub logs: Vec<String>,
}

impl EncodingResult {
    pub fn new() -> Self {
        Self {
            noisy_signal: Array1::from_vec(vec![]),
            clean_signal: Array1::from_vec(vec![]),
            qpsk_bitstream: Vec::new(),
            payload_bits: Vec::new(),
            total_frames: 0,
            logs: Vec::new(),
        }
    }
}

impl Default for EncodingResult {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_modulated_signal(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    matrices: &LDPCMatrices,
) -> EncodingResult {
    let mut logger = LogCollector::new();
    logger.log("generate_modulated_signal: unimplemented");
    let _ = (sim, protocol, matrices);
    EncodingResult {
        logs: logger.entries().to_vec(),
        ..EncodingResult::default()
    }
}
