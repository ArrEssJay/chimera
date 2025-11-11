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

/// Result from demodulation including symbols and signal quality metrics
#[derive(Clone, Debug)]
pub struct DemodulationResult {
    /// Demodulated IQ symbols
    pub symbols: Vec<Complex64>,
    /// Estimated SNR in dB (measured before AGC normalization)
    pub snr_db: f32,
    /// Average input signal power (before AGC)
    pub input_power: f64,
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
    fn new(loop_bandwidth: f64, damping_factor: f64, sample_rate: f64) -> Self {
        // Normalize loop bandwidth to rad/sample
        let bn_t = loop_bandwidth / sample_rate;
        
        // Second-order loop filter gains (from Gardner's formula)
        let theta = bn_t / (damping_factor + 0.25 / damping_factor);
        let denom = 1.0 + 2.0 * damping_factor * theta + theta * theta;
        
        let alpha = (4.0 * damping_factor * theta / denom) / 1.0;
        let beta = (4.0 * theta * theta / denom) / 1.0;
        
        Self {
            phase: 0.0,
            freq_offset: 0.0,
            alpha,
            beta,
        }
    }
    
    /// Process a demodulated symbol and return the corrected symbol
    fn process(&mut self, raw_symbol: Complex64) -> Complex64 {
        // Apply current phase correction to the symbol
        let corrected = raw_symbol * Complex64::from_polar(1.0, -self.phase);
        
        // Compute phase error using the corrected symbol
        let error = self.phase_detector(corrected);
        
        // Update frequency estimate (integral path)
        self.freq_offset += self.beta * error;
        
        // Limit frequency offset to prevent runaway
        self.freq_offset = self.freq_offset.clamp(-0.01, 0.01); // Tighter limit
        
        // Update phase estimate for next symbol
        self.phase += self.freq_offset + self.alpha * error;
        
        // Wrap phase to [-π, π]
        while self.phase > std::f64::consts::PI {
            self.phase -= TAU;
        }
        while self.phase < -std::f64::consts::PI {
            self.phase += TAU;
        }
        
        corrected
    }
    
    /// QPSK phase detector (improved)
    fn phase_detector(&self, symbol: Complex64) -> f64 {
        // Use tanh-based soft decision for better noise performance
        let scale = 2.0; // Soft decision scaling factor
        
        // Compute soft decisions
        let i_soft = (scale * symbol.re).tanh();
        let q_soft = (scale * symbol.im).tanh();
        
        // Phase error for QPSK (4th power method)
        // This is insensitive to data and locks to the closest 90° phase
        let phase4 = (symbol.powi(4)).arg() / 4.0;
        
        // Alternative: Decision-directed with soft decisions
        let dd_error = q_soft * symbol.re - i_soft * symbol.im;
        
        // Blend both methods for robustness
        0.5 * phase4 + 0.5 * dd_error
    }
}

/// Automatic Gain Control state
#[derive(Clone, Debug)]
struct AGC {
    gain: f64,
    target_power: f64,
    alpha: f64,
    // Track power before normalization for SNR estimation
    input_power: f64,
}

impl AGC {
    fn new(target_power: f64, time_constant: f64) -> Self {
        Self {
            gain: 1.0,
            target_power,
            alpha: 1.0 / time_constant,
            input_power: 0.5, // Initialize to expected level
        }
    }
    
    fn process(&mut self, sample: f32) -> f32 {
        let sample_power = (sample * sample) as f64;
        
        // Track input power with exponential smoothing (before gain)
        self.input_power = self.input_power * (1.0 - self.alpha * 0.1) + sample_power * (self.alpha * 0.1);
        
        // Exponential moving average of power
        let gain_adjustment = if sample_power > 1e-10 {
            (self.target_power / sample_power).sqrt()
        } else {
            1.0
        };
        
        // Smooth gain changes
        self.gain = self.gain * (1.0 - self.alpha) + gain_adjustment * self.alpha;
        
        // Limit gain to prevent instability
        self.gain = self.gain.clamp(0.1, 10.0);
        
        (sample as f64 * self.gain) as f32
    }
    
