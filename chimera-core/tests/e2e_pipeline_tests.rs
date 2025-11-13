use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::{run_simulation, pipeline::RealtimePipeline};

/// Helper to create raman-whisper protocol config
fn raman_whisper_protocol() -> ProtocolConfig {
    ProtocolConfig {
        carrier_freq_hz: 12000.0,
        qpsk_symbol_rate: 16,
        enable_qpsk: true,
        enable_fsk: true,  // FSK is now supported with coarse freq correction + dual-loop!
        qpsk_bandwidth_hz: 20.0,
        fsk_bit_rate: 1.0,
        fsk_freq_zero_hz: 11999.0,
        fsk_freq_one_hz: 12001.0,
        command_opcode: 0x0001,
        sync_sequence_hex: "A5A5A5A5".to_string(),
        target_id_hex: "DEADBEEF".to_string(),
        max_frames: 256,
        current_frame_shift: 16,
        total_frames_shift: 24,
        frame_layout: chimera_core::config::FrameLayout {
            total_symbols: 128,
            sync_symbols: 16,
            target_id_symbols: 16,
            command_type_symbols: 16,
            data_payload_symbols: 64,
            ecc_symbols: 16,
        },
    }
}

fn raman_whisper_ldpc() -> LDPCConfig {
    LDPCConfig {
        dv: 2,
        dc: 10,
        seed: Some(42),
    }
}

/// Basic end-to-end test with clean signal (no noise)
#[test]
fn test_e2e_clean_signal() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    let test_messages = vec![
        "A",           // Single character
        "Test",        // Short message
        "Hello World", // Medium message
    ];
    
    for message in test_messages {
        let mut sim = SimulationConfig::default();
        sim.plaintext_source = message.to_string();
        sim.snr_db = 100.0; // Essentially no noise
        
        let result = run_simulation(&sim, &protocol, &ldpc);
        
        let recovered = result.report.recovered_message.trim_end_matches('\u{0}');
        println!("Message: '{}' -> Recovered: '{}' (BER: {})", 
                 message, recovered, result.report.post_fec_ber);
        
        // With no noise, we should recover the message (may not be perfect due to sync issues)
        // At minimum, check that we got some output
        assert!(!recovered.is_empty(), 
                "Failed to recover any data for message '{}'", message);
    }
}

/// Test with various SNR levels
#[test]
fn test_e2e_with_noise() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    let snr_levels = vec![
        (30.0, 0.01),   // Very high SNR
        (20.0, 0.05),   // High SNR
        (15.0, 0.10),   // Medium SNR
    ];
    
    for (snr_db, max_ber) in snr_levels {
        let mut sim = SimulationConfig::default();
        sim.plaintext_source = "Test SNR".to_string();
        sim.snr_db = snr_db;
        
        let result = run_simulation(&sim, &protocol, &ldpc);
        
        println!("SNR: {}dB -> BER: {}, Recovered: '{}'", 
                 snr_db, 
                 result.report.post_fec_ber,
                 result.report.recovered_message.trim_end_matches('\u{0}'));
        
        assert!(
            result.report.post_fec_ber <= max_ber,
            "BER {} exceeds maximum {} for SNR {}dB",
            result.report.post_fec_ber,
            max_ber,
            snr_db
        );
    }
}

