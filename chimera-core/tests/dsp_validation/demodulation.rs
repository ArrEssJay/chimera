//! Demodulation Tests
//!
//! Tests for carrier recovery, symbol recovery, and demodulation accuracy

use chimera_core::{
    signal_processing::{
        modulation::symbols_to_carrier_signal,
        demodulation::audio_to_symbols,
    },
    channel::apply_audio_noise,
};
use crate::{signal_analysis, fixtures};
use rand::{SeedableRng, rngs::StdRng};
use num_complex::Complex64;

#[test]
fn test_carrier_recovery_clean_signal() {
    // Test carrier recovery with clean signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 50);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &mod_config);
    
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&audio, &demod_config);
    
    println!("Carrier Recovery (Clean) Test:");
    println!("  TX symbols: {}", symbols.len());
    println!("  RX symbols: {}", recovered.len());
    
    // Should recover correct number of symbols
    assert_eq!(
        recovered.len(), symbols.len(),
        "Symbol count mismatch"
    );
    
    // After lock (skip first few), symbols should have unit magnitude
    for (i, symbol) in recovered.iter().enumerate().skip(5) {
        let mag = symbol.norm();
        
        if i < 10 {
            println!("  Symbol {}: magnitude = {:.2}", i, mag);
        }
        
        assert!(
            mag > 0.5 && mag < 2.0,
            "Symbol {} magnitude out of range: {}",
            i, mag
        );
    }
}

#[test]
fn test_carrier_recovery_with_frequency_offset() {
    // Test with frequency offset
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    
    // TX at offset frequency
    let mut mod_config = fixtures::get_test_modulation_config(true, false);
    mod_config.carrier_freq = 12000.0 + 5.0; // +5 Hz offset
    
    let audio = symbols_to_carrier_signal(&symbols, &mod_config);
    
    // RX expects nominal frequency
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&audio, &demod_config);
    
    println!("Carrier Recovery (Frequency Offset) Test:");
    println!("  Frequency offset: +5 Hz");
    println!("  TX symbols: {}", symbols.len());
    println!("  RX symbols: {}", recovered.len());
    
    // Should still recover symbols (PLL should track offset)
    assert!(
        recovered.len() > 0,
        "Should recover symbols with frequency offset"
    );
    
    // Check magnitudes are reasonable
    let valid_count = recovered.iter()
        .skip(10) // Skip acquisition
        .filter(|s| s.norm() > 0.3 && s.norm() < 3.0)
        .count();
    
    let valid_ratio = valid_count as f32 / (recovered.len() - 10) as f32;
    
    println!("  Valid symbols: {:.1}%", valid_ratio * 100.0);
    
    assert!(
        valid_ratio > 0.8,
        "Too many invalid symbols with frequency offset"
    );
}

#[test]
fn test_demodulation_with_noise() {
    // Test demodulation with added noise
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    
    let clean_audio = symbols_to_carrier_signal(&symbols, &mod_config);
    
    // Add moderate noise (10 dB SNR)
    let noise_std = 0.1;
    let mut rng = StdRng::seed_from_u64(99999);
    let noisy_audio = apply_audio_noise(&clean_audio, noise_std, &mut rng);
    
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&noisy_audio, &demod_config);
    
    println!("Demodulation with Noise Test:");
    println!("  Noise std: {}", noise_std);
    println!("  TX symbols: {}", symbols.len());
    println!("  RX symbols: {}", recovered.len());
    
    // Should recover correct count
    assert_eq!(
        recovered.len(), symbols.len(),
        "Symbol count mismatch with noise"
    );
    
    // Compute EVM (skip first few for lock)
    let mut evm_sum = 0.0;
    let mut count = 0;
    
    for i in 10..symbols.len().min(recovered.len()) {
        // Find nearest ideal constellation point to recovered symbol
        let rx = recovered[i];
        let ideal_phases = [0.0, std::f64::consts::PI/2.0, std::f64::consts::PI, 3.0*std::f64::consts::PI/2.0];
        
        let nearest_ideal = ideal_phases.iter()
            .map(|&phase| Complex64::new(phase.cos(), phase.sin()))
            .min_by(|a, b| {
                let dist_a = (a - rx).norm();
                let dist_b = (b - rx).norm();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .unwrap();
        
        let error_vec = rx - nearest_ideal;
        evm_sum += error_vec.norm();
        count += 1;
    }
    
    let evm = (evm_sum / count as f64) * 100.0; // As percentage
    
    println!("  EVM: {:.1}%", evm);
    
    // With moderate noise, EVM should be measurable but not excessive
    assert!(
        evm < 50.0,
        "EVM too high with moderate noise: {:.1}%", evm
    );
}

#[test]
fn test_symbol_timing_recovery() {
    // Test timing recovery with clean signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::Alternating, 50);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &mod_config);
    
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&audio, &demod_config);
    
    println!("Symbol Timing Recovery Test:");
    println!("  TX symbols: {}", symbols.len());
    println!("  RX symbols: {}", recovered.len());
    
    // Should recover exactly the right number
    assert_eq!(
        recovered.len(), symbols.len(),
        "Timing recovery produced wrong symbol count"
    );
}

#[test]
fn test_phase_ambiguity_resolution() {
    // QPSK has 4-fold phase ambiguity - test that constellation is consistent
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &mod_config);
    
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&audio, &demod_config);
    
    println!("Phase Ambiguity Resolution Test:");
    
    // After lock, check that recovered phases are consistent
    // (may be rotated by 0°, 90°, 180°, or 270° but should stay consistent)
    let mut phase_diffs = Vec::new();
    
    for i in 11..recovered.len().min(30) {
        let phase1 = recovered[i-1].arg();
        let phase2 = recovered[i].arg();
        let mut diff = phase2 - phase1;
        
        // Normalize to [-π, π]
        while diff > std::f64::consts::PI { diff -= 2.0 * std::f64::consts::PI; }
        while diff < -std::f64::consts::PI { diff += 2.0 * std::f64::consts::PI; }
        
        phase_diffs.push(diff);
    }
    
    // Check for phase tracking (no large random jumps)
    let max_diff = phase_diffs.iter()
        .map(|d| d.abs())
        .fold(0.0f64, f64::max);
    
    println!("  Max phase jump: {:.1}°", max_diff.to_degrees());
    
    // Allow for symbol transitions (±90°) but not random jumps
    assert!(
        max_diff < 3.0,
        "Large phase jumps detected (poor phase lock)"
    );
}
