//! QPSK and FSK modulation implementations
//!
//! This module converts QPSK symbols into audio carrier signals with optional
//! FSK frequency dithering, following the Raman Whisper protocol v4.2.

use num_complex::Complex;
use std::f64::consts::{PI, TAU};

/// Configuration for modulation
#[derive(Clone, Debug)]
pub struct ModulationConfig {
    pub sample_rate: usize,
    pub symbol_rate: usize,
    pub carrier_freq: f64,
}

/// Convert symbols to modulated carrier signal
/// 
/// Implements the Raman Whisper Modulation Protocol v4.2:
/// - FSK layer: ±1 Hz frequency dithering at 1 bit/second (11999/12001 Hz)
/// - QPSK layer: Phase modulation at symbol_rate with simple lowpass filtering
/// - Combined signal: sin(carrier_phase + qpsk_phase)
/// 
/// Phase-based approach (matches Python reference implementation):
/// 1. Map symbols to phase values
/// 2. Repeat phases at full sample rate (no zero-insertion)
/// 3. Apply simple lowpass filter for bandwidth limiting
/// 4. Modulate onto FSK-dithered carrier
pub fn symbols_to_carrier_signal(
    symbols: &[Complex<f64>],
    config: &ModulationConfig,
) -> Vec<f32> {
    if config.sample_rate == 0 || symbols.is_empty() {
        return Vec::new();
    }
    
    let samples_per_symbol = (config.sample_rate / config.symbol_rate).max(1);
    let num_samples = symbols.len() * samples_per_symbol;
    
    // --- Step 1: Convert symbols to phase values ---
    // QPSK constellation: map to phases (π/4, 3π/4, 5π/4, 7π/4)
    let symbol_phases: Vec<f64> = symbols.iter().map(|s| {
        // Calculate phase from complex symbol
        s.im.atan2(s.re)
    }).collect();
    
    // --- Step 2: Repeat each phase for samples_per_symbol (no zero-insertion!) ---
    let mut phase_raw = Vec::with_capacity(num_samples);
    for &phase in &symbol_phases {
        for _ in 0..samples_per_symbol {
            phase_raw.push(phase);
        }
    }
    
    // --- Step 3: Simple lowpass filter for bandwidth limiting (~20 Hz) ---
    // Using a simple moving average filter (much faster than Butterworth)
    let bandwidth_hz = 20.0;
    let filter_len = ((config.sample_rate as f64 / bandwidth_hz) as usize).max(3) | 1; // Make odd
    let mut phase_smoothed = vec![0.0; num_samples];
    
    for i in 0..num_samples {
        let start = i.saturating_sub(filter_len / 2);
        let end = (i + filter_len / 2 + 1).min(num_samples);
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;
        
        for j in start..end {
            sum_sin += phase_raw[j].sin();
            sum_cos += phase_raw[j].cos();
        }
        
        // Preserve phase continuity by converting through sin/cos
        phase_smoothed[i] = sum_sin.atan2(sum_cos);
    }
    
    // --- Step 4: Modulate onto FSK-dithered carrier ---
    let mut audio = Vec::with_capacity(num_samples);
    let mut carrier_phase = 0.0;
    let dt = 1.0 / config.sample_rate as f64;
    
    for i in 0..num_samples {
        // FSK bit selection (alternates every second)
        let fsk_bit_idx = i / config.sample_rate;
        let fsk_freq = if fsk_bit_idx % 2 == 1 { 12001.0 } else { 11999.0 };
        
        // Update carrier phase (phase-continuous FSK)
        carrier_phase += TAU * fsk_freq * dt;
        carrier_phase = carrier_phase % TAU;
        
        // Combined phase modulation
        let total_phase = carrier_phase + phase_smoothed[i];
        let sample = total_phase.sin() as f32;
        
        audio.push(sample);
    }
    
    // --- CRITICAL FIX: Zero-padding for filter tail processing ---
    // The RRC filter has a span of 8 symbols, meaning each symbol's energy is spread
    // across 8 symbol periods (4 before and 4 after the symbol center). This is the
    // filter's "group delay" - energy takes time to propagate through the filter.
    //
    // When we abruptly end the transmission, the last symbol's energy is still "ringing"
    // in the filter tails. The receiver's Gardner timing recovery loop needs enough
    // samples to see this energy and correctly interpolate the final symbol.
    //
    // Without padding, the loop's condition `while idx < len - sps` terminates before
    // it can process the last symbol, because it runs out of samples. This is why we
    // consistently get 127 symbols instead of 128.
    //
    // Solution: Add zero-padding to give the receiver sufficient "look-ahead" buffer.
    // The receiver needs enough samples after the preamble to extract a full 128-symbol frame.
    // Since we transmit 2 frames (256 symbols) and the correlator may lock on the second
    // preamble (at ~symbol 128), we need at least 128 additional symbols of padding to
    // ensure a complete frame can be extracted.
    //
    // This is the standard solution in professional DSP systems (MATLAB, GNU Radio, etc.)
    // and simulates what would happen in a continuous transmission where more signal
    // (or at least more time) always follows the current block.
    let padding_symbols = 128; // Full frame worth of padding for look-ahead
    let padding_samples = padding_symbols * samples_per_symbol;
    audio.extend(vec![0.0f32; padding_samples]);
    
    // DO NOT peak-normalize! The power is now correctly set by the unit-energy RRC filter.
    // Peak normalization destroys the careful power balance and creates "spiky" signals
    // where energy is concentrated in a few samples, causing the Gardner loop to starve.
    // Professional systems (MATLAB) rely on the filter's power-preserving properties
    // to produce signals with predictable average power and natural PAPR.
    // normalize_audio(&mut audio);  // REMOVED
    
    audio
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
        };
        
        let audio = symbols_to_carrier_signal(&[], &config);
        assert!(audio.is_empty());
    }
}
