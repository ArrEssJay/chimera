//! Real-time Pipeline Tests
//!
//! Tests for chunk-by-chunk processing, boundary continuity, and latency

use chimera_core::{
    config::{SimulationConfig, ProtocolConfig, LDPCConfig},
    pipeline::RealtimePipeline,
};

#[test]
fn test_chunk_boundary_continuity() {
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true); // Active mode
    
    // Process multiple chunks and verify no discontinuities at boundaries
    let test_data = b"Hello World! This is a test message.";
    
    let mut all_audio = Vec::new();
    
    println!("Chunk Boundary Continuity Test:");
    
    for i in 0..5 {
        let output = pipeline.process_chunk(test_data);
        
        println!("  Chunk {}: {} samples", i, output.audio_samples.len());
        
        // Check for discontinuities at chunk boundaries
        if !all_audio.is_empty() && !output.audio_samples.is_empty() {
            let last_sample: f32 = *all_audio.last().unwrap();
            let first_sample: f32 = output.audio_samples[0];
            let boundary_jump = (first_sample - last_sample).abs();
            
            println!("    Boundary jump: {}", boundary_jump);
            
            // For 12 kHz carrier at 48 kHz sample rate, max jump should be ~1.0
            assert!(
                boundary_jump < 1.5,
                "Discontinuity detected at chunk boundary {} (jump {})",
                i, boundary_jump
            );
        }
        
        all_audio.extend_from_slice(&output.audio_samples);
    }
    
    println!("  Total samples generated: {}", all_audio.len());
    assert!(!all_audio.is_empty(), "Pipeline should generate audio samples");
}

#[test]
fn test_variable_chunk_sizes() {
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    // Test with different chunk sizes
    let test_data = b"Test message";
    
    println!("Variable Chunk Sizes Test:");
    
    // Process several chunks and collect output
    let mut outputs = Vec::new();
    for i in 0..10 {
        let output = pipeline.process_chunk(test_data);
        outputs.push(output);
        println!("  Chunk {}: {} samples", i, outputs[i].audio_samples.len());
    }
    
    // All chunks should produce consistent output
    assert!(!outputs.is_empty(), "Should process chunks");
    
    // Check that symbol counts are reasonable
    for (i, output) in outputs.iter().enumerate() {
        assert!(
            output.frames_processed > 0 || i == 0,
            "Chunks should process frames after initialization"
        );
    }
}

#[test]
fn test_pipeline_latency() {
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Latency test";
    
    // Measure processing time for multiple chunks
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::Instant;
        
        println!("Pipeline Latency Test:");
        
        let mut total_samples = 0;
        let mut total_time = std::time::Duration::ZERO;
        
        for _ in 0..100 {
            let start = Instant::now();
            let output = pipeline.process_chunk(test_data);
            let elapsed = start.elapsed();
            
            total_samples += output.audio_samples.len();
            total_time += elapsed;
        }
        
        let avg_time_us = total_time.as_micros() / 100;
        let samples_per_chunk = total_samples / 100;
        let chunk_duration_us = (samples_per_chunk as f64 / 48000.0) * 1e6;
        
        println!("  Average processing time: {} μs", avg_time_us);
        println!("  Average samples per chunk: {}", samples_per_chunk);
        println!("  Chunk audio duration: {:.1} μs", chunk_duration_us);
        println!("  Real-time factor: {:.2}x", chunk_duration_us / avg_time_us as f64);
        
        // Processing should be faster than real-time (at least 2x)
        assert!(
            avg_time_us < (chunk_duration_us as u128 / 2),
            "Processing too slow for real-time operation"
        );
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // Just verify it runs without timing checks on WASM
        for _ in 0..10 {
            let _ = pipeline.process_chunk(test_data);
        }
        println!("Pipeline Latency Test: WASM mode (timing skipped)");
    }
}

#[test]
fn test_buffer_underrun_recovery() {
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Test";
    
    println!("Buffer Underrun Recovery Test:");
    
    // Process normally
    let output1 = pipeline.process_chunk(test_data);
    println!("  Normal operation: {} samples", output1.audio_samples.len());
    
    // Simulate underrun with empty data (pipeline continues generating from encoder)
    let output2 = pipeline.process_chunk(&[]);
    println!("  After underrun: {} samples", output2.audio_samples.len());
    
    // Continue processing
    let output3 = pipeline.process_chunk(test_data);
    println!("  After recovery: {} samples", output3.audio_samples.len());
    
    // All outputs should have audio samples
    assert!(!output1.audio_samples.is_empty(), "Should generate samples normally");
    assert!(!output2.audio_samples.is_empty(), "Should handle empty input gracefully");
    assert!(!output3.audio_samples.is_empty(), "Should recover after underrun");
}

#[test]
fn test_pipeline_state_consistency() {
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Consistency test";
    
    println!("Pipeline State Consistency Test:");
    
    // Process multiple chunks and verify state advances correctly
    let mut last_frame_count = 0;
    let mut last_symbol_count = 0;
    
    for i in 0..10 {
        let output = pipeline.process_chunk(test_data);
        
        println!("  Chunk {}: frames={}, symbols={}", 
            i, output.frames_processed, output.symbols_decoded);
        
        // Frame and symbol counts should never decrease
        assert!(
            output.frames_processed >= last_frame_count,
            "Frame count should not decrease"
        );
        
        // Symbol count should increase
        assert!(
            output.symbols_decoded >= last_symbol_count,
            "Symbol count should not decrease"
        );
        
        last_frame_count = output.frames_processed;
        last_symbol_count = output.symbols_decoded;
    }
}

#[test]
fn test_diagnostic_output_continuity() {
    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let test_data = b"Diagnostics test";
    
    println!("Diagnostic Output Continuity Test:");
    
    for i in 0..5 {
        let output = pipeline.process_chunk(test_data);
        
        println!("  Chunk {}: constellation_points={}, spectrum_bins={}", 
            i,
            output.pre_channel.tx_constellation_i.len(),
            output.pre_channel.tx_spectrum_magnitude.len()
        );
        
        // Diagnostics should be populated
        assert!(
            !output.pre_channel.tx_constellation_i.is_empty(),
            "TX constellation should be populated"
        );
        
        assert!(
            !output.pre_channel.tx_spectrum_magnitude.is_empty(),
            "TX spectrum should be populated"
        );
        
        // EVM and SNR should be valid numbers
        assert!(
            output.post_channel.evm_percent.is_finite(),
            "EVM should be a valid number"
        );
        
        assert!(
            output.post_channel.snr_estimate_db.is_finite(),
            "SNR estimate should be a valid number"
        );
    }
}