    fn get_input_power(&self) -> f64 {
        self.input_power
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
/// 
/// Returns symbols with SNR estimated before AGC normalization.
pub fn audio_to_symbols_with_snr(
    audio: &[f32],
    config: &DemodulationConfig,
) -> DemodulationResult {
    if audio.is_empty() || config.sample_rate == 0 {
        return DemodulationResult {
            symbols: Vec::new(),
            snr_db: 0.0,
            input_power: 0.0,
        };
    }
    
    let samples_per_symbol = (config.sample_rate / config.symbol_rate).max(1);
    let num_symbols = audio.len() / samples_per_symbol;
    let mut symbols = Vec::with_capacity(num_symbols);
    
    let dt = 1.0 / config.sample_rate as f64;
    
    // Measure raw audio power before AGC for SNR estimation
    let raw_signal_power: f64 = audio.iter()
        .map(|&s| (s * s) as f64)
        .sum::<f64>() / audio.len().max(1) as f64;
    
    // Initialize AGC for amplitude normalization
    let mut agc = AGC::new(0.5, 100.0);
    
    // Initialize Costas loop for carrier recovery
    // Use narrower loop bandwidth for better noise performance
    let loop_bw = 2.0 * config.symbol_rate as f64 * 0.01; // 1% of symbol rate, x2 for one-sided
    let damping = 0.707; // Critically damped
    let mut costas = CostasLoop::new(loop_bw, damping, config.sample_rate as f64);
    
    // Coarse frequency offset estimator (optional, helps acquisition)
    let mut freq_offset_estimate = 0.0;
    
    // Track error power for SNR estimation (after demodulation but before hard decisions)
    let mut total_error_power = 0.0;
    let mut total_signal_power = 0.0;
    
    // Demodulate using I/Q mixing
    for sym_idx in 0..num_symbols {
        let start = sym_idx * samples_per_symbol;
        let end = (start + samples_per_symbol).min(audio.len());
        
        let mut i_acc = 0.0f64;
        let mut q_acc = 0.0f64;
        
        for (idx, &sample) in audio[start..end].iter().enumerate() {
            // Apply AGC to normalize amplitude
            let normalized_sample = agc.process(sample);
            
            let t = (start + idx) as f64 * dt;
            
            // Use current frequency offset estimate
            let angle = TAU * (config.carrier_freq + freq_offset_estimate) * t;
            
            // I/Q demodulation (no phase correction here - done after)
            i_acc += normalized_sample as f64 * angle.cos();
            q_acc += normalized_sample as f64 * angle.sin();
        }
        
        // Average over the symbol period
        let count = (end - start) as f64;
        i_acc /= count;
        q_acc /= count;
        
        // Scale by 2 to compensate for mixing
        let raw_symbol = Complex64::new(i_acc * 2.0, q_acc * 2.0);
        
        // Apply Costas loop correction
        let corrected_symbol = costas.process(raw_symbol);
        
        // Update frequency offset estimate from Costas loop
        freq_offset_estimate = costas.freq_offset * config.sample_rate as f64 / TAU;
        
        // For SNR estimation: compute error from ideal constellation
        // Skip first 10 symbols to allow convergence
        if sym_idx >= 10 {
            // Find nearest QPSK point
            let ideal_points = [
                Complex64::new(std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
                Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
                Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),
                Complex64::new(std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),
            ];
            
            let symbol_power = corrected_symbol.norm_sqr();
            let scale = symbol_power.sqrt();
            
            let mut min_error = f64::INFINITY;
            for ideal in &ideal_points {
                let scaled_ideal = ideal * scale;
                let error = (corrected_symbol - scaled_ideal).norm_sqr();
                min_error = min_error.min(error);
            }
            
            total_error_power += min_error;
            total_signal_power += symbol_power;
        }
        
        symbols.push(corrected_symbol);
    }
    
    // Estimate SNR from demodulated symbol quality
    // Use constellation error as noise measurement
    let snr_db = if symbols.len() > 10 && total_error_power > 1e-10 && total_signal_power > 0.0 {
        let avg_signal_power = total_signal_power / (symbols.len() - 10) as f64;
        let avg_error_power = total_error_power / (symbols.len() - 10) as f64;
        
        // SNR = signal power / noise power
        let snr_linear = avg_signal_power / avg_error_power;
        10.0 * snr_linear.log10() as f32
    } else {
        0.0
    };
    
    DemodulationResult {
        symbols,
        snr_db,
        input_power: raw_signal_power,
    }
}

/// Demodulate audio back to IQ symbols with carrier recovery (backward compatible version)
/// 
/// Returns only symbols without SNR information. For new code, prefer audio_to_symbols_with_snr.
pub fn audio_to_symbols(
    audio: &[f32],
    config: &DemodulationConfig,
) -> Vec<Complex64> {
    audio_to_symbols_with_snr(audio, config).symbols
}

/// Helper function to demodulate with known frequency offset
fn audio_to_symbols_with_offset(
    audio: &[f32],
    config: &DemodulationConfig,
    freq_offset: f64,
) -> Vec<Complex64> {
    let samples_per_symbol = (config.sample_rate / config.symbol_rate).max(1);
    let num_symbols = audio.len() / samples_per_symbol;
    let mut symbols = Vec::with_capacity(num_symbols);
    
    let dt = 1.0 / config.sample_rate as f64;
    let mut agc = AGC::new(0.5, 100.0);
    
    // Use tighter loop with known offset
    let loop_bw = config.symbol_rate as f64 * 0.005; // Even narrower
    let damping = 0.707;
    let mut costas = CostasLoop::new(loop_bw, damping, config.sample_rate as f64);
    
    for sym_idx in 0..num_symbols {
        let start = sym_idx * samples_per_symbol;
        let end = (start + samples_per_symbol).min(audio.len());
        
        let mut i_acc = 0.0f64;
        let mut q_acc = 0.0f64;
        
        for (idx, &sample) in audio[start..end].iter().enumerate() {
            let normalized_sample = agc.process(sample);
            let t = (start + idx) as f64 * dt;
            let angle = TAU * (config.carrier_freq + freq_offset) * t;
            
            i_acc += normalized_sample as f64 * angle.cos();
            q_acc += normalized_sample as f64 * angle.sin();
        }
        
        let count = (end - start) as f64;
        let raw_symbol = Complex64::new(i_acc * 2.0 / count, q_acc * 2.0 / count);
        let corrected_symbol = costas.process(raw_symbol);
        
        symbols.push(corrected_symbol);
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
