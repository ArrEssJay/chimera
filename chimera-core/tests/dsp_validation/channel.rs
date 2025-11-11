//! Channel Impairment Tests
//!
//! Tests for noise, attenuation, and other channel effects

use chimera_core::{
    channel::apply_audio_noise,
    signal_processing::modulation::symbols_to_carrier_signal,
};
use crate::{signal_analysis, fixtures};
use rand::{SeedableRng, rngs::StdRng};

#[test]
fn test_awgn_power_accuracy() {
    // Generate clean signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 100);
    let config = fixtures::get_test_modulation_config(false, false);
    let signal = symbols_to_carrier_signal(&symbols, &config);
    
    // Measure actual signal power in the audio domain
    let signal_power = signal.iter().map(|&x| x * x).sum::<f32>() / signal.len() as f32;
    
    // Add AWGN with known noise power
    let target_snr_db = 10.0;
    let noise_power = signal_power as f64 / 10.0f64.powf(target_snr_db / 10.0);
    let noise_std = noise_power.sqrt();
    
    let mut rng = StdRng::seed_from_u64(12345);
    let noisy_signal = apply_audio_noise(&signal, noise_std, &mut rng);
    
    // Measure actual SNR
    let measured_snr = signal_analysis::compute_snr_db(&signal, &noisy_signal);
    
    println!("AWGN Power Accuracy Test:");
    println!("  Signal power: {:.4}", signal_power);
    println!("  Noise std: {:.4}", noise_std);
    println!("  Target SNR: {} dB", target_snr_db);
    println!("  Measured SNR: {} dB", measured_snr);
    println!("  Error: {} dB", (measured_snr - target_snr_db as f32).abs());
    
    // Allow ±1 dB tolerance
    assert!(
        (measured_snr - target_snr_db as f32).abs() < 1.0,
        "SNR error exceeds tolerance"
    );
}

#[test]
fn test_awgn_gaussian_distribution() {
    // Generate noise samples and verify Gaussian distribution
    let signal = vec![0.0f32; 10000];
    let noise_std = 0.1;
    
    let mut rng = StdRng::seed_from_u64(67890);
    let noisy_signal = apply_audio_noise(&signal, noise_std, &mut rng);
    
    // Compute kurtosis (should be ~0 for Gaussian)
    let kurtosis = signal_analysis::compute_kurtosis(&noisy_signal);
    
    println!("AWGN Gaussian Distribution Test:");
    println!("  Kurtosis: {} (expect ~0 for Gaussian)", kurtosis);
    
    // Allow some variation due to finite sample size
    assert!(
        kurtosis.abs() < 0.5,
        "Noise distribution not Gaussian (kurtosis {})",
        kurtosis
    );
}

#[test]
fn test_attenuation_accuracy() {
    // Generate test signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllZeros, 100);
    let config = fixtures::get_test_modulation_config(false, false);
    let signal = symbols_to_carrier_signal(&symbols, &config);
    
    let original_power = signal_analysis::measure_power_db(&signal);
    
    println!("Attenuation Accuracy Test:");
    println!("  Original power: {} dB", original_power);
    
    // Test different attenuation levels
    let attenuations = [10.0, 20.0, 30.0];
    
    for &atten_db in &attenuations {
        // Apply attenuation
        let gain = 10.0f32.powf(-atten_db / 20.0);
        let attenuated: Vec<f32> = signal.iter().map(|&x| x * gain).collect();
        
        let attenuated_power = signal_analysis::measure_power_db(&attenuated);
        let actual_atten = original_power - attenuated_power;
        
        println!("  Target {} dB: measured {} dB, error {} dB",
            atten_db, actual_atten, (actual_atten - atten_db).abs());
        
        // Should be accurate to ±0.1 dB
        assert!(
            (actual_atten - atten_db).abs() < 0.1,
            "Attenuation error exceeds tolerance"
        );
    }
}

#[test]
fn test_multipath_fading_effect() {
    // Simulate simple multipath with delayed copy
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    let config = fixtures::get_test_modulation_config(true, false);
    let signal = symbols_to_carrier_signal(&symbols, &config);
    
    // Add delayed and attenuated copy
    let delay_samples = 10;
    let echo_gain = 0.3;
    
    let mut multipath_signal = signal.clone();
    for i in delay_samples..signal.len() {
        multipath_signal[i] += signal[i - delay_samples] * echo_gain;
    }
    
    println!("Multipath Fading Test:");
    println!("  Delay: {} samples", delay_samples);
    println!("  Echo gain: {}", echo_gain);
    
    // Verify signal is modified
    let difference: f32 = signal.iter()
        .zip(multipath_signal.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f32>() / signal.len() as f32;
    
    println!("  Mean difference: {}", difference);
    
    assert!(
        difference > 0.01,
        "Multipath should modify signal"
    );
}

#[test]
fn test_frequency_selective_fading() {
    // Test with frequency-dependent attenuation
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::Random, 200);
    let config = fixtures::get_test_modulation_config(true, false);
    let signal = symbols_to_carrier_signal(&symbols, &config);
    
    // Simple lowpass filter simulation (attenuate high frequencies)
    let mut filtered = signal.clone();
    let alpha = 0.7; // Smoothing factor
    
    for i in 1..filtered.len() {
        filtered[i] = alpha * filtered[i] + (1.0 - alpha) * filtered[i-1];
    }
    
    println!("Frequency Selective Fading Test:");
    
    let (freqs, psd_orig) = signal_analysis::compute_psd(&signal, config.sample_rate as f32);
    let (_, psd_filt) = signal_analysis::compute_psd(&filtered, config.sample_rate as f32);
    
    // Find carrier bin
    let carrier_bin = freqs.iter()
        .position(|&f| (f - config.carrier_freq as f32).abs() < 10.0)
        .unwrap_or(0);
    
    println!("  Carrier bin: {}, freq: {:.1} Hz", carrier_bin, freqs[carrier_bin]);
    println!("  Original PSD at carrier: {:.6}", psd_orig[carrier_bin]);
    println!("  Filtered PSD at carrier: {:.6}", psd_filt[carrier_bin]);
    
    let power_loss = if psd_filt[carrier_bin] > 0.0 && psd_orig[carrier_bin] > 0.0 {
        10.0 * (psd_filt[carrier_bin] / psd_orig[carrier_bin]).log10()
    } else {
        -std::f32::INFINITY
    };
    
    println!("  Power loss at carrier: {:.1} dB", power_loss);
    
    assert!(
        power_loss.is_finite() && power_loss < -0.1,
        "Frequency selective fading should attenuate signal (power_loss = {:.1} dB)", power_loss
    );
}
