//! THz carrier simulation for AID (Auditory Intermodulation Distortion) effect
//!
//! Simulates the non-linear mixing of two THz carriers in neural tissue:
//! - Pump beam (F1): 1.998 THz unmodulated carrier
//! - Data carrier (F2): 1.875 THz AM-modulated with 12kHz audio signal
//!
//! The third-order intermodulation produces the perceived 12kHz audio.
//!
//! ## Physical Model
//!
//! The simulation accurately models the heterodyne mixing process:
//! 1. **Carrier mixing**: E1·cos(2πF1t) × E2·(1+m·s(t))·cos(2πF2t)
//!    Produces difference frequency at F1-F2 = 123 GHz carrying the modulation
//! 2. **Biological demodulation**: Third-order intermodulation (E1² × E2)
//!    Extracts the audio envelope through non-linear neural tissue response
//!
//! Since we cannot numerically simulate THz oscillations at audio sample rates,
//! we model the mixing envelope - which is physically equivalent for the
//! demodulation process that produces the perceived audio signal.
//!
//! Extended to model secondary intermodulation between the AID signal and
//! external audio entering through normal auditory pathways.

use num_complex::Complex;
use rand::Rng;
use crate::config::AudioMixingConfig;

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
    /// Bypass THz simulation for validation (returns input signal unchanged)
    pub bypass_simulation: bool,
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
            bypass_simulation: false,
        }
    }
}

/// THz carrier generator with non-linear mixing simulation
pub struct ThzCarrierProcessor {
    config: ThzCarrierConfig,
    rng: rand::rngs::ThreadRng,
    external_audio: Option<Vec<f32>>,
    mixing_config: Option<AudioMixingConfig>,
}

impl ThzCarrierProcessor {
    pub fn new(config: ThzCarrierConfig, _sample_rate: f64) -> Self {
        Self {
            config,
            rng: rand::thread_rng(),
            external_audio: None,
            mixing_config: None,
        }
    }
    
    /// Load external audio for intermodulation mixing
    pub fn set_external_audio(&mut self, audio: Vec<f32>, mixing_config: AudioMixingConfig) {
        self.external_audio = Some(audio);
        self.mixing_config = Some(mixing_config);
    }
    
    /// Clear external audio
    pub fn clear_external_audio(&mut self) {
        self.external_audio = None;
        self.mixing_config = None;
    }
    
    /// Set modulation depth for idle/active mode
    pub fn set_modulation_depth(&mut self, depth: f32) {
        self.config.modulation_depth = depth.clamp(0.0, 1.0);
    }
    
    /// Set mixing coefficient (simulates biological response)
    pub fn set_mixing_coefficient(&mut self, coeff: f32) {
        self.config.mixing_coefficient = coeff.clamp(0.0, 1.0);
    }
    
    /// Apply AM modulation to data carrier and simulate THz mixing
    /// 
    /// This models the heterodyne mixing of:
    /// - Pump beam (F1 = 1.998 THz): unmodulated, high power
    /// - Data carrier (F2 = 1.875 THz): AM-modulated with audio signal
    /// 
    /// The mixing produces a difference frequency (F1 - F2 = 123 GHz) whose
    /// envelope carries the audio modulation. We simulate the envelope since
    /// we cannot numerically model THz oscillations at audio sample rates.
    /// 
    /// If bypass_simulation is enabled, returns the input signal as complex values (for validation).
    pub fn modulate_data_carrier(&mut self, audio_signal: &[f32]) -> Vec<Complex<f32>> {
        // Bypass: return input as complex for validation
        if self.config.bypass_simulation {
            return audio_signal.iter()
                .map(|&sample| Complex::new(sample, 0.0))
                .collect();
        }
        
        let mut output = Vec::with_capacity(audio_signal.len());
        
        // Calculate difference frequency for reference (though we model the envelope)
        let _diff_freq = (self.config.pump_frequency - self.config.data_frequency).abs();
        
        for &audio_sample in audio_signal {
            // Laser phase noise (accumulates over time for realism)
            let phase_noise = (self.rng.gen::<f32>() - 0.5) * self.config.phase_noise_std;
            
            // Pump beam envelope (unmodulated, constant power)
            let pump_envelope = self.config.pump_power;
            
            // Data carrier with AM modulation
            // Modulation: (1 + m·s(t)) where m is depth and s(t) is audio
            let modulation = 1.0 + self.config.modulation_depth * audio_sample;
            let data_envelope = self.config.data_power * modulation;
            
            // Heterodyne mixing of the two THz carriers
            // In the real system: E1·cos(2πF1t) × E2·(1+m·s(t))·cos(2πF2t)
            // Produces: 0.5·E1·E2·(1+m·s(t))·[cos(2π(F1-F2)t) + cos(2π(F1+F2)t)]
            // The difference term (F1-F2 = 123 GHz) carries the modulation
            
            // We represent the mixed envelope as a complex value
            // Amplitude represents the envelope of the 123 GHz difference frequency
            let mixed_amplitude = pump_envelope * data_envelope;
            
            // Phase noise affects both amplitude (through power fluctuation) and phase
            let amplitude_noise = 1.0 + phase_noise * 0.01; // Small amplitude jitter
            
            let combined = Complex::from_polar(
                mixed_amplitude * amplitude_noise,
                phase_noise
            );
            
            output.push(combined);
        }
        
        output
    }
    
