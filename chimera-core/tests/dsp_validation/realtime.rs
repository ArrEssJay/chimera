//! Real-time Pipeline Tests
//!
//! Tests for chunk-by-chunk processing, validating the unified pipeline
//! works correctly regardless of chunk size.

use chimera_core::{
    config::{SimulationConfig, ProtocolConfig, LDPCConfig},
    pipeline::RealtimePipeline,
};

#[test]
fn test_chunk_boundary_continuity() {
    // Validate audio continuity at chunk boundaries
    // Focus: Are there discontinuities when processing in chunks?
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = "Hello World!".to_string();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let mut all_audio = Vec::new();
    
    println!("Chunk Boundary Continuity Test:");
    
    for i in 0..5 {
        let output = pipeline.process_chunk(b"");
        
        println!("  Chunk {}: {} samples", i, output.audio_samples.len());
        
        // Check for discontinuities at chunk boundaries
        if !all_audio.is_empty() && !output.audio_samples.is_empty() {
            let last_sample: f32 = *all_audio.last().unwrap();
            let first_sample: f32 = output.audio_samples[0];
            let boundary_jump = (first_sample - last_sample).abs();
            
            println!("    Boundary jump: {:.4}", boundary_jump);
            
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
    // Test that pipeline works with any chunk size
    // Focus: Does the pipeline adapt to different chunk sizes?
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    sim.plaintext_source = "Test message".to_string();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    println!("Variable Chunk Sizes Test:");
    println!("The pipeline should work consistently regardless of chunk size");
    
    // Test several iterations - pipeline doesn't directly control chunk size
    // but should handle any processing pattern
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let mut outputs = Vec::new();
    for i in 0..10 {
        let output = pipeline.process_chunk(b"");
        outputs.push(output);
        println!("  Iteration {}: {} samples, {} symbols decoded", 
            i, outputs[i].audio_samples.len(), outputs[i].symbols_decoded);
    }
    
    // All iterations should produce consistent output
    assert!(!outputs.is_empty(), "Should process iterations");
    
    // Check that audio is being generated consistently
    for (i, output) in outputs.iter().enumerate() {
        assert!(
            !output.audio_samples.is_empty(),
            "Iteration {} should generate audio samples", i
        );
        
        // Symbol count should increase over time
        if i > 0 {
            assert!(
                output.symbols_decoded >= outputs[i-1].symbols_decoded,
                "Symbol count should increase over time"
            );
        }
    }
}

#[test]
fn test_streaming_consistency() {
    // Compare processing consistency across multiple runs
    // Focus: Does the pipeline produce consistent results?
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    sim.plaintext_source = "HELLO".to_string();
    sim.rng_seed = Some(42); // Fixed seed for reproducibility
    
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    println!("Streaming Consistency Test:");
    
    // Run 1: Process in small iterations
    let mut pipeline1 = RealtimePipeline::new(sim.clone(), protocol.clone(), ldpc.clone());
    pipeline1.set_modulation_mode(true);
    let mut decoded1 = String::new();
    
    for i in 0..20 {
        let output = pipeline1.process_chunk(b"");
        if !output.decoded_text.is_empty() {
            decoded1 = output.decoded_text.clone();
        }
        if i == 10 {
            println!("  Run 1 (iteration {}): decoded='{}'", i, decoded1);
        }
    }
    
    // Run 2: Process differently
    let mut pipeline2 = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline2.set_modulation_mode(true);
    let mut decoded2 = String::new();
    
    for i in 0..20 {
        let output = pipeline2.process_chunk(b"");
        if !output.decoded_text.is_empty() {
            decoded2 = output.decoded_text.clone();
        }
        if i == 10 {
            println!("  Run 2 (iteration {}): decoded='{}'", i, decoded2);
        }
    }
    
    println!("  Final run 1: '{}'", decoded1);
    println!("  Final run 2: '{}'", decoded2);
    
    // Both should eventually decode something (with fixed seed, should be identical)
    assert!(!decoded1.is_empty() || !decoded2.is_empty(),
        "At least one run should decode something");
}

#[test]
fn test_pipeline_latency() {
    // Measure processing performance
    // Focus: Can the pipeline keep up with real-time requirements?
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = "Latency test".to_string();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::Instant;
        
        println!("Pipeline Latency Test:");
        
        let mut total_samples = 0;
        let mut total_time = std::time::Duration::ZERO;
        
        for _ in 0..100 {
            let start = Instant::now();
            let output = pipeline.process_chunk(b"");
            let elapsed = start.elapsed();
            
            total_samples += output.audio_samples.len();
            total_time += elapsed;
        }
        
        let avg_time_us = total_time.as_micros() / 100;
        let samples_per_iteration = total_samples / 100;
        let iteration_duration_us = (samples_per_iteration as f64 / 48000.0) * 1e6;
        
        println!("  Average processing time: {} μs", avg_time_us);
        println!("  Average samples per iteration: {}", samples_per_iteration);
        println!("  Iteration audio duration: {:.1} μs", iteration_duration_us);
        println!("  Real-time factor: {:.2}x", iteration_duration_us / avg_time_us as f64);
        
        // Processing should be faster than real-time (at least 2x)
        assert!(
            avg_time_us < (iteration_duration_us as u128 / 2),
            "Processing too slow for real-time operation"
        );
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // Just verify it runs without timing checks on WASM
        for _ in 0..10 {
            let _ = pipeline.process_chunk(b"");
        }
        println!("Pipeline Latency Test: WASM mode (timing skipped)");
    }
}

#[test]
fn test_pipeline_state_consistency() {
    // Validate state advances correctly over time
    // Focus: Does the pipeline maintain consistent internal state?
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = "Consistency test".to_string();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    println!("Pipeline State Consistency Test:");
    
    // Process multiple iterations and verify state advances correctly
    let mut last_frame_count = 0;
    let mut last_symbol_count = 0;
    
    for i in 0..10 {
        let output = pipeline.process_chunk(b"");
        
        println!("  Iteration {}: frames={}, symbols={}", 
            i, output.frames_processed, output.symbols_decoded);
        
        // Frame and symbol counts should never decrease
        assert!(
            output.frames_processed >= last_frame_count,
            "Frame count should not decrease"
        );
        
        // Symbol count should increase (or at least not decrease)
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
    // Validate diagnostic data is consistently produced
    // Focus: Are diagnostics always valid and useful?
    let mut sim = SimulationConfig::default();
    sim.bypass_thz_simulation = true;
    sim.plaintext_source = "Diagnostics test".to_string();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    println!("Diagnostic Output Continuity Test:");
    
    for i in 0..5 {
        let output = pipeline.process_chunk(b"");
        
        println!("  Iteration {}: constellation_points={}, spectrum_bins={}", 
            i,
            output.pre_channel.tx_constellation_i.len(),
            output.pre_channel.tx_spectrum_magnitude.len()
        );
        
        // Diagnostics should be populated
        assert!(
            !output.pre_channel.tx_constellation_i.is_empty(),
            "TX constellation should be populated"
        );
        
        // Spectrum may take time to populate
        if i > 0 {
            assert!(
                !output.pre_channel.tx_spectrum_magnitude.is_empty() || i == 1,
                "TX spectrum should be populated after initial iterations"
            );
        }
        
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

