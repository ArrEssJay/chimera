//! QPSK and FSK modulation implementations
//!
//! This module converts QPSK symbols into audio carrier signals with optional
//! FSK frequency dithering, following the Raman Whisper protocol v4.2.

use num_complex::Complex;
use std::f64::consts::TAU;
use super::filters::apply_rrc_filter;

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
/// - QPSK layer: Phase modulation at symbol_rate with ~20 Hz bandwidth via RRC filtering
/// - Combined signal: I*cos(ωt) - Q*sin(ωt)
/// 
/// Uses proper RRC pulse shaping at sample rate for bandwidth limiting and ISI reduction.
pub fn symbols_to_carrier_signal(
    symbols: &[Complex<f64>],
    config: &ModulationConfig,
) -> Vec<f32> {
    if config.sample_rate == 0 || symbols.is_empty() {
        return Vec::new();
    }
    
    let samples_per_symbol = (config.sample_rate / config.symbol_rate).max(1);
    let num_samples = symbols.len() * samples_per_symbol;
    
    // Step 1: Upsample symbols to sample rate (zero-insertion for pulse shaping)
    let mut i_upsampled = vec![0.0f32; num_samples];
    let mut q_upsampled = vec![0.0f32; num_samples];
    
    if config.enable_qpsk {
        for (idx, symbol) in symbols.iter().enumerate() {
            let sample_idx = idx * samples_per_symbol;
            i_upsampled[sample_idx] = symbol.re as f32;
            q_upsampled[sample_idx] = symbol.im as f32;
        }
    }
    
    // Step 2: Apply RRC pulse shaping filter at sample rate
    let i_filtered = if config.enable_qpsk {
        let filtered = apply_rrc_filter(&i_upsampled, config.sample_rate, config.symbol_rate);
        
        // CRITICAL: Apply interpolation gain to compensate for upsampling.
        // The RRC filter has unit ENERGY normalization. When we upsample by inserting
        // zeros, we spread the symbol's energy over `sps` samples, reducing power by
        // a factor of sps. Since power ∝ amplitude², we multiply by sqrt(sps) to
        // restore the original power. This gain is sqrt(sps), not sps, because the
        // filter's unit-energy property already provides the correct normalization.
        // This ensures symbol magnitude ~1.0, matching receiver expectations.
        let gain = (samples_per_symbol as f32).sqrt();
        filtered.iter().map(|&x| x * gain).collect()
    } else {
        vec![0.0f32; num_samples]
    };
    
    let q_filtered = if config.enable_qpsk {
        let filtered = apply_rrc_filter(&q_upsampled, config.sample_rate, config.symbol_rate);
        let gain = (samples_per_symbol as f32).sqrt();
        filtered.iter().map(|&x| x * gain).collect()
    } else {
        vec![0.0f32; num_samples]
    };
    
    // Step 3: Modulate onto carrier with optional FSK
    let mut audio = Vec::with_capacity(num_samples);
    let samples_per_fsk_bit = config.sample_rate; // 1 bit/second
    let num_fsk_bits = (num_samples + samples_per_fsk_bit - 1) / samples_per_fsk_bit;
    let fsk_bits: Vec<u8> = (0..num_fsk_bits).map(|i| (i % 2) as u8).collect();
    
    for sample_idx in 0..num_samples {
        // FSK frequency selection
        let fsk_bit_idx = sample_idx / samples_per_fsk_bit;
        let fsk_freq = if config.enable_fsk && fsk_bit_idx < fsk_bits.len() && fsk_bits[fsk_bit_idx] == 1 {
            12001.0
        } else if config.enable_fsk {
            11999.0
        } else {
            config.carrier_freq
        };
        
        // Calculate instantaneous phase
        let t = sample_idx as f64 / config.sample_rate as f64;
        let carrier_phase = TAU * fsk_freq * t;
        
        // QPSK modulation: I*cos(ωt) - Q*sin(ωt)
        let sample = i_filtered[sample_idx] * (carrier_phase.cos() as f32) -
                    q_filtered[sample_idx] * (carrier_phase.sin() as f32);
        
        audio.push(sample);
    }
    
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
