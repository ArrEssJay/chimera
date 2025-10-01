//! Diagnostic data structures for reporting.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DemodulationDiagnostics {
    pub received_symbols_i: Vec<f64>,
    pub received_symbols_q: Vec<f64>,
    pub timing_error: Vec<f64>,
    pub nco_freq_offset: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimulationReport {
    pub pre_fec_errors: usize,
    pub pre_fec_ber: f64,
    pub post_fec_errors: usize,
    pub post_fec_ber: f64,
    pub recovered_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagnosticsBundle {
    pub encoding_logs: Vec<String>,
    pub decoding_logs: Vec<String>,
    pub demodulation: DemodulationDiagnostics,
}
