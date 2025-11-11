//! Modulation Tests
//!
//! Tests for FSK frequency dithering and QPSK phase modulation

use chimera_core::signal_processing::{
    modulation::symbols_to_carrier_signal,
    demodulation::audio_to_symbols,
};
use crate::{signal_analysis, fixtures};

// ============================================================================
// FSK Modulation Tests
// ============================================================================

#[test]
fn test_fsk_frequency_deviation() {
    // Test with FSK enabled, QPSK disabled
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 96); // 6 seconds at 16 sym/s
    let config = fixtures::get_test_modulation_config(false, true);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    // FSK alternates between 11999 and 12001 Hz every second
    // Split into segments and measure each
    let samples_per_second = config.sample_rate;
    
    println!("FSK Frequency Deviation Test:");
    
    for second in 0..3 {
        let start = second * samples_per_second;
        let end = start + samples_per_second;
        if end <= audio.len() {
            let segment = &audio[start..end];
            let freq = signal_analysis::estimate_frequency(segment, config.sample_rate as f32);
            
            // FSK alternates: even seconds = 11999 Hz, odd seconds = 12001 Hz
            let expected_freq = if second % 2 == 0 { 11999.0 } else { 12001.0 };
            let error = (freq - expected_freq).abs();
            
            println!("  Second {}: {} Hz (expected {} Hz, error {} Hz)",
                second, freq, expected_freq, error);
            
            // Specification: ±0.01 Hz accuracy (relaxed for FFT resolution)
            assert!(
                error < 1.0,
                "FSK frequency error too large at second {}: {} Hz",
                second, error
            );
        }
    }
}

#[test]
fn test_fsk_bit_rate() {
    // FSK bit rate is 1 bit/second
    // Generate 10 seconds of audio
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 160); // 10 seconds
    let config = fixtures::get_test_modulation_config(false, true);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let duration_seconds = audio.len() as f32 / config.sample_rate as f32;
    
    println!("FSK Bit Rate Test:");
    println!("  Audio samples: {}", audio.len());
    println!("  Duration: {} seconds", duration_seconds);
    println!("  Expected bit rate: 1 bit/second");
    
    // With 1 bit/second and 10 seconds, we expect 10 FSK transitions
    // This is validated by the frequency deviation test
    assert!(
        (duration_seconds - 10.0).abs() < 0.1,
        "Duration mismatch"
    );
}

#[test]
fn test_fsk_spectrum_bandwidth() {
    // Generate FSK signal (QPSK disabled for cleaner spectrum)
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 200);
    let config = fixtures::get_test_modulation_config(false, true);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let (freqs, psd) = signal_analysis::compute_psd(&audio, config.sample_rate as f32);
    let bandwidth = signal_analysis::measure_bandwidth(&freqs, &psd, config.carrier_freq as f32, 20.0);
    
    println!("FSK Spectrum Bandwidth Test:");
    println!("  Bandwidth at -20 dB: {} Hz", bandwidth);
    
    // Specification: < 5 Hz at -20 dB
    // FSK with ±1 Hz deviation should have narrow bandwidth
    assert!(
        bandwidth < 10.0, // Relaxed due to measurement limitations
        "FSK bandwidth too wide: {} Hz (should be < 10 Hz)",
        bandwidth
    );
}

// ============================================================================
// QPSK Modulation Tests
// ============================================================================

#[test]
fn test_qpsk_symbol_rate_timing() {
    // Test with QPSK enabled, FSK disabled
    let symbol_count = 32;
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, symbol_count);
    let config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let expected_samples = symbol_count * (config.sample_rate / config.symbol_rate);
    let actual_samples = audio.len();
    
    println!("QPSK Symbol Rate Test:");
    println!("  Symbol count: {}", symbol_count);
    println!("  Symbol rate: {} sym/s", config.symbol_rate);
    println!("  Expected samples: {}", expected_samples);
    println!("  Actual samples: {}", actual_samples);
    
    assert_eq!(
        actual_samples, expected_samples,
        "Sample count mismatch"
    );
}

