//! Diagnostic and Telemetry Tests
//!
//! Tests for constellation diagrams, spectrum analyzers, EVM, and SNR estimation

use chimera_core::{
    config::{SimulationConfig, ProtocolConfig, LDPCConfig},
    pipeline::RealtimePipeline,
    signal_processing::{
        modulation::symbols_to_carrier_signal,
        demodulation::audio_to_symbols,
    },
    diagnostics::constellation::normalize_constellation,
};
use crate::fixtures;
use num_complex::Complex;

#[test]
fn test_constellation_diagram_accuracy() {
    // Generate known symbols and verify constellation extraction
    let symbols = fixtures::generate_test_symbols(fixtures::SymbolPattern::AllFourPhases, 100);
    let mod_config = fixtures::get_test_modulation_config(true, false);
    
    let audio = symbols_to_carrier_signal(&symbols, &mod_config);
    let demod_config = fixtures::get_test_demodulation_config();
    let recovered = audio_to_symbols(&audio, &demod_config);
    
    let (i_vals, q_vals) = normalize_constellation(&recovered);
    
    println!("Constellation Diagram Accuracy Test:");
    println!("  Recovered symbols: {}", recovered.len());
    println!("  I values: {}", i_vals.len());
    println!("  Q values: {}", q_vals.len());
    
    assert_eq!(i_vals.len(), q_vals.len(), "I and Q should have same length");
    assert_eq!(i_vals.len(), recovered.len(), "Normalized constellation should match symbol count");
    
    // Check that first few symbols are reasonable
    for i in 10..20.min(i_vals.len()) {
        let magnitude = (i_vals[i].powi(2) + q_vals[i].powi(2)).sqrt();
        println!("  Symbol {}: I={:.2}, Q={:.2}, mag={:.2}", i, i_vals[i], q_vals[i], magnitude);
        
        assert!(
            magnitude > 0.1 && magnitude < 5.0,
            "Constellation point {} out of range", i
        );
    }
}

#[test]
fn test_evm_calculation() {
    // Test EVM computation with known distortion
    let mut sim = SimulationConfig::default();
    sim.snr_db = 15.0;
    
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"EVM test message";
    
    println!("EVM Calculation Test:");
    
    // Process a few chunks and collect EVM values
    let mut evm_values = Vec::new();
    
    for i in 0..10 {
        let output = pipeline.process_chunk(test_data);
        
        if output.post_channel.evm_percent.is_finite() && output.post_channel.evm_percent > 0.0 {
            evm_values.push(output.post_channel.evm_percent);
            
            if i < 5 {
                println!("  Chunk {}: EVM = {:.1}%", i, output.post_channel.evm_percent);
            }
        }
    }
    
    assert!(!evm_values.is_empty(), "Should compute EVM values");
    
    let avg_evm = evm_values.iter().sum::<f32>() / evm_values.len() as f32;
    
    println!("  Average EVM: {:.1}%", avg_evm);
    
    // At 15 dB SNR, EVM should be measurable but reasonable
    assert!(
        avg_evm > 5.0 && avg_evm < 50.0,
        "EVM out of expected range: {:.1}%", avg_evm
    );
}

#[test]
fn test_snr_estimation() {
    // Test SNR estimator accuracy
    println!("SNR Estimation Test:");
    
    let target_snrs = [5.0, 10.0, 15.0, 20.0];
    
    for &target_snr_db in &target_snrs {
        let mut sim = SimulationConfig::default();
        sim.snr_db = target_snr_db;
        sim.link_loss_db = 0.0;
        
        let protocol = ProtocolConfig::default();
        let ldpc = LDPCConfig::default();
        
        let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
        pipeline.set_modulation_mode(true);
        
        let test_data = b"SNR estimation test";
        
        // Process multiple chunks to get stable estimate
        let mut snr_estimates = Vec::new();
        
        for _ in 0..20 {
            let output = pipeline.process_chunk(test_data);
            
            if output.post_channel.snr_estimate_db.is_finite() {
                snr_estimates.push(output.post_channel.snr_estimate_db);
            }
        }
        
        if !snr_estimates.is_empty() {
            let avg_estimate = snr_estimates.iter().sum::<f32>() / snr_estimates.len() as f32;
            let error = (avg_estimate - target_snr_db as f32).abs();
            
            println!("  Target {:.1} dB: estimated {:.1} dB, error {:.1} dB",
                target_snr_db, avg_estimate, error);
            
            // Allow ±3 dB estimation error
            assert!(
                error < 3.0,
                "SNR estimation error too large"
            );
        }
    }
}

