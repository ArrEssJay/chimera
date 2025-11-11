//! Integration Tests
//!
//! End-to-end tests for complete transmission chains

use chimera_core::{
    config::{SimulationConfig, ProtocolConfig, LDPCConfig},
    pipeline::RealtimePipeline,
};

#[test]
fn test_complete_transmission_chain() {
    // Test full encode → modulate → channel → demodulate → decode path
    let mut sim = SimulationConfig::default();
    sim.snr_db = 20.0; // Clean channel for bit-exact recovery
    sim.link_loss_db = 0.0;
    
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_message = b"The quick brown fox jumps over the lazy dog";
    
    println!("Complete Transmission Chain Test:");
    println!("  Test message: {:?}", String::from_utf8_lossy(test_message));
    
    // Process enough chunks to encode and decode the message
    let mut decoded_bytes = Vec::new();
    
    for i in 0..50 {
        let output = pipeline.process_chunk(test_message);
        
        if !output.decoded_data.is_empty() {
            decoded_bytes.extend_from_slice(&output.decoded_data);
            println!("  Chunk {}: decoded {} bytes", i, output.decoded_data.len());
        }
        
        if output.post_channel.ber_average < 0.001 {
            println!("  Chunk {}: BER = {:.6}", i, output.post_channel.ber_average);
        }
    }
    
    println!("  Total decoded bytes: {}", decoded_bytes.len());
    
    // In clean channel, should successfully decode data
    assert!(
        !decoded_bytes.is_empty(),
        "Should decode data in clean channel"
    );
}

#[test]
fn test_ber_vs_snr_curve() {
    // Generate BER curve across SNR range
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    println!("BER vs SNR Curve Test:");
    println!("  SNR (dB) | BER");
    println!("  ---------|----------");
    
    let snr_values = [0.0, 5.0, 10.0, 15.0, 20.0];
    let mut ber_values = Vec::new();
    
    for &snr_db in &snr_values {
        let mut sim = SimulationConfig::default();
        sim.snr_db = snr_db;
        sim.link_loss_db = 0.0;
        
        let mut pipeline = RealtimePipeline::new(sim, protocol.clone(), ldpc.clone());
        pipeline.set_modulation_mode(true);
        
        let test_data = b"BER test message for SNR measurement";
        
        // Process multiple chunks to accumulate BER statistics
        let mut ber_sum = 0.0;
        let mut ber_count = 0;
        
        for _ in 0..20 {
            let output = pipeline.process_chunk(test_data);
            
            if output.post_channel.ber_average > 0.0 {
                ber_sum += output.post_channel.ber_average;
                ber_count += 1;
            }
        }
        
        let avg_ber = if ber_count > 0 {
            ber_sum / ber_count as f32
        } else {
            0.0
        };
        
        ber_values.push(avg_ber);
        
        println!("  {:8.1} | {:.6}", snr_db, avg_ber);
    }
    
    // BER should decrease as SNR increases
    for i in 1..ber_values.len() {
        assert!(
            ber_values[i] <= ber_values[i-1] || ber_values[i] < 0.01,
            "BER should decrease with increasing SNR"
        );
    }
    
    // At high SNR (20 dB), BER should be very low
    assert!(
        ber_values[ber_values.len()-1] < 0.01,
        "BER should be low at high SNR"
    );
}

#[test]
fn test_graceful_degradation_with_noise() {
    // Test system behavior as channel conditions worsen
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    println!("Graceful Degradation Test:");
    
    let snr_values = [20.0, 15.0, 10.0, 5.0, 0.0];
    
    for &snr_db in &snr_values {
        let mut sim = SimulationConfig::default();
        sim.snr_db = snr_db;
        
        let mut pipeline = RealtimePipeline::new(sim, protocol.clone(), ldpc.clone());
        pipeline.set_modulation_mode(true);
        
        let test_data = b"Degradation test";
        
        // Process a few chunks
        let mut evm_sum = 0.0;
        let mut snr_sum = 0.0;
        let mut count = 0;
        
        for _ in 0..10 {
            let output = pipeline.process_chunk(test_data);
            
            evm_sum += output.post_channel.evm_percent;
            snr_sum += output.post_channel.snr_estimate_db;
            count += 1;
        }
        
        let avg_evm = evm_sum / count as f32;
        let avg_snr = snr_sum / count as f32;
        
        println!("  SNR {:.1} dB: EVM = {:.1}%, SNR estimate = {:.1} dB",
            snr_db, avg_evm, avg_snr);
        
        // System should remain stable (no crashes or NaN values)
        assert!(avg_evm.is_finite(), "EVM should be valid at SNR {} dB", snr_db);
        assert!(avg_snr.is_finite(), "SNR estimate should be valid at SNR {} dB", snr_db);
    }
}

#[test]
fn test_long_running_stability() {
    // Test pipeline stability over extended operation
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Stability test";
    
    println!("Long Running Stability Test:");
    
    // Process 1000 chunks
    let mut max_evm: f32 = 0.0;
    let mut min_snr = f32::INFINITY;
    
    for i in 0..1000 {
        let output = pipeline.process_chunk(test_data);
        
        max_evm = max_evm.max(output.post_channel.evm_percent);
        min_snr = min_snr.min(output.post_channel.snr_estimate_db);
        
        if i % 100 == 0 {
            println!("  Chunk {}: EVM={:.1}%, SNR={:.1} dB, frames={}",
                i,
                output.post_channel.evm_percent,
                output.post_channel.snr_estimate_db,
                output.frames_processed
            );
        }
        
        // Check for NaN or infinite values
        assert!(
            output.post_channel.evm_percent.is_finite(),
            "EVM became invalid at chunk {}", i
        );
        
        assert!(
            output.post_channel.snr_estimate_db.is_finite(),
            "SNR became invalid at chunk {}", i
        );
    }
    
    println!("  Max EVM over 1000 chunks: {:.1}%", max_evm);
    println!("  Min SNR over 1000 chunks: {:.1} dB", min_snr);
    
    // System should remain stable
    assert!(max_evm < 100.0, "EVM should remain reasonable");
    assert!(min_snr > -10.0, "SNR should remain reasonable");
}

#[test]
fn test_mode_switching_stability() {
    // Test switching between idle and active modes
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    
    let test_data = b"Mode switch test";
    
    println!("Mode Switching Stability Test:");
    
    for i in 0..20 {
        // Alternate between idle and active
        let is_active = i % 2 == 0;
        pipeline.set_modulation_mode(is_active);
        
        let output = pipeline.process_chunk(test_data);
        
        println!("  Chunk {}: mode={}, samples={}",
            i,
            if is_active { "active" } else { "idle" },
            output.audio_samples.len()
        );
        
        // Should generate samples in both modes
        assert!(
            !output.audio_samples.is_empty(),
            "Should generate audio in both modes"
        );
        
        // Diagnostics should be valid
        assert!(
            output.post_channel.evm_percent.is_finite(),
            "EVM should be valid after mode switch"
        );
    }
}
