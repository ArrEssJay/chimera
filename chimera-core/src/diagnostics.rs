use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SymbolDecision {
    pub index: usize,
    pub decided_bits: [u8; 2],
    pub average_i: f64,
    pub average_q: f64,
    pub min_distance: f64,
    pub distances: [f64; 4],
    pub soft_metrics: [f64; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FrameDescriptor {
    pub frame_index: usize,
    pub total_frames: usize,
    pub command_opcode: u32,
    pub command_value: u32,
    pub frame_label: String,
    pub payload_preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModulationAudio {
    pub sample_rate: usize,
    pub carrier_freq_hz: f64,
    pub clean: Vec<f32>,
    pub noisy: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DemodulationDiagnostics {
    pub received_symbols_i: Vec<f64>,
    pub received_symbols_q: Vec<f64>,
    pub symbol_decisions: Vec<SymbolDecision>,
    pub timing_error: Vec<f64>,
    pub nco_freq_offset: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SimulationReport {
    pub pre_fec_errors: usize,
    pub pre_fec_ber: f64,
    pub post_fec_errors: usize,
    pub post_fec_ber: f64,
    pub recovered_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DiagnosticsBundle {
    pub encoding_logs: Vec<String>,
    pub decoding_logs: Vec<String>,
    pub demodulation: DemodulationDiagnostics,
    pub modulation_audio: Option<ModulationAudio>,
    pub tx_bits: Vec<u8>,
    pub tx_symbols_i: Vec<f64>,
    pub tx_symbols_q: Vec<f64>,
    pub clean_baseband: Vec<f64>,
    pub noisy_baseband: Vec<f64>,
    pub frames: Vec<FrameDescriptor>,
}
