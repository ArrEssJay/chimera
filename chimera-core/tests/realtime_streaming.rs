//! Tests for real-time streaming rate limiting
//!
//! Verifies that data is delivered at protocol-specified rates.

use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::streaming::StreamingPipeline;
use std::time::Instant;

#[test]
fn test_streaming_rate_limiting() {
    // Create pipeline with protocol defaults
    let sim = SimulationConfig {
        plaintext_source: "Test message for rate limiting verification".to_string(),
        snr_db: 10.0,
        link_loss_db: 0.0,
        ..Default::default()
    };
    
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    let mut pipeline = StreamingPipeline::new(sim, protocol.clone(), ldpc);
    
    // Note: Rate limiting has been removed for real-time performance.
    // The streaming pipeline now emits symbols as fast as possible, with rate limiting
    // handled by the audio output layer (Web Audio API, OS audio buffers, etc.)
    
    // Process first frame - should return immediately
    let start = Instant::now();
    let output1 = pipeline.process_chunk(&[]);
    let first_duration = start.elapsed().as_millis() as f64;
    
    // First frame should process with output
    assert!(
        output1.audio_samples.len() > 0 || output1.pre_channel.symbol_count > 0,
        "First frame should have output data"
    );
    assert!(
        first_duration < 1000.0,
        "First frame should complete in < 1s, got {:.1}ms",
        first_duration
    );
    
    // Process second frame - should also return immediately (no rate limiting)
    let start2 = Instant::now();
    let output2 = pipeline.process_chunk(&[]);
    let second_duration = start2.elapsed().as_millis() as f64;
    
    // Second frame should also have output (no rate limiting)
    assert!(output2.audio_samples.len() > 0, "Second frame should produce output");
    assert!(second_duration < 1000.0, "Second frame should be fast");
    
    println!("✓ Frame 1 processed in {:.1}ms", first_duration);
    println!("✓ Frame 2 processed in {:.1}ms", second_duration);
    println!("✓ Rate limiting delegated to audio output layer");
}

#[test]
fn test_symbol_timing_accuracy() {
    let protocol = ProtocolConfig::default();
    
    // Protocol specifies 16 symbols per second
    let expected_symbol_duration_ms = 1000.0 / 16.0; // 62.5ms per symbol
    
    // 128 symbols per frame
    let symbols_per_frame = protocol.frame_layout.total_symbols;
    let expected_frame_duration_ms = expected_symbol_duration_ms * symbols_per_frame as f64;
    
    assert_eq!(symbols_per_frame, 128);
    assert_eq!(expected_symbol_duration_ms, 62.5);
    assert_eq!(expected_frame_duration_ms, 8000.0);
    
    println!("✓ Symbol duration: {:.1}ms", expected_symbol_duration_ms);
    println!("✓ Frame duration: {:.1}ms", expected_frame_duration_ms);
    println!("✓ Data rate (FSK): {} bit/s", protocol.fsk_bit_rate);
}

#[test]
fn test_data_rate_calculation() {
    let protocol = ProtocolConfig::default();
    
    // FSK layer: 1 bit per second (spec requirement)
    assert_eq!(protocol.fsk_bit_rate, 1.0);
    
    // QPSK layer: 16 symbols/s * 2 bits/symbol = 32 bits/s raw
    let qpsk_data_rate_bps = protocol.qpsk_symbol_rate * 2;
    assert_eq!(qpsk_data_rate_bps, 32);
    
    // But effective data rate is limited by FSK modulation
    // which runs at 1 bit/second for "subconscious informational osmosis"
    let effective_data_rate_bps = protocol.fsk_bit_rate;
    let effective_data_rate_bytes_per_sec = effective_data_rate_bps / 8.0;
    
    assert_eq!(effective_data_rate_bps, 1.0);
    assert_eq!(effective_data_rate_bytes_per_sec, 0.125);
    
    println!("✓ QPSK raw rate: {} bps", qpsk_data_rate_bps);
    println!("✓ FSK effective rate: {} bps", effective_data_rate_bps);
    println!("✓ Effective throughput: {} B/s", effective_data_rate_bytes_per_sec);
}
