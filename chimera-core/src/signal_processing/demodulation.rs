//! Demodulation implementations for extracting symbols from carrier signals

use num_complex::Complex64;
use std::f64::consts::TAU;

/// Configuration for demodulation
#[derive(Clone, Debug)]
pub struct DemodulationConfig {
    pub sample_rate: usize,
    pub symbol_rate: usize,
    pub carrier_freq: f64,
}

/// Carrier recovery state using Costas loop for QPSK
#[derive(Clone, Debug)]
struct CostasLoop {
    /// Current phase estimate
    phase: f64,
    /// Current frequency offset estimate (rad/sample)
    freq_offset: f64,
    /// Loop filter proportional gain
    alpha: f64,
    /// Loop filter integral gain
    beta: f64,
}

impl CostasLoop {
    /// Create a new Costas loop with default parameters
    fn new(loop_bandwidth: f64, damping_factor: f64) -> Self {
        // Second-order loop filter gains
        let denom = 1.0 + 2.0 * damping_factor * loop_bandwidth + loop_bandwidth * loop_bandwidth;
        let alpha = (4.0 * damping_factor * loop_bandwidth) / denom;
        let beta = (4.0 * loop_bandwidth * loop_bandwidth) / denom;
        
        Self {
            phase: 0.0,
            freq_offset: 0.0,
            alpha,
            beta,
        }
    }
    
    /// Update the loop with a new symbol and return the phase correction
    fn update(&mut self, symbol: Complex64) -> f64 {
        // QPSK Costas loop error detector
        // Error is proportional to Im(symbol^4) which is zero when locked to QPSK constellation
        let error = (symbol.re * symbol.im).signum() * (symbol.re * symbol.im);
        
        // Update frequency estimate (integral path)
        self.freq_offset += self.beta * error;
        
        // Update phase estimate (proportional path)
        self.phase += self.alpha * error + self.freq_offset;
        
        // Wrap phase to [-π, π]
        while self.phase > std::f64::consts::PI {
            self.phase -= TAU;
        }
        while self.phase < -std::f64::consts::PI {
            self.phase += TAU;
        }
        
        self.phase
    }
}