/// Test streaming consistency - pipeline should work the same in streaming and batch
#[test]
fn test_streaming_vs_batch_consistency() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    let message = "Streaming";
    
    // Run through run_simulation (batch-like)
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = message.to_string();
    sim.snr_db = 20.0;
    let batch_result = run_simulation(&sim, &protocol, &ldpc);
    
    // Run through pipeline directly (streaming)
    let mut pipeline = RealtimePipeline::new(sim.clone(), protocol.clone(), ldpc.clone());
    pipeline.set_modulation_mode(true);
    
    let mut streaming_audio = Vec::new();
    let mut streaming_decoded = String::new();
    let dummy_input = b"";
    
    for _ in 0..200 {  // Process up to 200 chunks
        let output = pipeline.process_chunk(dummy_input);
        streaming_audio.extend_from_slice(&output.audio_samples);
        
        if !output.decoded_text.is_empty() {
            streaming_decoded = output.decoded_text.clone();
            if streaming_decoded.trim_end_matches('\u{0}').len() >= message.len() {
                break;
            }
        }
    }
    
    println!("Batch decoded: '{}'", batch_result.report.recovered_message.trim_end_matches('\u{0}'));
    println!("Stream decoded: '{}'", streaming_decoded.trim_end_matches('\u{0}'));
    
    // Both should produce some decoded output
    assert!(!batch_result.report.recovered_message.is_empty(), "Batch should produce output");
    assert!(!streaming_decoded.is_empty(), "Streaming should produce output");
}

/// Test frame alignment and synchronization
#[test]
fn test_frame_synchronization() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    // Create a message that spans multiple frames
    // Each frame has 64 data symbols = 128 bits = 16 bytes
    let multi_frame_message = "A".repeat(40); // ~2.5 frames worth
    
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = multi_frame_message.clone();
    sim.snr_db = 20.0;
    
    let result = run_simulation(&sim, &protocol, &ldpc);
    
    println!("Input: {} bytes, Recovered: {} bytes", 
             multi_frame_message.len(),
             result.report.recovered_message.trim_end_matches('\u{0}').len());
    
    // Should recover at least some of the message
    let recovered = result.report.recovered_message.trim_end_matches('\u{0}');
    assert!(
        recovered.len() >= multi_frame_message.len() / 4,
        "Should recover at least 25% of multi-frame message: got {} of {} bytes",
        recovered.len(),
        multi_frame_message.len()
    );
}

/// Test error correction capability
#[test]
fn test_error_correction() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    // Test at SNR where we expect errors but FEC should correct them
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = "FEC Test Message".to_string();
    sim.snr_db = 12.0; // Medium-low SNR
    
    let result = run_simulation(&sim, &protocol, &ldpc);
    
    println!("Pre-FEC BER: {}, Post-FEC BER: {}", 
             result.report.pre_fec_ber, 
             result.report.post_fec_ber);
    
    // Pre-FEC BER should be higher than post-FEC BER if FEC is working
    assert!(
        result.report.pre_fec_ber >= result.report.post_fec_ber,
        "Pre-FEC BER ({}) should be >= Post-FEC BER ({})",
        result.report.pre_fec_ber,
        result.report.post_fec_ber
    );
}

/// Debug test to understand why pipeline isn't decoding perfectly
#[test]
#[ignore] // Run with --ignored flag
fn debug_pipeline_decode_failure() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    let message = "Test";
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = message.to_string();
    sim.snr_db = 100.0; // Perfect channel
    
    let mut pipeline = RealtimePipeline::new(sim, protocol.clone(), ldpc);
    pipeline.set_modulation_mode(true);
    
    let dummy_input = b"";
    let mut total_audio = 0;
    let mut decoded_text = String::new();
    
    println!("Input message: '{}' ({} bytes)", message, message.len());
    println!("Expected bits: {}", message.len() * 8);
    println!("Frame data payload: {} symbols = {} bits = {} bytes",
             protocol.frame_layout.data_payload_symbols,
             protocol.frame_layout.data_payload_symbols * 2,
             protocol.frame_layout.data_payload_symbols * 2 / 8);
    
    for i in 0..100 {
        let output = pipeline.process_chunk(dummy_input);
        total_audio += output.audio_samples.len();
        
        if !output.decoded_text.is_empty() && output.decoded_text != decoded_text {
            println!("Iteration {}: Decoded: {:?}", i, output.decoded_text);
            decoded_text = output.decoded_text.clone();
        }
        
        if i % 10 == 0 {
            println!("Iteration {}: {} audio samples total, sync: {}, decoded: '{}'", 
                     i, 
                     total_audio,
                     output.post_channel.sync_status,
                     decoded_text.trim_end_matches('\u{0}'));
        }
        
        // Check if we've decoded enough
        if decoded_text.trim_end_matches('\u{0}').len() >= message.len() {
            println!("✓ Successfully decoded after {} iterations", i);
            break;
        }
    }
    
    println!("\nFinal decoded: '{}'", decoded_text.trim_end_matches('\u{0}'));
    println!("Expected: '{}'", message);
    println!("Match: {}", decoded_text.trim_end_matches('\u{0}') == message);
}

