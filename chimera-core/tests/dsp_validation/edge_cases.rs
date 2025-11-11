//! Edge Cases and Stress Tests
//!
//! Tests for extreme conditions and boundary cases

use chimera_core::{
    config::{SimulationConfig, ProtocolConfig, LDPCConfig},
    pipeline::RealtimePipeline,
    signal_processing::{
        modulation::symbols_to_carrier_signal,
        demodulation::audio_to_symbols,
    },
};
use crate::{signal_analysis, fixtures};
use rand::{Rng, SeedableRng};

#[test]
fn test_extreme_snr_conditions() {
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    println!("Extreme SNR Conditions Test:");
    
    // Test at very low SNR (-10 dB)
    let mut sim_low = SimulationConfig::default();
    sim_low.snr_db = -10.0;
    
    let mut pipeline_low = RealtimePipeline::new(sim_low, protocol.clone(), ldpc.clone());
    pipeline_low.set_modulation_mode(true);
    
    let test_data = b"Low SNR test";
    let output_low = pipeline_low.process_chunk(test_data);
    
    println!("  SNR -10 dB: EVM={:.1}%, SNR_est={:.1} dB",
        output_low.post_channel.evm_percent,
        output_low.post_channel.snr_estimate_db
    );
    
    assert!(
        output_low.post_channel.evm_percent.is_finite(),
        "Should handle very low SNR without crashing"
    );
    
    // Test at very high SNR (30 dB)
    let mut sim_high = SimulationConfig::default();
    sim_high.snr_db = 30.0;
    
    let mut pipeline_high = RealtimePipeline::new(sim_high, protocol.clone(), ldpc.clone());
    pipeline_high.set_modulation_mode(true);
    
    let output_high = pipeline_high.process_chunk(test_data);
    
    println!("  SNR 30 dB: EVM={:.1}%, SNR_est={:.1} dB",
        output_high.post_channel.evm_percent,
        output_high.post_channel.snr_estimate_db
    );
    
    assert!(
        output_high.post_channel.evm_percent < 10.0,
        "High SNR should have low EVM"
    );
}

#[test]
fn test_extreme_frequency_offsets() {
    // Test demodulator with large frequency offsets
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    
    println!("Extreme Frequency Offset Test:");
    
    let offsets = [0.0, 10.0, 50.0, 100.0];
    
    for &offset_hz in &offsets {
        let mut mod_config = fixtures::get_test_modulation_config(true, false);
        mod_config.carrier_freq = 12000.0 + offset_hz;
        
        let audio = symbols_to_carrier_signal(&symbols, &mod_config);
        
        let demod_config = fixtures::get_test_demodulation_config(); // Expects 12 kHz
        let rx_symbols = audio_to_symbols(&audio, &demod_config);
        
        println!("  Offset {} Hz: recovered {} symbols", offset_hz, rx_symbols.len());
        
        assert!(
            !rx_symbols.is_empty(),
            "Should recover symbols with {} Hz offset", offset_hz
        );
    }
}

#[test]
fn test_dc_offset_rejection() {
    // Add DC bias to signal and test demodulator
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 50);
    let config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    println!("DC Offset Rejection Test:");
    
    let dc_offsets = [0.0, 0.1, 0.3, 0.5];
    
    for &dc_offset in &dc_offsets {
        // Add DC bias
        let biased_audio: Vec<f32> = audio.iter().map(|&x| x + dc_offset).collect();
        
        let demod_config = fixtures::get_test_demodulation_config();
        let rx_symbols = audio_to_symbols(&biased_audio, &demod_config);
        
        println!("  DC offset {}: recovered {} symbols", dc_offset, rx_symbols.len());
        
        assert!(
            !rx_symbols.is_empty(),
            "Should handle DC offset of {}", dc_offset
        );
    }
}