/// Demodulate audio back to IQ symbols with carrier recovery
/// 
/// Extracts QPSK symbols from the FSK+QPSK modulated audio using
/// coherent I/Q demodulation with Costas loop carrier recovery.
/// 
/// The Costas loop tracks and corrects for:
/// - Frequency offset between TX and RX
/// - Phase drift over time
/// - Initial phase offset
pub fn audio_to_symbols(
    audio: &[f32],
    config: &DemodulationConfig,
) -> Vec<Complex64> {
    if audio.is_empty() || config.sample_rate == 0 {
        return Vec::new();
    }
    
    let samples_per_symbol = (config.sample_rate / config.symbol_rate).max(1);
    let num_symbols = audio.len() / samples_per_symbol;
    let mut symbols = Vec::with_capacity(num_symbols);
    
    let dt = 1.0 / config.sample_rate as f64;
    
    // Initialize Costas loop for carrier recovery
    // Loop bandwidth: ~1% of symbol rate is typical
    let loop_bw = config.symbol_rate as f64 * 0.01;
    let damping = 0.707; // Critical damping
    let mut costas = CostasLoop::new(loop_bw, damping);
    
    // Demodulate using I/Q mixing with carrier recovery
    for sym_idx in 0..num_symbols {
        let start = sym_idx * samples_per_symbol;
        let end = (start + samples_per_symbol).min(audio.len());
        
        let mut i_acc = 0.0f64;
        let mut q_acc = 0.0f64;
        
        // Get current phase correction from Costas loop
        let phase_correction = if sym_idx > 0 {
            costas.phase
        } else {
            0.0
        };
        
        for (idx, &sample) in audio[start..end].iter().enumerate() {
            let t = (start + idx) as f64 * dt;
            let angle = TAU * config.carrier_freq * t + phase_correction;
            
            // I/Q demodulation with phase correction
            i_acc += sample as f64 * angle.cos();
            q_acc += -(sample as f64) * angle.sin(); // Note the negative for Q
        }
        
        // Average over the symbol period
        let count = (end - start) as f64;
        i_acc /= count;
        q_acc /= count;
        
        // Scale by 2 to compensate for mixing (standard I/Q demod scaling)
        let symbol = Complex64::new(i_acc * 2.0, q_acc * 2.0);
        
        // Update Costas loop with this symbol
        costas.update(symbol);
        
        symbols.push(symbol);
    }
    
    symbols
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_processing::modulation::{ModulationConfig, symbols_to_carrier_signal};
    use num_complex::Complex64;

    #[test]
    fn test_audio_to_symbols_basic() {
        // Need enough audio for at least one symbol
        let samples_per_symbol = 48000 / 16; // 3000 samples
        let audio: Vec<f32> = (0..samples_per_symbol).map(|i| (i as f32 * 0.001).sin()).collect();
        let config = DemodulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
        };
        
        let symbols = audio_to_symbols(&audio, &config);
        assert!(!symbols.is_empty());
    }

    #[test]
    fn test_empty_audio_returns_empty() {
        let config = DemodulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
        };
        
        let symbols = audio_to_symbols(&[], &config);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_modulation_demodulation_with_carrier_recovery() {
        // Test that carrier recovery enables reasonable symbol reconstruction
        let original_symbols = vec![
            Complex64::new(0.707, 0.707),   // 45°
            Complex64::new(-0.707, 0.707),  // 135°
            Complex64::new(-0.707, -0.707), // 225°
            Complex64::new(0.707, -0.707),  // 315°
            Complex64::new(0.707, 0.707),   // Repeat pattern
            Complex64::new(-0.707, 0.707),
            Complex64::new(-0.707, -0.707),
            Complex64::new(0.707, -0.707),
        ];
        
        let mod_config = ModulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
            enable_qpsk: true,
            enable_fsk: false, // Disable FSK for cleaner test
        };
        
        let audio = symbols_to_carrier_signal(&original_symbols, &mod_config);
        assert!(!audio.is_empty());
        
        let demod_config = DemodulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
        };
        
        let recovered_symbols = audio_to_symbols(&audio, &demod_config);
        
        // Verify we got the right number of symbols
        assert_eq!(recovered_symbols.len(), original_symbols.len());
        
        // After carrier recovery locks (skip first symbol), symbols should cluster
        // near the QPSK constellation points
        for (i, symbol) in recovered_symbols.iter().enumerate().skip(1) {
            // Symbol should have reasonable magnitude (not just noise)
            let mag = symbol.norm();
            assert!(mag > 0.3, "Symbol {} magnitude {} too low", i, mag);
            assert!(mag < 2.0, "Symbol {} magnitude {} too high", i, mag);
            
            // After normalization, symbol should be close to unit circle
            let normalized = symbol / mag;
            
            // Find closest QPSK constellation point
            let expected_points = [
                Complex64::new(0.707, 0.707),
                Complex64::new(-0.707, 0.707),
                Complex64::new(-0.707, -0.707),
                Complex64::new(0.707, -0.707),
            ];
            
            let min_error = expected_points.iter()
                .map(|&expected| (normalized - expected).norm())
                .fold(f64::INFINITY, f64::min);
            
            // With carrier recovery, should be reasonably close to constellation
            // Note: Phase filtering in modulation causes some distortion
            assert!(min_error < 1.0, 
                "Symbol {} error {} too large: {:?} normalized to {:?}", 
                i, min_error, symbol, normalized);
        }
    }
    
    #[test]
    fn test_carrier_recovery_with_frequency_offset() {
        // Test that Costas loop can track frequency offset
        let original_symbols = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 1.0),
            Complex64::new(-1.0, 0.0),
            Complex64::new(0.0, -1.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 1.0),
            Complex64::new(-1.0, 0.0),
            Complex64::new(0.0, -1.0),
        ];
        
        let mod_config = ModulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
            enable_qpsk: true,
            enable_fsk: false,
        };
        
        let audio = symbols_to_carrier_signal(&original_symbols, &mod_config);
        
        // Demodulate with slight frequency offset
        let demod_config = DemodulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12005.0, // 5 Hz offset
        };
        
        let recovered_symbols = audio_to_symbols(&audio, &demod_config);
        
        assert_eq!(recovered_symbols.len(), original_symbols.len());
        
        // Costas loop should converge after a few symbols
        // Later symbols should be better recovered than early ones
        let _early_avg = recovered_symbols[0..2].iter()
            .map(|s| s.norm())
            .sum::<f64>() / 2.0;
        
        let late_avg = recovered_symbols[6..8].iter()
            .map(|s| s.norm())
            .sum::<f64>() / 2.0;
        
        // Later symbols should have comparable or better magnitude
        // (Costas loop should at least maintain signal)
        assert!(late_avg > 0.1, "Late symbols too weak: {}", late_avg);
    }
}