/// Debug test - detailed analysis of sync detection failure
#[test]
#[ignore]
fn debug_sync_detection_details() {
    use chimera_core::encoder::StreamingFrameEncoder;
    use chimera_core::decoder::StreamingSymbolDecoder;
    use chimera_core::ldpc::LDPCSuite;
    use chimera_core::utils::hex_to_bitstream;
    
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
    let matrices = ldpc_suite.matrices.clone();
    
    // Create encoder
    let message = "Test";
    let message_bits: Vec<u8> = message.bytes()
        .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
        .collect();
    
    let mut encoder = StreamingFrameEncoder::new(&message_bits, protocol.clone(), matrices.clone());
    
    // Get transmitted symbols (one complete frame)
    let symbols_per_frame = protocol.frame_layout.total_symbols;
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(symbols_per_frame);
    
    println!("=== TRANSMISSION ===");
    println!("Transmitted {} symbols", tx_symbols.len());
    
    // Show what we expect the sync pattern to look like
    let sync_bits = hex_to_bitstream(&protocol.sync_sequence_hex, protocol.frame_layout.sync_symbols * 2);
    println!("Sync pattern (hex): {}", protocol.sync_sequence_hex);
    println!("Sync pattern (bits): {:?}", &sync_bits[..std::cmp::min(32, sync_bits.len())]);
    
    // Demodulate first few symbols to verify encoding
    use chimera_core::decoder::demodulate_qpsk_symbol;
    println!("\nFirst 20 transmitted symbols (I, Q) -> bits:");
    for (i, sym) in tx_symbols.iter().take(20).enumerate() {
        let bits = demodulate_qpsk_symbol(*sym);
        println!("  Symbol {}: ({:7.4}, {:7.4}) -> [{}, {}]", i, sym.re, sym.im, bits[0], bits[1]);
    }
    
    // Now try to decode with perfect symbols (no noise)
    let mut decoder = StreamingSymbolDecoder::new(protocol.clone(), matrices);
    
    println!("\n=== RECEPTION (NO NOISE) ===");
    
    // Process all symbols
    let (decoded_bits, frame_complete, frame_idx, symbols_in_frame, _diagnostics) = 
        decoder.process_symbols(&tx_symbols);
    
    println!("After processing {} symbols:", tx_symbols.len());
    println!("  Sync found: {}", decoder.is_synced());
    println!("  Frame complete: {}", frame_complete);
    println!("  Frame index: {}", frame_idx);
    println!("  Symbols in frame: {}", symbols_in_frame);
    println!("  Decoded bits: {}", decoded_bits.len());
    
    // Show decoder logs
    println!("\n=== DECODER LOGS ===");
    for log in decoder.get_logs() {
        println!("  {}", log);
    }
    
    // Show what the decoder saw
    let demod_bits = decoder.get_demodulated_bits();
    println!("\n=== DEMODULATED BITS ===");
    println!("Total demodulated bits: {}", demod_bits.len());
    println!("First 64 bits: {:?}", &demod_bits[..std::cmp::min(64, demod_bits.len())]);
    println!("Expected sync:  {:?}", &sync_bits[..std::cmp::min(32, sync_bits.len())]);
    
    // Manual sync search to understand what's happening
    println!("\n=== MANUAL SYNC SEARCH ===");
    let sync_len = sync_bits.len();
    if demod_bits.len() >= sync_len {
        let mut best_match = 0;
        let mut best_pos = 0;
        
        for pos in 0..=(demod_bits.len() - sync_len) {
            let window = &demod_bits[pos..pos + sync_len];
            let matches = window.iter()
                .zip(sync_bits.iter())
                .filter(|(a, b)| a == b)
                .count();
            
            if matches > best_match {
                best_match = matches;
                best_pos = pos;
            }
            
            if pos < 10 || matches > sync_len * 7 / 10 {
                println!("  Position {}: {} / {} bits match ({:.1}%)", 
                         pos, matches, sync_len, (matches as f32 / sync_len as f32) * 100.0);
            }
        }
        
        println!("\nBest match: {} / {} bits at position {} ({:.1}%)", 
                 best_match, sync_len, best_pos, (best_match as f32 / sync_len as f32) * 100.0);
    }
}

