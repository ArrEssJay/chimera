//! Channel modeling and impairment simulation
//!
//! This module provides functions to simulate realistic channel effects
//! including AWGN (Additive White Gaussian Noise) and attenuation.

use num_complex::Complex64;
use rand::Rng;
use rand_distr::StandardNormal;

/// Apply AWGN noise to audio samples
/// 
/// # Arguments
/// * `audio` - Input audio samples
/// * `noise_std` - Standard deviation of noise (controls SNR)
/// * `rng` - Random number generator for noise samples
/// 
/// # Returns
/// New vector with noise added
pub fn apply_audio_noise<R: Rng>(
    audio: &[f32],
    noise_std: f64,
    rng: &mut R,
) -> Vec<f32> {
    let mut noisy = audio.to_vec();
    
    for sample in noisy.iter_mut() {
        let noise: f64 = rng.sample::<f64, _>(StandardNormal) * (noise_std * 0.1);
        *sample += noise as f32;
    }
    
    noisy
}

/// Apply AWGN noise to complex symbols
/// 
/// # Arguments
/// * `symbols` - Input IQ symbols
/// * `noise_std` - Standard deviation of noise (controls SNR)
/// * `rng` - Random number generator
/// 
/// # Returns
/// New vector with noise added to both I and Q components
pub fn apply_symbol_noise<R: Rng>(
    symbols: &[Complex64],
    noise_std: f64,
    rng: &mut R,
) -> Vec<Complex64> {
    symbols.iter().map(|symbol| {
        let noise_i: f64 = rng.sample::<f64, _>(StandardNormal) * noise_std;
        let noise_q: f64 = rng.sample::<f64, _>(StandardNormal) * noise_std;
        symbol + Complex64::new(noise_i, noise_q)
    }).collect()
}

/// Apply channel attenuation and noise to symbols
/// 
/// # Arguments
/// * `symbols` - Input IQ symbols
/// * `attenuation` - Attenuation factor (< 1.0 reduces power)
/// * `noise_std` - Noise standard deviation
/// * `rng` - Random number generator
pub fn apply_channel<R: Rng>(
    symbols: &[Complex64],
    attenuation: f64,
    noise_std: f64,
    rng: &mut R,
) -> Vec<Complex64> {
    symbols.iter().map(|symbol| {
        let attenuated = symbol * attenuation;
        let noise_i: f64 = rng.sample::<f64, _>(StandardNormal) * noise_std;
        let noise_q: f64 = rng.sample::<f64, _>(StandardNormal) * noise_std;
        attenuated + Complex64::new(noise_i, noise_q)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_audio_noise_deterministic_with_seed() {
        let audio = vec![0.1, 0.2, 0.3];
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        
        let noisy1 = apply_audio_noise(&audio, 0.01, &mut rng1);
        let noisy2 = apply_audio_noise(&audio, 0.01, &mut rng2);
        
        assert_eq!(noisy1, noisy2);
    }

    #[test]
    fn test_symbol_noise_affects_both_components() {
        let symbols = vec![Complex64::new(1.0, 0.0)];
        let mut rng = StdRng::seed_from_u64(42);
        
        let noisy = apply_symbol_noise(&symbols, 0.1, &mut rng);
        
        assert_ne!(noisy[0].re, 1.0);
        assert_ne!(noisy[0].im, 0.0);
    }

    #[test]
    fn test_channel_attenuation_reduces_power() {
        let symbols = vec![Complex64::new(1.0, 1.0)];
        let mut rng = StdRng::seed_from_u64(42);
        
        let processed = apply_channel(&symbols, 0.5, 0.0, &mut rng);
        
        // With attenuation 0.5 and no noise, power should be reduced
        let original_power = symbols[0].norm_sqr();
        let processed_power = processed[0].norm_sqr();
        
        assert!(processed_power < original_power);
    }

    #[test]
    fn test_zero_noise_std_preserves_signal() {
        let symbols = vec![Complex64::new(1.0, 0.5)];
        let mut rng = StdRng::seed_from_u64(42);
        
        let noisy = apply_symbol_noise(&symbols, 0.0, &mut rng);
        
        assert_eq!(noisy[0], symbols[0]);
    }
}
