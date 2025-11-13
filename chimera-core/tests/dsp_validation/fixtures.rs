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
            
            // QPSK constellation at ±45°, ±135° 
            let qpsk_points = [
                Complex64::new(std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),   // 45°
                Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),  // 135°
                Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2), // 225°
                Complex64::new(std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),  // 315°
            ];
            
            (0..count)
                .map(|_| qpsk_points[rng.gen_range(0..4)])
                .collect()
        },
        SymbolPattern::AllFourPhases => {
            // Standard QPSK constellation at ±45°, ±135°
            let phases = [
                Complex64::new(std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),   // 45° [0,1]
                Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),  // 135° [0,0]
                Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2), // 225° [1,1]
                Complex64::new(std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),  // 315° [1,0]
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
pub fn get_test_modulation_config(_enable_qpsk: bool, _enable_fsk: bool) -> ModulationConfig {
    // Note: FSK and QPSK are always enabled per spec, parameters kept for API compatibility
    ModulationConfig {
        sample_rate: TEST_SAMPLE_RATE,
        symbol_rate: TEST_SYMBOL_RATE,
        carrier_freq: TEST_CARRIER_FREQ,
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

/// Generate a realistic frame with sync preamble + payload
/// This uses the proven architecture from mod/demod unit tests via shared implementation
pub fn generate_realistic_frame(payload_symbols: usize, seed: u64) -> Vec<Complex64> {
    chimera_core::signal_processing::demodulation::generate_test_frame(payload_symbols, seed)
}

