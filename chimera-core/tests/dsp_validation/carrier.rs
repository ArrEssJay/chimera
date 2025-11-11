//! Carrier Generation Tests
//!
//! Tests for pure sinusoid carrier generation and stability

use chimera_core::signal_processing::modulation::symbols_to_carrier_signal;
use crate::{signal_analysis, fixtures};

#[test]
fn test_carrier_frequency_accuracy() {
    // Generate pure carrier (QPSK and FSK disabled)
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 16);
    let config = fixtures::get_test_modulation_config(false, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    // Measure carrier frequency
    let measured_freq = signal_analysis::estimate_frequency(&audio, config.sample_rate as f32);
    let expected_freq = config.carrier_freq as f32;
    
    println!("Carrier Frequency Test:");
    println!("  Expected: {} Hz", expected_freq);
    println!("  Measured: {} Hz", measured_freq);
    println!("  Error: {} Hz", (measured_freq - expected_freq).abs());
    
    // Specification: ±0.1 Hz accuracy
    assert!(
        (measured_freq - expected_freq).abs() < 0.1,
        "Carrier frequency error exceeds ±0.1 Hz"
    );
}

#[test]
fn test_carrier_amplitude_stability() {
    // Generate pure carrier
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 100);
    let config = fixtures::get_test_modulation_config(false, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let peak_amp = signal_analysis::measure_peak_amplitude(&audio);
    let rms_amp = signal_analysis::measure_rms_amplitude(&audio);
    
    // For pure sinusoid, peak should be sqrt(2) * RMS
    let expected_ratio = 2.0f32.sqrt();
    let actual_ratio = peak_amp / rms_amp;
    
    println!("Carrier Amplitude Stability Test:");
    println!("  Peak amplitude: {}", peak_amp);
    println!("  RMS amplitude: {}", rms_amp);
    println!("  Peak/RMS ratio: {} (expected {})", actual_ratio, expected_ratio);
    
    // Normalized audio should have peak near 1.0
    assert!(
        peak_amp > 0.9 && peak_amp < 1.1,
        "Peak amplitude not near 1.0 for normalized carrier"
    );
    
    // Peak/RMS ratio should match sinusoid
    assert!(
        (actual_ratio - expected_ratio).abs() < 0.05,
        "Peak/RMS ratio does not match pure sinusoid (expected {}, got {})",
        expected_ratio, actual_ratio
    );
}

#[test]
fn test_carrier_phase_continuity() {
    // Generate pure carrier
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 50);
    let config = fixtures::get_test_modulation_config(false, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    // Check for large sample-to-sample jumps
    // For 12 kHz at 48 kHz sample rate, theoretical max is ~1.0 for normalized signal
    let threshold = 1.5f32; // Allow some margin
    let is_continuous = signal_analysis::check_phase_continuity(&audio, threshold);
    
    println!("Carrier Phase Continuity Test:");
    println!("  Threshold: {} (max sample jump)", threshold);
    println!("  Continuous: {}", is_continuous);
    
    assert!(
        is_continuous,
        "Carrier phase is not continuous (has discontinuities)"
    );
}

#[test]
fn test_carrier_thd() {
    // Generate pure carrier with many samples for good frequency resolution
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 200);
    let config = fixtures::get_test_modulation_config(false, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let thd_db = signal_analysis::compute_thd(&audio, config.carrier_freq as f32, config.sample_rate as f32);
    
    println!("Carrier THD Test:");
    println!("  THD: {} dB", thd_db);
    
    // Specification: THD < -40 dB
    assert!(
        thd_db < -40.0,
        "Carrier THD exceeds specification (expected < -40 dB, got {} dB)", thd_db
    );
}