    /// Simulate non-linear mixing (demodulation in neural tissue)
    /// Implements third-order intermodulation: E1^2 * E2
    /// 
    /// The biological tissue acts as a non-linear mixer that performs:
    /// E_total^3 ≈ E1^2 × E2 (third-order term)
    /// 
    /// This extracts the modulation envelope from the 123 GHz difference frequency,
    /// producing the audible 12 kHz signal through biological envelope detection.
    /// 
    /// If external audio is provided, also models secondary intermodulation
    /// between the AID signal and acoustic input at the cochlear nerve junction.
    /// 
    /// If bypass_simulation is enabled, returns the real part of the input signal (for validation).
    pub fn nonlinear_mixing(&self, signal: &[Complex<f32>]) -> Vec<f32> {
        // Bypass: extract real part only, skip all mixing simulation
        if self.config.bypass_simulation {
            return signal.iter().map(|c| c.re).collect();
        }
        
        let mut output = Vec::with_capacity(signal.len());
        
        for &sample in signal {
            let magnitude = sample.norm();
            
            // Third-order intermodulation: |E|² × Re(E)
            // This is the dominant term in E1² × E2 mixing
            // |E|² represents the power (envelope squared)
            // Re(E) provides the phase information
            // Together they extract the modulation
            let third_order_product = magnitude * magnitude * sample.re;
            
            // Apply mixing coefficient (represents biological response efficiency)
            // This accounts for tissue properties, neural sensitivity, etc.
            let demodulated = self.config.mixing_coefficient * third_order_product;
            
            output.push(demodulated);
        }
        
        // Apply simple DC blocking to center around zero (biological AC coupling)
        let mean: f32 = output.iter().sum::<f32>() / output.len() as f32;
        output.iter_mut().for_each(|x| *x -= mean);
        
        // Normalize to maintain signal amplitude
        let max_abs = output.iter()
            .map(|&x| x.abs())
            .fold(0.0f32, f32::max);
        
        // Scale to preserve typical audio range (-1.0 to 1.0)
        // Targeting ~0.5 peak for headroom
        if max_abs > 1e-6 {
            let scale = 0.5 / max_abs;
            output.iter_mut().for_each(|x| *x *= scale);
        }
        
        // Apply secondary intermodulation if external audio is present
        if let (Some(ext_audio), Some(config)) = (&self.external_audio, &self.mixing_config) {
            self.apply_biological_intermodulation(&mut output, ext_audio, config);
        }
        
        output
    }
    
