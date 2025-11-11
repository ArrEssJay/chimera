//! Frame Synchronization and Timing Recovery Tests
//!
//! Tests for sync acquisition, false sync rejection, and symbol timing

use chimera_core::{
    config::{SimulationConfig, ProtocolConfig, LDPCConfig},
    pipeline::RealtimePipeline,
};

#[test]
fn test_frame_sync_acquisition_time() {
    // Test how quickly decoder locks onto sync pattern
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Sync acquisition test message";
    
    println!("Frame Sync Acquisition Test:");
    
    let mut first_sync_chunk = None;
    
    for i in 0..20 {
        let output = pipeline.process_chunk(test_data);
        
        if output.post_channel.sync_status && first_sync_chunk.is_none() {
            first_sync_chunk = Some(i);
            println!("  Sync achieved at chunk: {}", i);
        }
        
        if i < 5 {
            println!("  Chunk {}: sync={}, frames={}",
                i,
                output.post_channel.sync_status,
                output.frames_processed
            );
        }
    }
    
    // Should achieve sync within reasonable time
    assert!(
        first_sync_chunk.is_some(),
        "Should achieve sync within test duration"
    );
    
    if let Some(chunk) = first_sync_chunk {
        assert!(
            chunk < 10,
            "Sync acquisition took too long (chunk {})", chunk
        );
    }
}

#[test]
fn test_sync_status_consistency() {
    // Test that sync status remains stable once acquired
    let mut sim = SimulationConfig::default();
    sim.snr_db = 20.0; // Clean channel
    sim.bypass_thz_simulation = true;
    
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Sync stability test";
    
    println!("Sync Status Consistency Test:");
    
    // Process many chunks and track sync status
    let mut sync_count = 0;
    let total_chunks = 50;
    
    for i in 0..total_chunks {
        let output = pipeline.process_chunk(test_data);
        
        if output.post_channel.sync_status {
            sync_count += 1;
        }
        
        if i % 10 == 0 {
            println!("  Chunk {}: sync={}", i, output.post_channel.sync_status);
        }
    }
    
    let sync_ratio = sync_count as f32 / total_chunks as f32;
    
    println!("  Sync ratio: {:.1}%", sync_ratio * 100.0);
    
    // In clean channel, should maintain sync most of the time
    assert!(
        sync_ratio > 0.7,
        "Sync should be stable in clean channel"
    );
}

#[test]
fn test_lock_status_reporting() {
    // Test lock status transitions through acquisition phases
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Lock status test";
    
    println!("Lock Status Reporting Test:");
    
    let mut seen_states = std::collections::HashSet::new();
    
    for i in 0..20 {
        let output = pipeline.process_chunk(test_data);
        
        seen_states.insert(output.post_channel.lock_status.clone());
        
        if i < 10 {
            println!("  Chunk {}: lock_status={}", i, output.post_channel.lock_status);
        }
    }
    
    println!("  Observed states: {:?}", seen_states);
    
    // Should see at least one state
    assert!(
        !seen_states.is_empty(),
        "Should report lock status"
    );
}

#[test]
fn test_symbol_timing_with_noise() {
    // Test timing recovery under noisy conditions
    let mut sim = SimulationConfig::default();
    sim.snr_db = 10.0; // Moderate noise
    sim.bypass_thz_simulation = true;
    
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Timing recovery under noise";
    
    println!("Symbol Timing with Noise Test:");
    
    // Process chunks and check timing error
    for i in 0..10 {
        let output = pipeline.process_chunk(test_data);
        
        if !output.post_channel.timing_error.is_empty() {
            let avg_timing_error = output.post_channel.timing_error.iter()
                .sum::<f32>() / output.post_channel.timing_error.len() as f32;
            
            println!("  Chunk {}: avg_timing_error={:.3}", i, avg_timing_error);
            
            // Timing error should converge to near zero
            if i > 5 {
                assert!(
                    avg_timing_error.abs() < 0.5,
                    "Timing error too large after convergence"
                );
            }
        }
    }
}

#[test]
fn test_frequency_offset_tracking() {
    // Test frequency offset reporting
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Frequency tracking test";
    
    println!("Frequency Offset Tracking Test:");
    
    for i in 0..10 {
        let output = pipeline.process_chunk(test_data);
        
        println!("  Chunk {}: freq_offset={:.2} Hz, phase_offset={:.3} rad",
            i,
            output.post_channel.frequency_offset_hz,
            output.post_channel.phase_offset_rad
        );
        
        // Values should be finite
        assert!(
            output.post_channel.frequency_offset_hz.is_finite(),
            "Frequency offset should be valid"
        );
        
        assert!(
            output.post_channel.phase_offset_rad.is_finite(),
            "Phase offset should be valid"
        );
    }
}
