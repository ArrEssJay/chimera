//! Wrapper around existing demodulator
//!
//! This is a thin adapter - it calls the existing, proven demodulator code
//! without modifying it.

use crate::signal_processing::demodulation::{
    DemodulationConfig, DemodulationResult, audio_to_symbols_with_snr
};

/// Wrapper around existing demodulator (DO NOT modify the actual demodulator)
pub struct DemodulatorWrapper {
    config: DemodulationConfig,
}

impl DemodulatorWrapper {
    pub fn new(
        sample_rate: usize,
        symbol_rate: usize,
        carrier_freq: f64,
    ) -> Self {
        Self {
            config: DemodulationConfig {
                sample_rate,
                symbol_rate,
                carrier_freq,
            },
        }
    }
    
    /// Convert audio signal to symbols
    /// 
    /// Calls the existing, proven demodulator code
    pub fn demodulate(&self, audio: &[f32]) -> DemodulationResult {
        // Use the existing demodulation function - DO NOT reimplement
        audio_to_symbols_with_snr(audio, &self.config)
    }
}