#[test]
fn test_phase_noise_tolerance() {
    // Add phase noise to carrier
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 50);
    let config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    println!("Phase Noise Tolerance Test:");
    
    // Add random phase jitter
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let phase_noise_std = 0.1; // radians
    
    let mut noisy_audio = audio.clone();
    let mut phase_accumulator = 0.0f32;
    
    for sample in noisy_audio.iter_mut() {
        phase_accumulator += rng.gen::<f32>() * phase_noise_std - phase_noise_std / 2.0;
        *sample *= phase_accumulator.cos();
    }
    
    let demod_config = fixtures::get_test_demodulation_config();
    let rx_symbols = audio_to_symbols(&noisy_audio, &demod_config);
    
    println!("  Phase noise std: {} rad", phase_noise_std);
    println!("  Recovered {} symbols", rx_symbols.len());
    
    assert!(
        !rx_symbols.is_empty(),
        "Should handle phase noise"
    );
}

#[test]
fn test_clipping_recovery() {
    // Test behavior with clipped signal
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 50);
    let config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    println!("Clipping Recovery Test:");
    
    let clip_levels = [0.5, 0.7, 0.9];
    
    for &clip_level in &clip_levels {
        // Apply hard clipping
        let clipped_audio: Vec<f32> = audio.iter()
            .map(|&x| x.clamp(-clip_level, clip_level))
            .collect();
        
        let demod_config = fixtures::get_test_demodulation_config();
        let rx_symbols = audio_to_symbols(&clipped_audio, &demod_config);
        
        println!("  Clip level {}: recovered {} symbols", clip_level, rx_symbols.len());
        
        assert!(
            !rx_symbols.is_empty(),
            "Should recover from clipping at level {}", clip_level
        );
    }
}

#[test]
fn test_very_long_signal() {
    // Test with extended signal duration
    let symbols = fixtures::generate_test_symbols(
        fixtures::SymbolPattern::AllFourPhases, 
        1000 // Large number of symbols
    );
    let config = fixtures::get_test_modulation_config(true, true);
    
    println!("Very Long Signal Test:");
    println!("  Generating {} symbols...", symbols.len());
    
    let audio = symbols_to_carrier_signal(&symbols, &config);
    
    println!("  Generated {} audio samples", audio.len());
    println!("  Duration: {:.1} seconds", audio.len() as f32 / config.sample_rate as f32);
    
    // Verify signal properties
    let freq = signal_analysis::estimate_frequency(&audio, config.sample_rate as f32);
    let power = signal_analysis::measure_power_db(&audio);
    
    println!("  Measured frequency: {} Hz", freq);
    println!("  Measured power: {} dB", power);
    
    assert!(
        (freq - config.carrier_freq as f32).abs() < 1.0,
        "Frequency should remain stable over long duration"
    );
    
    assert!(
        power > -10.0,
        "Power should remain consistent"
    );
}

#[test]
fn test_empty_input_handling() {
    // Test with empty or minimal input
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    println!("Empty Input Handling Test:");
    
    // Empty input
    let output1 = pipeline.process_chunk(&[]);
    println!("  Empty input: {} samples", output1.audio_samples.len());
    
    assert!(
        !output1.audio_samples.is_empty(),
        "Should generate audio even with empty input"
    );
    
    // Single byte
    let output2 = pipeline.process_chunk(&[0x42]);
    println!("  Single byte: {} samples", output2.audio_samples.len());
    
    assert!(
        !output2.audio_samples.is_empty(),
        "Should handle single byte input"
    );
}

#[test]
fn test_rapid_reconfiguration() {
    // Test rapid parameter changes
    let mut sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim.clone(), protocol.clone(), ldpc.clone());
    
    println!("Rapid Reconfiguration Test:");
    
    let test_data = b"Reconfig test";
    
    for i in 0..10 {
        // Change SNR rapidly
        sim.snr_db = 5.0 + (i as f64) * 2.0;
        pipeline.update_channel_params(sim.snr_db, sim.link_loss_db);
        
        let output = pipeline.process_chunk(test_data);
        
        println!("  Config {}: SNR={:.1} dB, samples={}",
            i, sim.snr_db, output.audio_samples.len());
        
        assert!(
            !output.audio_samples.is_empty(),
            "Should handle rapid reconfiguration"
        );
    }
}