#[test]
fn test_diagnostic_output_format() {
    // Verify diagnostic outputs are in expected format
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Format test";
    let output = pipeline.process_chunk(test_data);
    
    println!("Diagnostic Output Format Test:");
    
    // Check pre-channel diagnostics
    println!("  TX constellation: {} points", output.pre_channel.tx_constellation_i.len());
    println!("  TX spectrum: {} bins", output.pre_channel.tx_spectrum_magnitude.len());
    
    assert_eq!(
        output.pre_channel.tx_constellation_i.len(),
        output.pre_channel.tx_constellation_q.len(),
        "Constellation I and Q should have same length"
    );
    
    // Check post-channel diagnostics
    println!("  RX constellation: {} points", output.post_channel.rx_constellation_i.len());
    println!("  RX spectrum: {} bins", output.post_channel.rx_spectrum_magnitude.len());
    
    assert_eq!(
        output.post_channel.rx_constellation_i.len(),
        output.post_channel.rx_constellation_q.len(),
        "RX constellation I and Q should have same length"
    );
    
    // Check metrics are valid numbers
    assert!(output.post_channel.evm_percent.is_finite(), "EVM should be finite");
    assert!(output.post_channel.snr_estimate_db.is_finite(), "SNR should be finite");
    assert!(output.post_channel.ber_instantaneous >= 0.0, "BER should be non-negative");
}

#[test]
fn test_frame_layout_info() {
    // Verify frame layout information is accurate
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Frame layout test";
    let output = pipeline.process_chunk(test_data);
    
    println!("Frame Layout Info Test:");
    println!("  Sync bytes: {}", output.pre_channel.frame_layout.sync_bytes);
    println!("  Data bytes: {}", output.pre_channel.frame_layout.data_bytes);
    println!("  Parity bytes: {}", output.pre_channel.frame_layout.parity_bytes);
    println!("  Total bytes: {}", output.pre_channel.frame_layout.total_bytes);
    
    // Verify consistency
    let computed_total = output.pre_channel.frame_layout.sync_bytes
        + output.pre_channel.frame_layout.target_id_bytes
        + output.pre_channel.frame_layout.command_type_bytes
        + output.pre_channel.frame_layout.data_bytes
        + output.pre_channel.frame_layout.parity_bytes;
    
    assert_eq!(
        computed_total,
        output.pre_channel.frame_layout.total_bytes,
        "Frame layout totals don't match"
    );
}

#[test]
fn test_fsk_state_reporting() {
    // Verify FSK state information is reported correctly
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    let mut protocol = ProtocolConfig::default();
    protocol.enable_fsk = true; // Ensure FSK is enabled
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"FSK state test";
    
    for i in 0..5 {
        let output = pipeline.process_chunk(test_data);
        
        if let Some(ref fsk_state) = output.fsk_state {
            println!("Chunk {}: FSK freq={} Hz, bit={}, bit_index={}",
                i,
                fsk_state.current_frequency_hz,
                fsk_state.current_bit,
                fsk_state.bit_index
            );
            
            // Frequency should be 12000 ± 2 Hz (allowing for estimation variance)
            assert!(
                (fsk_state.current_frequency_hz - 12000.0).abs() <= 2.0,
                "FSK frequency out of range"
            );
            
            // Bit should be 0 or 1
            assert!(
                fsk_state.current_bit == 0 || fsk_state.current_bit == 1,
                "FSK bit should be 0 or 1"
            );
        }
    }
}