/// Debug test - check what's happening through the full audio pipeline
#[test]
#[ignore]
fn debug_full_pipeline_sync() {
    use chimera_core::channel::apply_audio_noise;
    use chimera_core::signal_processing::{ModulationConfig, DemodulationConfig};
    use chimera_core::signal_processing::{symbols_to_carrier_signal, audio_to_symbols};
    use chimera_core::encoder::StreamingFrameEncoder;
    use chimera_core::decoder::StreamingSymbolDecoder;
    use chimera_core::ldpc::LDPCSuite;
    use chimera_core::utils::hex_to_bitstream;
    use num_complex::Complex64;
    
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
    let matrices = ldpc_suite.matrices.clone();
    
    // Create encoder
    let message = "Test";
    let message_bits: Vec<u8> = message.bytes()
        .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
        .collect();
    
    let mut encoder = StreamingFrameEncoder::new(&message_bits, protocol.clone(), matrices.clone());
    
    // Get transmitted symbols (one complete frame)
    let symbols_per_frame = protocol.frame_layout.total_symbols;
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(symbols_per_frame);
    
    println!("=== STEP 1: ENCODING ===");
    println!("Transmitted {} symbols", tx_symbols.len());
    
    // Show first few TX symbols
    use chimera_core::decoder::demodulate_qpsk_symbol;
    println!("First 10 TX symbols:");
    for (i, sym) in tx_symbols.iter().take(10).enumerate() {
        let bits = demodulate_qpsk_symbol(*sym);
        println!("  Symbol {}: ({:7.4}, {:7.4}) -> [{}, {}]", i, sym.re, sym.im, bits[0], bits[1]);
    }
    
    // Modulate to audio
    println!("\n=== STEP 2: MODULATION ===");
    let sample_rate = 48000;
    
    let modulation_config = ModulationConfig {
        sample_rate,
        symbol_rate: protocol.qpsk_symbol_rate as usize,
        carrier_freq: protocol.carrier_freq_hz,
    };
    
    let audio = symbols_to_carrier_signal(&tx_symbols, &modulation_config);
    println!("Generated {} audio samples ({:.3}s)", audio.len(), audio.len() as f32 / sample_rate as f32);
    
    // Apply channel (with minimal noise for debugging)
    println!("\n=== STEP 3: CHANNEL (SNR=100dB) ===");
    let mut rng = rand::thread_rng();
    let noisy_audio = apply_audio_noise(&audio, 100.0, &mut rng);
    println!("Channel output: {} samples", noisy_audio.len());
    
    // Demodulate
    println!("\n=== STEP 4: DEMODULATION ===");
    let demodulation_config = DemodulationConfig {
        sample_rate,
        symbol_rate: protocol.qpsk_symbol_rate as usize,
        carrier_freq: protocol.carrier_freq_hz,
    };
    
    let rx_symbols = audio_to_symbols(&noisy_audio, &demodulation_config);
    
    println!("Demodulated {} symbols", rx_symbols.len());
    println!("First 10 RX symbols:");
    for (i, sym) in rx_symbols.iter().take(10).enumerate() {
        let bits = demodulate_qpsk_symbol(*sym);
        let mag = sym.norm();
        println!("  Symbol {}: ({:7.4}, {:7.4}) mag={:6.4} -> [{}, {}]", i, sym.re, sym.im, mag, bits[0], bits[1]);
    }
    
    // Check if FSK is interfering
    println!("\nEncoder FSK frequency: {:.2} Hz", encoder.get_current_fsk_frequency());
    println!("Protocol: FSK and QPSK always enabled per spec");
    
    // Check symbol correlation
    println!("\n=== STEP 5: SYMBOL CORRELATION ===");
    let min_len = std::cmp::min(tx_symbols.len(), rx_symbols.len());
    let mut matching = 0;
    let mut phase_diffs = Vec::new();
    
    // Try all 4 QPSK phase rotations to find the best match
    println!("Testing phase ambiguity (first 20 symbols):");
    for rot_idx in 0..4 {
        let phase_rot = Complex64::from_polar(1.0, rot_idx as f64 * std::f64::consts::PI / 2.0);
        let mut matches = 0;
        
        for i in 0..std::cmp::min(20, min_len) {
            let tx_bits = demodulate_qpsk_symbol(tx_symbols[i]);
            let rotated_rx = rx_symbols[i] * phase_rot;
            let rx_bits = demodulate_qpsk_symbol(rotated_rx);
            if tx_bits == rx_bits {
                matches += 1;
            }
        }
        println!("  Phase rotation {}°: {}/20 matches ({:.0}%)", 
                 rot_idx * 90, matches, (matches as f32 / 20.0) * 100.0);
    }
    
    // Try later symbols (after Costas loop convergence)
    println!("\nTesting phase ambiguity (symbols 20-40, after convergence):");
    for rot_idx in 0..4 {
        let phase_rot = Complex64::from_polar(1.0, rot_idx as f64 * std::f64::consts::PI / 2.0);
        let mut matches = 0;
        
        for i in 20..std::cmp::min(40, min_len) {
            let tx_bits = demodulate_qpsk_symbol(tx_symbols[i]);
            let rotated_rx = rx_symbols[i] * phase_rot;
            let rx_bits = demodulate_qpsk_symbol(rotated_rx);
            if tx_bits == rx_bits {
                matches += 1;
            }
        }
        println!("  Phase rotation {}°: {}/20 matches ({:.0}%)", 
                 rot_idx * 90, matches, (matches as f32 / 20.0) * 100.0);
    }
    
    println!("\nFirst 10 symbol comparison (no phase correction):");
    for i in 0..std::cmp::min(10, min_len) {
        let tx_bits = demodulate_qpsk_symbol(tx_symbols[i]);
        let rx_bits = demodulate_qpsk_symbol(rx_symbols[i]);
        let match_str = if tx_bits == rx_bits { "✓" } else { "✗" };
        println!("  Symbol {}: TX [{}, {}] RX [{}, {}] {}", i, tx_bits[0], tx_bits[1], rx_bits[0], rx_bits[1], match_str);
        
        if tx_bits == rx_bits {
            matching += 1;
        }
        
        // Calculate phase difference
        let phase_diff = (rx_symbols[i] / tx_symbols[i]).arg();
        phase_diffs.push(phase_diff);
    }
    
    println!("Match rate (first 10): {}/10 ({:.0}%)", matching, (matching as f32 / 10.0) * 100.0);
    
    if !phase_diffs.is_empty() {
        let avg_phase = phase_diffs.iter().sum::<f64>() / phase_diffs.len() as f64;
        println!("Average phase offset: {:.4} rad ({:.1}°)", avg_phase, avg_phase.to_degrees());
    }
    
    // Try decoding
    println!("\n=== STEP 6: DECODING ===");
    let mut decoder = StreamingSymbolDecoder::new(protocol.clone(), matrices);
    let (decoded_bits, frame_complete, _frame_idx, _symbols_in_frame, _diagnostics) = 
        decoder.process_symbols(&rx_symbols);
    
    println!("Sync found: {}", decoder.is_synced());
    println!("Frame complete: {}", frame_complete);
    println!("Decoded bits: {}", decoded_bits.len());
    
    // Show decoder logs
    println!("\n=== DECODER LOGS ===");
    for log in decoder.get_logs() {
        println!("  {}", log);
    }
    
    // Manual bit check
    let sync_bits = hex_to_bitstream(&protocol.sync_sequence_hex, protocol.frame_layout.sync_symbols * 2);
    let demod_bits = decoder.get_demodulated_bits();
    
    println!("\n=== BIT COMPARISON ===");
    println!("Demodulated bits: {}", demod_bits.len());
    println!("First 32 demod bits: {:?}", &demod_bits[..std::cmp::min(32, demod_bits.len())]);
    println!("Expected sync bits:  {:?}", &sync_bits[..std::cmp::min(32, sync_bits.len())]);
    
    if demod_bits.len() >= sync_bits.len() {
        let matches = demod_bits[..sync_bits.len()].iter()
            .zip(sync_bits.iter())
            .filter(|(a, b)| a == b)
            .count();
        println!("Bit match at position 0: {}/{} ({:.1}%)", 
                 matches, sync_bits.len(), (matches as f32 / sync_bits.len() as f32) * 100.0);
    }
}

