//! THz Carrier Processing Tests
//!
//! Tests for the Atmospheric Ionospheric Ducting (AID) effect simulation
//! via THz carrier modulation and third-order intermodulation

use chimera_core::{
    thz_carriers::{ThzCarrierProcessor, ThzCarrierConfig},
    signal_processing::modulation::symbols_to_carrier_signal,
};
use crate::fixtures;

#[test]
fn test_thz_modulation_depth_control() {
    let sample_rate = 48000.0;
    let mut config = ThzCarrierConfig::default();
    config.bypass_simulation = false;
    
    let mut processor = ThzCarrierProcessor::new(config, sample_rate);
    
    // Generate a test carrier signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 100);
    let mod_config = fixtures::get_test_modulation_config(false, false);
    let carrier = symbols_to_carrier_signal(&symbols, &mod_config);
    
    // Test idle mode (5% depth - default)
    processor.set_modulation_depth(0.05);
    let idle_modulated = processor.modulate_data_carrier(&carrier);
    
    // Test active mode (75% depth)
    processor.set_modulation_depth(0.75);
    let active_modulated = processor.modulate_data_carrier(&carrier);
    
    // Measure amplitude variation in complex output
    let idle_magnitudes: Vec<f32> = idle_modulated.iter().map(|c| c.norm()).collect();
    let active_magnitudes: Vec<f32> = active_modulated.iter().map(|c| c.norm()).collect();
    
    let idle_peak = idle_magnitudes.iter().cloned().fold(0.0f32, f32::max);
    let idle_min = idle_magnitudes.iter().cloned().fold(f32::INFINITY, f32::min);
    
    let active_peak = active_magnitudes.iter().cloned().fold(0.0f32, f32::max);
    let active_min = active_magnitudes.iter().cloned().fold(f32::INFINITY, f32::min);
    
    println!("THz Modulation Depth Control Test:");
    println!("  Idle mode peak: {:.3}, min: {:.3}", idle_peak, idle_min);
    println!("  Active mode peak: {:.3}, min: {:.3}", active_peak, active_min);
    
    // Active mode should have greater variation than idle mode
    let idle_variation = idle_peak / idle_min.max(0.001);
    let active_variation = active_peak / active_min.max(0.001);
    
    println!("  Idle variation ratio: {:.2}", idle_variation);
    println!("  Active variation ratio: {:.2}", active_variation);
    
    assert!(
        active_variation > idle_variation,
        "Active mode should show more amplitude variation than idle mode"
    );
}

#[test]
fn test_thz_carrier_frequency_parameters() {
    let sample_rate = 48000.0;
    let config = ThzCarrierConfig::default();
    
    let _processor = ThzCarrierProcessor::new(config.clone(), sample_rate);
    
    // THz carriers should be at specified frequencies
    println!("THz Carrier Frequency Parameters Test:");
    println!("  Pump frequency: {} Hz ({:.3} THz)", config.pump_frequency, config.pump_frequency / 1e12);
    println!("  Data frequency: {} Hz ({:.3} THz)", config.data_frequency, config.data_frequency / 1e12);
    
    assert_eq!(
        config.pump_frequency, 1.998e12,
        "Pump frequency should be 1.998 THz"
    );
    
    assert_eq!(
        config.data_frequency, 1.875e12,
        "Data frequency should be 1.875 THz"
    );
}

#[test]
fn test_thz_bypass_mode() {
    let sample_rate = 48000.0;
    
    // Create two processors - one with bypass, one without
    let mut bypass_config = ThzCarrierConfig::default();
    bypass_config.bypass_simulation = true;
    let mut bypass_processor = ThzCarrierProcessor::new(bypass_config, sample_rate);
    
    let mut active_config = ThzCarrierConfig::default();
    active_config.bypass_simulation = false;
    let mut active_processor = ThzCarrierProcessor::new(active_config, sample_rate);
    
    // Generate test signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 100);
    let mod_config = fixtures::get_test_modulation_config(false, false);
    let carrier = symbols_to_carrier_signal(&symbols, &mod_config);
    
    // Process through both
    let bypass_output = bypass_processor.modulate_data_carrier(&carrier);
    let active_output = active_processor.modulate_data_carrier(&carrier);
    
    println!("THz Bypass Mode Test:");
    println!("  Bypass output samples: {}", bypass_output.len());
    println!("  Active output samples: {}", active_output.len());
    
    // Both should produce output
    assert_eq!(bypass_output.len(), carrier.len(), "Bypass should preserve sample count");
    assert_eq!(active_output.len(), carrier.len(), "Active should preserve sample count");
    
    // In bypass mode, real part should match input
    let bypass_matches = carrier.iter().zip(bypass_output.iter())
        .all(|(input, output)| (input - output.re).abs() < 1e-5);
    
    println!("  Bypass preserves input: {}", bypass_matches);
    
    assert!(
        bypass_matches,
        "Bypass mode should pass signal through (as real part of complex)"
    );
}

#[test]
fn test_thz_modulation_produces_complex_output() {
    let sample_rate = 48000.0;
    let mut config = ThzCarrierConfig::default();
    config.bypass_simulation = false;
    config.mixing_coefficient = 0.5;
    
    let mut processor = ThzCarrierProcessor::new(config, sample_rate);
    processor.set_modulation_depth(0.5);
    
    // Generate QPSK modulated signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    let carrier = symbols_to_carrier_signal(&symbols, &mod_config);
    
    let modulated = processor.modulate_data_carrier(&carrier);
    
    println!("THz Complex Output Test:");
    println!("  Input samples: {}", carrier.len());
    println!("  Output samples: {}", modulated.len());
    
    // Check first few samples
    for (i, sample) in modulated.iter().enumerate().take(5) {
        println!("  Sample {}: re={:.3}, im={:.3}, mag={:.3}",
            i, sample.re, sample.im, sample.norm());
    }
    
    assert_eq!(modulated.len(), carrier.len(), "Sample count should match");
    
    // Output should be complex (have imaginary components from phase noise)
    let has_imaginary = modulated.iter().any(|c| c.im.abs() > 1e-6);
    
    assert!(
        has_imaginary,
        "THz modulation should produce complex output with phase information"
    );
}

#[test]
fn test_thz_mixing_coefficient_effect() {
    let sample_rate = 48000.0;
    
    // Test with different mixing coefficients
    let coefficients = [0.0, 0.3, 0.7, 1.0];
    
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 100);
    let mod_config = fixtures::get_test_modulation_config(false, false);
    let carrier = symbols_to_carrier_signal(&symbols, &mod_config);
    
    println!("THz Mixing Coefficient Effect Test:");
    
    for &coeff in &coefficients {
        let mut config = ThzCarrierConfig::default();
        config.bypass_simulation = false;
        config.mixing_coefficient = coeff;
        
        let mut processor = ThzCarrierProcessor::new(config, sample_rate);
        processor.set_modulation_depth(0.5);
        
        let modulated = processor.modulate_data_carrier(&carrier);
        
        // Measure average magnitude
        let avg_magnitude: f32 = modulated.iter()
            .map(|c| c.norm())
            .sum::<f32>() / modulated.len() as f32;
        
        println!("  Coefficient {}: avg_magnitude = {:.3}", coeff, avg_magnitude);
        
        // All should produce valid output
        assert!(
            avg_magnitude > 0.0 && avg_magnitude.is_finite(),
            "Invalid magnitude for coefficient {}", coeff
        );
    }
}
