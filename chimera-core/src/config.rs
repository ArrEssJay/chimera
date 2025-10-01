//! Configuration types for the Chimera pipeline.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BitDepth {
    Pcm16,
    Pcm24,
    Pcm32,
    #[default]
    Float32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameLayout {
    pub total_symbols: usize,
    pub sync_symbols: usize,
    pub target_id_symbols: usize,
    pub command_type_symbols: usize,
    pub data_payload_symbols: usize,
    pub ecc_symbols: usize,
}

impl Default for FrameLayout {
    fn default() -> Self {
        Self {
            total_symbols: 128,
            sync_symbols: 16,
            target_id_symbols: 16,
            command_type_symbols: 16,
            data_payload_symbols: 64,
            ecc_symbols: 16,
        }
    }
}

impl FrameLayout {
    pub fn message_bits(&self) -> usize {
        self.data_payload_symbols * 2
    }

    pub fn ecc_bits(&self) -> usize {
        self.ecc_symbols * 2
    }

    pub fn codeword_bits(&self) -> usize {
        self.message_bits() + self.ecc_bits()
    }

    pub fn frame_bits(&self) -> usize {
        self.total_symbols * 2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub carrier_freq_hz: f64,
    pub qpsk_symbol_rate: usize,
    pub qpsk_bandwidth_hz: f64,
    pub fsk_bit_rate: f64,
    pub fsk_freq_zero_hz: f64,
    pub fsk_freq_one_hz: f64,
    pub command_opcode: u32,
    pub frame_layout: FrameLayout,
    pub sync_sequence_hex: String,
    pub target_id_hex: String,
    pub max_frames: usize,
    pub current_frame_shift: usize,
    pub total_frames_shift: usize,
}

impl ProtocolConfig {
    pub fn fsk_freq_deviation_hz(&self) -> f64 {
        self.fsk_freq_one_hz - self.carrier_freq_hz
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            carrier_freq_hz: 12_000.0,
            qpsk_symbol_rate: 16,
            qpsk_bandwidth_hz: 20.0,
            fsk_bit_rate: 1.0,
            fsk_freq_zero_hz: 11_999.0,
            fsk_freq_one_hz: 12_001.0,
            command_opcode: 0x0001,
            frame_layout: FrameLayout::default(),
            sync_sequence_hex: "A5A5A5A5".to_string(),
            target_id_hex: "DEADBEEF".to_string(),
            max_frames: 256,
            current_frame_shift: 16,
            total_frames_shift: 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LDPCConfig {
    pub dv: usize,
    pub dc: usize,
    pub seed: Option<u64>,
}

impl Default for LDPCConfig {
    fn default() -> Self {
        Self {
            dv: 2,
            dc: 10,
            seed: Some(42),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub sample_rate: usize,
    pub bit_depth: BitDepth,
    pub snr_db: f64,
    pub plaintext_source: String,
    pub rng_seed: Option<u64>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            bit_depth: BitDepth::default(),
            snr_db: 3.0,
            plaintext_source: "This is a longer message demonstrating the protocol-compliant, rate-4/5 LDPC error correction.".into(),
            rng_seed: None,
        }
    }
}