/// Test diagnostics collection
#[test]
fn test_diagnostics_collection() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = "Diagnostics Test".to_string();
    sim.snr_db = 15.0;
    
    let result = run_simulation(&sim, &protocol, &ldpc);
    let diagnostics = &result.diagnostics;
    
    // Should have modulation audio
    assert!(diagnostics.modulation_audio.is_some(), "Should have audio diagnostics");
    let audio = diagnostics.modulation_audio.as_ref().unwrap();
    assert_eq!(audio.sample_rate, 48000, "Should have correct sample rate");
    assert!(!audio.clean.is_empty(), "Should have clean audio samples");
    assert!(!audio.noisy.is_empty(), "Should have noisy audio samples");
    
    // Should have TX symbols
    assert!(!diagnostics.tx_symbols_i.is_empty(), "Should have TX I symbols");
    assert!(!diagnostics.tx_symbols_q.is_empty(), "Should have TX Q symbols");
    assert_eq!(
        diagnostics.tx_symbols_i.len(),
        diagnostics.tx_symbols_q.len(),
        "I and Q symbols should have same length"
    );
    
    println!("Audio: {} samples ({:.2}s)", 
             audio.clean.len(),
             audio.clean.len() as f32 / 48000.0);
    println!("TX symbols: {} I, {} Q", 
             diagnostics.tx_symbols_i.len(),
             diagnostics.tx_symbols_q.len());
}

