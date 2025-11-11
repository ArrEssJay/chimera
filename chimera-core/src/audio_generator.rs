//! Audio signal generators for testing and intermodulation simulation
//!
//! Provides pink noise, constant tone, and frequency sweep generators
//! All signals are generated at 48kHz sample rate with float32 output
//! Tones are band-limited appropriately for Nyquist (< 24kHz)

use std::f64::consts::PI;

/// Audio generator types for testing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneratorType {
    /// Pink noise (1/f spectrum)
    PinkNoise,
    /// Constant sine tone at specified frequency (Hz)
    Tone(f64),
    /// Linear frequency sweep from 100Hz to 20kHz
    SweepLinear,
    /// Logarithmic frequency sweep from 100Hz to 20kHz
    SweepLog,
}

/// Generate audio samples based on generator type
pub fn generate_audio(
    generator_type: GeneratorType,
    duration_secs: f64,
    sample_rate: usize,
) -> Vec<f32> {
    let num_samples = (duration_secs * sample_rate as f64) as usize;
    
    match generator_type {
        GeneratorType::PinkNoise => generate_pink_noise(num_samples),
        GeneratorType::Tone(freq_hz) => generate_tone(freq_hz, num_samples, sample_rate),
        GeneratorType::SweepLinear => generate_sweep_linear(num_samples, sample_rate),
        GeneratorType::SweepLog => generate_sweep_log(num_samples, sample_rate),
    }
}

/// Generate pink noise using the Voss-McCartney algorithm
/// Pink noise has equal energy per octave (1/f spectrum)
pub fn generate_pink_noise(num_samples: usize) -> Vec<f32> {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut output = Vec::with_capacity(num_samples);
    
    // Voss-McCartney algorithm with 16 octaves
    const NUM_GENERATORS: usize = 16;
    let mut generators = [0.0f64; NUM_GENERATORS];
    let mut counter = 0u32;
    
    for _ in 0..num_samples {
        // Update generators based on bit pattern of counter
        let mut changed_bits = counter ^ (counter + 1);
        for gen in generators.iter_mut() {
            if changed_bits & 1 != 0 {
                *gen = rng.gen_range(-1.0..1.0);
            }
            changed_bits >>= 1;
        }
        
        // Sum all generators
        let sum: f64 = generators.iter().sum();
        
        // Normalize to approximately [-1, 1]
        let normalized = sum / (NUM_GENERATORS as f64).sqrt();
        output.push(normalized as f32);
        
        counter = counter.wrapping_add(1);
    }
    
    output
}

/// Generate a constant frequency sine tone
/// Frequency is band-limited to ensure it's below Nyquist
pub fn generate_tone(freq_hz: f64, num_samples: usize, sample_rate: usize) -> Vec<f32> {
    let nyquist = sample_rate as f64 / 2.0;
    
    // Enforce Nyquist limit with safety margin
    let freq = if freq_hz >= nyquist * 0.95 {
        nyquist * 0.95
    } else {
        freq_hz
    };
    
    let omega = 2.0 * PI * freq / sample_rate as f64;
    
    (0..num_samples)
        .map(|i| (omega * i as f64).sin() as f32)
        .collect()
}

/// Generate a linear frequency sweep from 100Hz to 20kHz
/// Uses instantaneous frequency that increases linearly with time
pub fn generate_sweep_linear(num_samples: usize, sample_rate: usize) -> Vec<f32> {
    const START_FREQ: f64 = 100.0;
    const END_FREQ: f64 = 20_000.0;
    
    let nyquist = sample_rate as f64 / 2.0;
    let max_freq = (nyquist * 0.95).min(END_FREQ);
    
    let duration = num_samples as f64 / sample_rate as f64;
    let freq_rate = (max_freq - START_FREQ) / duration; // Hz per second
    
    let mut phase = 0.0;
    let mut output = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let freq = START_FREQ + freq_rate * t;
        
        // Phase accumulation for proper sine generation
        let phase_increment = 2.0 * PI * freq / sample_rate as f64;
        phase += phase_increment;
        
        // Keep phase bounded to avoid precision issues
        if phase > 2.0 * PI {
            phase -= 2.0 * PI;
        }
        
        output.push(phase.sin() as f32);
    }
    
    output
}

