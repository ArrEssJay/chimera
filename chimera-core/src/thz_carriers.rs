//! THz carrier simulation for AID (Auditory Intermodulation Distortion) effect
//!
//! Simulates the non-linear mixing of two THz carriers in neural tissue:
//! - Pump beam (F1): 1.998 THz unmodulated carrier
//! - Data carrier (F2): 1.875 THz AM-modulated with 12kHz audio signal
//!
//! The third-order intermodulation produces the perceived 12kHz audio.

use num_complex::Complex;
use rand::Rng;

/// Terahertz carrier configuration for AID effect
#[derive(Debug, Clone)]
pub struct ThzCarrierConfig {
    /// Pump beam frequency (F1) in THz
    pub pump_frequency: f64,
    /// Data carrier frequency (F2) in THz  
    pub data_frequency: f64,
    /// Pump beam power (relative)
    pub pump_power: f32,
    /// Data carrier power (relative)
    pub data_power: f32,
    /// Modulation depth (0.0 to 1.0)
    pub modulation_depth: f32,
    /// Non-linearity coefficient for mixing
    pub mixing_coefficient: f32,
    /// Phase noise standard deviation (radians)
    pub phase_noise_std: f32,
}

impl Default for ThzCarrierConfig {
    fn default() -> Self {
        Self {
            pump_frequency: 1.998e12,  // 1.998 THz
            data_frequency: 1.875e12,  // 1.875 THz
            pump_power: 1.0,
            data_power: 0.3,
            modulation_depth: 0.05,    // 5% idle state
            mixing_coefficient: 0.7,
            phase_noise_std: 0.001,
        }
    }
}

/// THz carrier generator with non-linear mixing simulation
pub struct ThzCarrierProcessor {
    config: ThzCarrierConfig,
    rng: rand::rngs::ThreadRng,
}

impl ThzCarrierProcessor {
    pub fn new(config: ThzCarrierConfig, _sample_rate: f64) -> Self {
        Self {
            config,
            rng: rand::thread_rng(),
        }
    }
    
    /// Set modulation depth for idle/active mode
    pub fn set_modulation_depth(&mut self, depth: f32) {
        self.config.modulation_depth = depth.clamp(0.0, 1.0);
    }
    
    /// Set mixing coefficient (simulates biological response)
    pub fn set_mixing_coefficient(&mut self, coeff: f32) {
        self.config.mixing_coefficient = coeff.clamp(0.0, 1.0);
    }
    
    /// Apply AM modulation to data carrier
    pub fn modulate_data_carrier(&mut self, audio_signal: &[f32]) -> Vec<Complex<f32>> {
        let mut output = Vec::with_capacity(audio_signal.len());
        
        // THz carriers are at ~2 THz - far above audio frequencies
        // We don't simulate their actual oscillations, just their modulation envelope
        // The "carrier" phase here represents slow variations, not the THz oscillation itself
        
        for &audio_sample in audio_signal {
            // Add phase noise for realism (laser phase noise)
            let phase_noise = (self.rng.gen::<f32>() - 0.5) * self.config.phase_noise_std;
            
            // The AM modulation envelope (not an oscillating carrier!)
            // This represents the amplitude variation of the THz carrier
            let modulation = 1.0 + self.config.modulation_depth * audio_sample;
            
            // Generate complex signal representing the modulation envelope
            // Phase represents the instantaneous state, not a frequency
            let amplitude = self.config.pump_power + self.config.data_power * modulation;
            let combined = Complex::from_polar(
                amplitude,
                phase_noise
            );
            
            output.push(combined);
        }
        
        output
    }
    
    /// Simulate non-linear mixing (demodulation in neural tissue)
    /// Implements third-order intermodulation: E1^2 * E2
    pub fn nonlinear_mixing(&self, signal: &[Complex<f32>]) -> Vec<f32> {
        let mut output = Vec::with_capacity(signal.len());
        
        for &sample in signal {
            let magnitude = sample.norm();
            
            // Third-order intermodulation: |signal|² * Re(signal)
            // This creates sum and difference frequencies
            // The difference frequency (F2 - F1) falls in the audio range
            let nonlinear_term = magnitude * magnitude * sample.re;
            
            // Apply mixing coefficient (represents biological response efficiency)
            let mixed = self.config.mixing_coefficient * nonlinear_term;
            
            output.push(mixed);
        }
        
        // Apply simple DC blocking to center around zero
        let mean: f32 = output.iter().sum::<f32>() / output.len() as f32;
        output.iter_mut().for_each(|x| *x -= mean);
        
        // Normalize to maintain signal amplitude
        // Find peak amplitude
        let max_abs = output.iter()
            .map(|&x| x.abs())
            .fold(0.0f32, f32::max);
        
        // Scale to preserve typical audio range (-1.0 to 1.0)
        // Targeting ~0.5 peak for headroom
        if max_abs > 1e-6 {
            let scale = 0.5 / max_abs;
            output.iter_mut().for_each(|x| *x *= scale);
        }
        
        output
    }
    
    /// Get current configuration
    pub fn config(&self) -> &ThzCarrierConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_modulation_depth_clamping() {
        let config = ThzCarrierConfig::default();
        let mut processor = ThzCarrierProcessor::new(config, 48000.0);
        
        processor.set_modulation_depth(1.5);
        assert_eq!(processor.config.modulation_depth, 1.0);
        
        processor.set_modulation_depth(-0.5);
        assert_eq!(processor.config.modulation_depth, 0.0);
    }
    
    #[test]
    fn test_carrier_modulation() {
        let config = ThzCarrierConfig::default();
        let mut processor = ThzCarrierProcessor::new(config, 48000.0);
        
        let audio = vec![0.5f32; 100];
        let modulated = processor.modulate_data_carrier(&audio);
        
        assert_eq!(modulated.len(), 100);
        assert!(modulated.iter().all(|c| c.norm() > 0.0));
    }
    
    #[test]
    fn test_nonlinear_mixing() {
        let config = ThzCarrierConfig::default();
        let processor = ThzCarrierProcessor::new(config, 48000.0);
        
        let signal = vec![Complex::new(1.0, 0.5); 100];
        let mixed = processor.nonlinear_mixing(&signal);
        
        assert_eq!(mixed.len(), 100);
        
        // Check DC blocking worked
        let mean: f32 = mixed.iter().sum::<f32>() / mixed.len() as f32;
        assert!(mean.abs() < 1e-6);
    }
}