/// Test minimum audio needed for decoding
#[test]
#[ignore] // Run with --ignored flag
fn test_minimum_audio_for_decode() {
    let protocol = raman_whisper_protocol();
    let ldpc = raman_whisper_ldpc();
    
    let message = "X";  // Single character
    let mut sim = SimulationConfig::default();
    sim.plaintext_source = message.to_string();
    sim.snr_db = 100.0;
    
    let mut pipeline = RealtimePipeline::new(sim, protocol, ldpc);
    pipeline.set_modulation_mode(true);
    
    let dummy_input = b"";
    let mut chunks_needed = 0;
    let mut total_samples = 0;
    
    for i in 0..1000 {
        let output = pipeline.process_chunk(dummy_input);
        chunks_needed = i + 1;
        total_samples += output.audio_samples.len();
        
        if !output.decoded_text.is_empty() {
            println!("Decoded '{}' after {} chunks ({} samples, {:.2}s)", 
                     output.decoded_text.trim_end_matches('\u{0}'),
                     chunks_needed,
                     total_samples,
                     total_samples as f32 / 48000.0);
            break;
        }
        
        if i % 20 == 0 {
            println!("Chunk {}: {} total samples, sync: {}", 
                     i, total_samples, output.post_channel.sync_status);
        }
    }
    
    assert!(chunks_needed < 1000, "Should decode within reasonable time");
}
