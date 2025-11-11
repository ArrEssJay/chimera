//! QPSK and FSK modulation implementations
//!
//! This module converts QPSK symbols into audio carrier signals with optional
//! FSK frequency dithering, following the Raman Whisper protocol v4.2.

use num_complex::Complex;
use std::f64::consts::TAU;

/// Configuration for modulation
#[derive(Clone, Debug)]
pub struct ModulationConfig {
    pub sample_rate: usize,
    pub symbol_rate: usize,
    pub carrier_freq: f64,
    pub enable_qpsk: bool,
    pub enable_fsk: bool,
}

/// Convert symbols to modulated carrier signal
/// 
/// Implements the Raman Whisper Modulation Protocol v4.2:
/// - FSK layer: ±1 Hz frequency dithering at 1 bit/second (11999/12001 Hz)
/// - QPSK layer: Phase modulation at symbol_rate with ~20 Hz bandwidth
/// - Combined signal: sin(fsk_phase + qpsk_phase)
/// 
/// Debug parameters allow disabling QPSK/FSK to isolate the pure carrier
pub fn symbols_to_carrier_signal(
    symbols: &[Complex<f64>],
    config: &ModulationConfig,
) -> Vec<f32> {
    if config.sample_rate == 0 || symbols.is_empty() {
        return Vec::new();
    }
    
    let samples_per_symbol = (config.sample_rate / config.symbol_rate).max(1);
    let samples_per_fsk_bit = config.sample_rate; // FSK at 1 bit/second
    let num_samples = symbols.len() * samples_per_symbol;
    let mut audio = Vec::with_capacity(num_samples);
    
    // Pre-compute QPSK phases for all symbols
    let qpsk_phases: Vec<f64> = symbols.iter()
        .map(|symbol| symbol.arg())
        .collect();
    
    // Apply low-pass filter to QPSK phases to limit bandwidth to ~20 Hz
    let filtered_phases = lowpass_filter_phases(&qpsk_phases, config.sample_rate);
    
    // Generate FSK bit pattern (simple alternating pattern for demo)
    let num_fsk_bits = (num_samples + samples_per_fsk_bit - 1) / samples_per_fsk_bit;
    let fsk_bits: Vec<u8> = (0..num_fsk_bits)
        .map(|i| (i % 2) as u8)
        .collect();
    
    // Phase accumulator for FSK carrier
    let mut carrier_phase = 0.0f64;
    
    // Generate audio samples with combined FSK+QPSK modulation
    for sample_idx in 0..num_samples {
        // Determine current FSK frequency (11999 or 12001 Hz)
        let fsk_bit_idx = sample_idx / samples_per_fsk_bit;
        let fsk_freq = if config.enable_fsk && fsk_bit_idx < fsk_bits.len() && fsk_bits[fsk_bit_idx] == 1 {
            12001.0
        } else if config.enable_fsk {
            11999.0
        } else {
            config.carrier_freq  // Pure carrier if FSK disabled
        };
        
        // Accumulate carrier phase with FSK frequency
        carrier_phase += TAU * fsk_freq / config.sample_rate as f64;
        
        // Get interpolated QPSK phase for this sample
        let symbol_idx = sample_idx / samples_per_symbol;
        let qpsk_phase = if config.enable_qpsk && symbol_idx < filtered_phases.len() {
            filtered_phases[symbol_idx]
        } else {
            0.0  // No QPSK modulation when disabled
        };
        
        // Combine FSK carrier phase with QPSK phase offset
        let total_phase = carrier_phase + qpsk_phase;
        
        // Generate sample
        audio.push(total_phase.sin() as f32);
        
        // Wrap phase to prevent overflow
        if carrier_phase > TAU {
            carrier_phase -= TAU;
        }
    }
    
    normalize_audio(&mut audio);
    audio
}

/// Apply low-pass filter to phase values to limit bandwidth
fn lowpass_filter_phases(phases: &[f64], sample_rate: usize) -> Vec<f64> {
    // Simple moving average filter approximation
    let filter_window = (sample_rate as f64 / 40.0) as usize; // ~40 Hz cutoff
    let mut filtered = vec![0.0; phases.len()];
    
    for i in 0..phases.len() {
        let start = i.saturating_sub(filter_window / 2);
        let end = (i + filter_window / 2 + 1).min(phases.len());
        let sum: f64 = phases[start..end].iter().sum();
        filtered[i] = sum / (end - start) as f64;
    }
    
    filtered
}

/// Normalize audio samples to prevent clipping
pub fn normalize_audio(samples: &mut [f32]) {
    let max_amp = samples.iter()
        .map(|&v| v.abs())
        .fold(0.0f32, f32::max);
    
    if max_amp > 1.0 {
        let scale = 1.0 / max_amp;
        for value in samples.iter_mut() {
            *value *= scale;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    #[test]
    fn test_symbols_to_carrier_produces_audio() {
        let symbols = vec![
            Complex64::new(0.707, 0.707),
            Complex64::new(-0.707, 0.707),
        ];
        let config = ModulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
            enable_qpsk: true,
            enable_fsk: true,
        };
        
        let audio = symbols_to_carrier_signal(&symbols, &config);
        
        assert!(!audio.is_empty());
        assert_eq!(audio.len(), symbols.len() * (48000 / 16));
    }

    #[test]
    fn test_normalize_audio_prevents_clipping() {
        let mut samples = vec![0.5, 1.5, -2.0, 0.8];
        normalize_audio(&mut samples);
        
        for &sample in &samples {
            assert!(sample.abs() <= 1.0);
        }
    }

    #[test]
    fn test_empty_input_returns_empty() {
        let config = ModulationConfig {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
            enable_qpsk: true,
            enable_fsk: true,
        };
        
        let audio = symbols_to_carrier_signal(&[], &config);
        assert!(audio.is_empty());
    }
}
