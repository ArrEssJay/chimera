//! Test Fixtures and Common Test Data
//!
//! Provides standard test configurations and symbol patterns

use chimera_core::signal_processing::{
    modulation::ModulationConfig,
    demodulation::DemodulationConfig,
};
use num_complex::Complex64;

/// Symbol patterns for testing
#[allow(dead_code)]
pub enum SymbolPattern {
    /// All zeros (1+0j) - for carrier testing
    AllZeros,
    /// Alternating between two phases
    Alternating,
    /// Random QPSK symbols
    Random,
    /// All four QPSK constellation points in sequence
    AllFourPhases,
    /// Specific repeating pattern
    Custom(Vec<Complex64>),
}

/// Generate test symbols according to pattern
pub fn generate_test_symbols(pattern: SymbolPattern, count: usize) -> Vec<Complex64> {
    match pattern {
        SymbolPattern::AllZeros => {
            vec![Complex64::new(1.0, 0.0); count]
        },
        SymbolPattern::Alternating => {
            (0..count)
                .map(|i| {
                    if i % 2 == 0 {
                        Complex64::new(1.0, 0.0)
                    } else {
                        Complex64::new(-1.0, 0.0)
                    }
                })
                .collect()
        },
        SymbolPattern::Random => {
            use rand::{Rng, SeedableRng};
            let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
            
            (0..count)
                .map(|_| {
                    let phase = rng.gen_range(0..4) as f64 * std::f64::consts::PI / 2.0;
                    Complex64::new(phase.cos(), phase.sin())
                })
                .collect()
        },
        SymbolPattern::AllFourPhases => {
            let phases = [
                Complex64::new(1.0, 0.0),   // 0°
                Complex64::new(0.0, 1.0),   // 90°
                Complex64::new(-1.0, 0.0),  // 180°
                Complex64::new(0.0, -1.0),  // 270°
            ];
            
            (0..count)
                .map(|i| phases[i % 4])
                .collect()
        },
        SymbolPattern::Custom(pattern) => {
            (0..count)
                .map(|i| pattern[i % pattern.len()])
                .collect()
        },
    }
}

/// Get standard test configuration for specific test type
pub fn get_test_modulation_config(enable_qpsk: bool, enable_fsk: bool) -> ModulationConfig {
    ModulationConfig {
        sample_rate: 48000,
        carrier_freq: 12000.0,
        symbol_rate: 16,
        enable_qpsk,
        enable_fsk,
    }
}

/// Get standard demodulation config
pub fn get_test_demodulation_config() -> DemodulationConfig {
    DemodulationConfig {
        sample_rate: 48000,
        carrier_freq: 12000.0,
        symbol_rate: 16,
    }
}
