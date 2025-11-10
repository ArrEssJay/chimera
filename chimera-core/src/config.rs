//! Configuration types for the Chimera pipeline.
use serde::{Deserialize, Serialize};
use crate::errors::{ConfigError, Result};

// Default value functions for serde
fn default_true() -> bool { true }



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolConfig {
    pub carrier_freq_hz: f64,
    pub qpsk_symbol_rate: usize,
    
    /// Debug: Enable/disable QPSK modulation (if false, outputs pure carrier)
    #[serde(default = "default_true")]
    pub enable_qpsk: bool,
    
    /// Debug: Enable/disable FSK frequency dithering (if false, uses constant carrier freq)
    #[serde(default = "default_true")]
    pub enable_fsk: bool,
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
    
    /// Validate protocol configuration
    pub fn validate(&self) -> Result<()> {
        // Check sample rate validity
        if self.qpsk_symbol_rate == 0 {
            return Err(ConfigError::InvalidSymbolRate { 
                rate: self.qpsk_symbol_rate 
            }.into());
        }
        
        // Check Nyquist criterion for carrier frequency
        // Need at least 2x carrier frequency as sample rate
        let min_sample_rate = self.carrier_freq_hz * 2.0;
        let actual_sample_rate = SimulationConfig::SAMPLE_RATE as f64;
        if actual_sample_rate < min_sample_rate {
            return Err(ConfigError::NyquistViolation {
                carrier_hz: self.carrier_freq_hz,
                min_required_hz: min_sample_rate,
                actual_hz: actual_sample_rate,
            }.into());
        }
        
        // Validate FSK frequencies
        if !self.fsk_freq_zero_hz.is_finite() || !self.fsk_freq_one_hz.is_finite() {
            return Err(ConfigError::InvalidFskFrequencies {
                f0: self.fsk_freq_zero_hz,
                f1: self.fsk_freq_one_hz,
            }.into());
        }
        
        // Validate frame layout
        self.frame_layout.validate()?;
        
        Ok(())
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            carrier_freq_hz: 12_000.0,
            qpsk_symbol_rate: 16,
            enable_qpsk: true,
            enable_fsk: true,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationConfig {
    pub snr_db: f64,
    pub link_loss_db: f64,
    pub plaintext_source: String,
    pub rng_seed: Option<u64>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            snr_db: -3.0,
            link_loss_db: 0.0,
            plaintext_source: "This is a longer message demonstrating the protocol-compliant, rate-4/5 LDPC error correction.".into(),
            rng_seed: None,
        }
    }
}

impl SimulationConfig {
    /// Audio sample rate is fixed at 48 kHz
    pub const SAMPLE_RATE: usize = 48_000;
    
    /// Validate simulation configuration
    pub fn validate(&self) -> Result<()> {
        if !self.snr_db.is_finite() {
            return Err(ConfigError::InvalidSnr { snr_db: self.snr_db }.into());
        }
        if !self.link_loss_db.is_finite() || self.link_loss_db < 0.0 {
            return Err(ConfigError::InvalidSnr { snr_db: self.link_loss_db }.into());
        }
        Ok(())
    }
}

impl FrameLayout {
    /// Validate frame layout consistency
    pub fn validate(&self) -> Result<()> {
        let computed_total = self.sync_symbols 
            + self.target_id_symbols 
            + self.command_type_symbols 
            + self.data_payload_symbols 
            + self.ecc_symbols;
        
        if computed_total != self.total_symbols {
            return Err(ConfigError::InvalidFrameLayout {
                reason: format!(
                    "Symbol sum mismatch: {} + {} + {} + {} + {} = {}, expected {}",
                    self.sync_symbols,
                    self.target_id_symbols,
                    self.command_type_symbols,
                    self.data_payload_symbols,
                    self.ecc_symbols,
                    computed_total,
                    self.total_symbols
                )
            }.into());
        }
        
        if self.total_symbols == 0 {
            return Err(ConfigError::InvalidFrameLayout {
                reason: "total_symbols cannot be zero".to_string()
            }.into());
        }
        
        Ok(())
    }
}