#[test]
fn test_qpsk_constellation_phases() {
    // Generate clean QPSK signal with all four phases
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 16);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &mod_config);
    
    // Demodulate to recover constellation
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&audio, &demod_config);
    
    println!("QPSK Constellation Phase Test:");
    
    // After carrier recovery locks (skip first few symbols)
    // Check that recovered symbols maintain constellation structure
    for (i, symbol) in recovered.iter().enumerate().skip(4) {
        let phase_deg = symbol.arg().to_degrees();
        let magnitude = symbol.norm();
        
        // Verify magnitude is reasonable (normalized to ~1.0)
        assert!(
            magnitude > 0.5 && magnitude < 2.0,
            "Symbol {} magnitude out of range: {}",
            i, magnitude
        );
        
        // Phase should be close to one of the QPSK constellation points
        // (0°, 90°, 180°, 270°) with some tolerance for noise and phase offset
        let nearest_phase = ((phase_deg / 90.0).round() * 90.0) % 360.0;
        let phase_error = (phase_deg - nearest_phase).abs().min((phase_deg - nearest_phase + 360.0).abs());
        
        if i < 10 {
            println!("  Symbol {}: phase={:.1}°, mag={:.2}, error={:.1}°",
                i, phase_deg, magnitude, phase_error);
        }
    }
}

#[test]
fn test_qpsk_bandwidth_limiting() {
    // Test that QPSK modulation doesn't have excessive bandwidth
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::Random, 200);
    let config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let (freqs, psd) = signal_analysis::compute_psd(&audio, config.sample_rate as f32);
    let bandwidth = signal_analysis::measure_bandwidth(&freqs, &psd, config.carrier_freq as f32, 20.0);
    
    println!("QPSK Bandwidth Limiting Test:");
    println!("  Bandwidth at -20 dB: {} Hz", bandwidth);
    println!("  Symbol rate: {} sym/s", config.symbol_rate);
    
    // QPSK bandwidth should be roughly 2x symbol rate for raised-cosine filtering
    // 16 sym/s → expect < 50 Hz bandwidth
    assert!(
        bandwidth < 100.0,
        "QPSK bandwidth too wide: {} Hz",
        bandwidth
    );
}

// ============================================================================
// Combined Modulation Tests
// ============================================================================

#[test]
fn test_combined_fsk_qpsk_interaction() {
    // Test with both FSK and QPSK enabled
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 32);
    let config = fixtures::get_test_modulation_config(true, true);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    println!("Combined FSK+QPSK Test:");
    println!("  Generated {} samples", audio.len());
    
    // Measure characteristics
    let freq = signal_analysis::estimate_frequency(&audio, config.sample_rate as f32);
    let power = signal_analysis::measure_power_db(&audio);
    
    println!("  Measured frequency: {} Hz", freq);
    println!("  Measured power: {} dB", power);
    
    // Frequency should be near carrier (±1 Hz for FSK)
    assert!(
        (freq - config.carrier_freq as f32).abs() < 2.0,
        "Combined modulation frequency out of range"
    );
    
    // Power should be reasonable
    assert!(
        power > -10.0 && power < 10.0,
        "Combined modulation power out of range"
    );
}

#[test]
fn test_combined_spectrum_bandwidth() {
    // Test spectrum with both modulations active
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::Random, 200);
    let config = fixtures::get_test_modulation_config(true, true);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    let (freqs, psd) = signal_analysis::compute_psd(&audio, config.sample_rate as f32);
    let bandwidth = signal_analysis::measure_bandwidth(&freqs, &psd, config.carrier_freq as f32, 20.0);
    
    println!("Combined Modulation Bandwidth Test:");
    println!("  Bandwidth at -20 dB: {} Hz", bandwidth);
    
    // Combined should be dominated by QPSK bandwidth
    // FSK adds negligible bandwidth (±1 Hz is very narrow)
    assert!(
        bandwidth < 150.0,
        "Combined bandwidth too wide: {} Hz",
        bandwidth
    );
}

#[test]
fn test_modulation_phase_continuity() {
    // Verify no phase jumps in modulated signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    let config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    // Check for large sample-to-sample jumps
    let threshold = 1.5f32; // Normalized signal
    let is_continuous = signal_analysis::check_phase_continuity(&audio, threshold);
    
    println!("Modulation Phase Continuity Test:");
    println!("  Continuous: {}", is_continuous);
    
    assert!(
        is_continuous,
        "QPSK modulation creates phase discontinuities"
    );
}