/// Generate a logarithmic frequency sweep from 100Hz to 20kHz
/// Frequency increases exponentially, spending equal time per octave
pub fn generate_sweep_log(num_samples: usize, sample_rate: usize) -> Vec<f32> {
    const START_FREQ: f64 = 100.0;
    const END_FREQ: f64 = 20_000.0;
    
    let nyquist = sample_rate as f64 / 2.0;
    let max_freq = (nyquist * 0.95).min(END_FREQ);
    
    let duration = num_samples as f64 / sample_rate as f64;
    let log_freq_range = (max_freq / START_FREQ).ln();
    
    let mut phase = 0.0;
    let mut output = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let freq = START_FREQ * ((log_freq_range * t / duration).exp());
        
        // Phase accumulation
        let phase_increment = 2.0 * PI * freq / sample_rate as f64;
        phase += phase_increment;
        
        // Keep phase bounded
        if phase > 2.0 * PI {
            phase -= 2.0 * PI;
        }
        
        output.push(phase.sin() as f32);
    }
    
    output
}

/// Apply a fade-in and fade-out envelope to prevent clicks
pub fn apply_fade_envelope(samples: &mut [f32], fade_samples: usize) {
    let fade_len = fade_samples.min(samples.len() / 2);
    let total_samples = samples.len();
    
    // Fade in (first fade_len samples)
    for (i, sample) in samples.iter_mut().enumerate().take(fade_len) {
        let fade = i as f32 / fade_len as f32;
        // Use sine curve for smooth fade
        let envelope = (fade * std::f32::consts::PI / 2.0).sin();
        *sample *= envelope;
    }
    
    // Fade out (last fade_len samples)
    let start_idx = total_samples.saturating_sub(fade_len);
    for (i, sample) in samples.iter_mut().enumerate().skip(start_idx) {
        let fade = (total_samples - i) as f32 / fade_len as f32;
        let envelope = (fade * std::f32::consts::PI / 2.0).sin();
        *sample *= envelope;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pink_noise_generation() {
        let samples = generate_pink_noise(48000);
        assert_eq!(samples.len(), 48000);
        
        // Check that samples are bounded
        for &sample in &samples {
            assert!(sample.abs() <= 5.0); // Allow some headroom for pink noise
        }
    }
    
    #[test]
    fn test_tone_generation() {
        let samples = generate_tone(1000.0, 480, 48000);
        assert_eq!(samples.len(), 480);
        
        // Check amplitude is reasonable
        let max_amp = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(max_amp > 0.9 && max_amp <= 1.0);
    }
    
    #[test]
    fn test_nyquist_limiting() {
        let sample_rate = 48000;
        let nyquist = sample_rate as f64 / 2.0;
        
        // Try to generate a tone above Nyquist
        let samples = generate_tone(nyquist * 1.5, 480, sample_rate);
        
        // Should not crash and should produce valid samples
        assert_eq!(samples.len(), 480);
        for &sample in &samples {
            assert!(sample.is_finite());
        }
    }
    
    #[test]
    fn test_sweep_linear() {
        let samples = generate_sweep_linear(48000, 48000);
        assert_eq!(samples.len(), 48000);
        
        // Check samples are bounded
        for &sample in &samples {
            assert!(sample.abs() <= 1.0);
        }
    }
    
    #[test]
    fn test_sweep_log() {
        let samples = generate_sweep_log(48000, 48000);
        assert_eq!(samples.len(), 48000);
        
        // Check samples are bounded
        for &sample in &samples {
            assert!(sample.abs() <= 1.0);
        }
    }
    
    #[test]
    fn test_fade_envelope() {
        let mut samples = vec![1.0f32; 1000];
        apply_fade_envelope(&mut samples, 100);
        
        // Check fade in
        assert!(samples[0] < 0.1);
        assert!(samples[50] < samples[99]);
        assert!((samples[100] - 1.0).abs() < 0.1);
        
        // Check fade out
        assert!((samples[899] - 1.0).abs() < 0.1);
        assert!(samples[950] < samples[900]);
        assert!(samples[999] < 0.1);
    }
}
