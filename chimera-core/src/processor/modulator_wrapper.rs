//! Wrapper around existing modulator
//!
//! This is a thin adapter - it calls the existing, proven modulator code
//! without modifying it.

use num_complex::Complex64;
use crate::signal_processing::modulation::{ModulationConfig, symbols_to_carrier_signal};

/// Wrapper around existing modulator (DO NOT modify the actual modulator)
pub struct ModulatorWrapper {
    config: ModulationConfig,
}

impl ModulatorWrapper {
    pub fn new(
        sample_rate: usize,
        symbol_rate: usize,
        carrier_freq: f64,
        enable_qpsk: bool,
        enable_fsk: bool,
    ) -> Self {
        Self {
            config: ModulationConfig {
                sample_rate,
                symbol_rate,
                carrier_freq,
                enable_qpsk,
                enable_fsk,
            },
        }
    }
    
    /// Convert symbols to modulated audio signal
    /// 
    /// Calls the existing, proven modulator code
    pub fn modulate(&self, symbols: &[Complex64]) -> Vec<f32> {
        // Use the existing modulation function - DO NOT reimplement
        symbols_to_carrier_signal(symbols, &self.config)
    }
}