    /// Apply biological intermodulation between AID signal and external audio
    /// 
    /// Models the secondary mixing that occurs at:
    /// 1. Cochlear nerve junction (second-order products)
    /// 2. Auditory cortex (third-order products and parametric effects)
    /// 3. Cortical integration (perceptual blending)
    fn apply_biological_intermodulation(
        &self,
        aid_signal: &mut [f32],
        external_audio: &[f32],
        config: &AudioMixingConfig,
    ) {
        let min_len = aid_signal.len().min(external_audio.len());
        
        for i in 0..min_len {
            let aid = aid_signal[i] * config.aid_signal_gain;
            let ext = external_audio[i] * config.external_audio_gain;
            
            // Second-order intermodulation (cochlear nerve junction)
            // Produces sum and difference frequencies: f_aid ± f_ext
            let second_order = if config.enable_second_order {
                config.second_order_coefficient * aid * ext
            } else {
                0.0
            };
            
            // Third-order intermodulation (cortical processing)
            // Produces products like: 2*f_aid ± f_ext, f_aid ± 2*f_ext
            let third_order = if config.enable_third_order {
                // Model as: aid² * ext + aid * ext²
                config.third_order_coefficient * (aid * aid * ext + aid * ext * ext)
            } else {
                0.0
            };
            
            // Cortical integration - perceptual blending
            // This represents the brain's non-linear integration of the two signals
            let cortical_blend = config.cortical_coefficient * (aid + ext);
            
            // Combine all components
            // The direct signals are attenuated because they partially convert to intermod products
            let direct_attenuation = 1.0 - (config.second_order_coefficient + 
                                           config.third_order_coefficient + 
                                           config.cortical_coefficient) * 0.3;
            
            aid_signal[i] = direct_attenuation * aid + 
                           second_order + 
                           third_order + 
                           cortical_blend;
        }
        
        // Normalize to prevent clipping
        let max_abs = aid_signal.iter()
            .map(|&x| x.abs())
            .fold(0.0f32, f32::max);
        
        if max_abs > 1.0 {
            let scale = 0.95 / max_abs;
            aid_signal.iter_mut().for_each(|x| *x *= scale);
        }
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
    
    #[test]
    fn test_bypass_simulation() {
        let mut config = ThzCarrierConfig::default();
        config.bypass_simulation = true;
        let mut processor = ThzCarrierProcessor::new(config, 48000.0);
        
        // Test input signal
        let audio = vec![0.5, -0.3, 0.8, -0.1, 0.0];
        
        // When bypassed, modulate_data_carrier should return input as complex
        let modulated = processor.modulate_data_carrier(&audio);
        assert_eq!(modulated.len(), audio.len());
        for (i, &sample) in audio.iter().enumerate() {
            assert_eq!(modulated[i].re, sample);
            assert_eq!(modulated[i].im, 0.0);
        }
        
        // When bypassed, nonlinear_mixing should extract real part only
        let demodulated = processor.nonlinear_mixing(&modulated);
        assert_eq!(demodulated.len(), audio.len());
        for (i, &expected) in audio.iter().enumerate() {
            assert_eq!(demodulated[i], expected);
        }
    }
    
    #[test]
    fn test_bypass_vs_normal_simulation() {
        let audio = vec![0.5, -0.3, 0.8, -0.1, 0.0];
        
        // Normal simulation
        let mut normal_config = ThzCarrierConfig::default();
        normal_config.bypass_simulation = false;
        let mut normal_processor = ThzCarrierProcessor::new(normal_config, 48000.0);
        let normal_modulated = normal_processor.modulate_data_carrier(&audio);
        let normal_output = normal_processor.nonlinear_mixing(&normal_modulated);
        
        // Bypassed simulation
        let mut bypass_config = ThzCarrierConfig::default();
        bypass_config.bypass_simulation = true;
        let mut bypass_processor = ThzCarrierProcessor::new(bypass_config, 48000.0);
        let bypass_modulated = bypass_processor.modulate_data_carrier(&audio);
        let bypass_output = bypass_processor.nonlinear_mixing(&bypass_modulated);
        
        // Bypass should return input unchanged
        assert_eq!(bypass_output, audio);
        
        // Normal simulation should be different from input (due to mixing effects)
        // Allow for some tolerance due to normalization
        let mut different = false;
        for (i, &sample) in audio.iter().enumerate() {
            if (normal_output[i] - sample).abs() > 0.01 {
                different = true;
                break;
            }
        }
        assert!(different, "Normal simulation should differ from input signal");
    }
}
