use proptest::prelude::*;
use chimera_core::{
    config::{ProtocolConfig, LDPCConfig, FrameLayout},
    encoder::StreamingFrameEncoder,
    decoder::StreamingSymbolDecoder,
    ldpc::{LDPCSuite, decode_ldpc},
    utils::string_to_bitstream,
};

proptest! {
    /// Test that encoding and decoding round-trip correctly for any message
    #[test]
    fn test_encode_decode_roundtrip(message in "[a-zA-Z0-9 ]{1,100}") {
        let protocol = ProtocolConfig::default();
        let ldpc_config = LDPCConfig::default();
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
        
        let payload_bits = string_to_bitstream(&message);
        let mut encoder = StreamingFrameEncoder::new(
            &payload_bits,
            protocol.clone(),
            ldpc_suite.matrices.clone(),
        );
        
        // Generate all symbols for all frames
        let total_symbols = protocol.frame_layout.total_symbols * encoder.total_frames;
        let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(total_symbols);
        
        // Decode
        let mut decoder = StreamingSymbolDecoder::new(
            protocol.clone(),
            ldpc_suite.matrices.clone(),
        );
        
        decoder.process_symbols(&tx_symbols);
        let decoded_payload = decoder.get_decoded_payload();
        
        // Compare original and decoded (up to length of decoded)
        let min_len = decoded_payload.len().min(payload_bits.len());
        prop_assert_eq!(&decoded_payload[..min_len], &payload_bits[..min_len]);
    }
    
    /// Test LDPC encoding/decoding roundtrip
    #[test]
    fn test_ldpc_roundtrip(message_bits in prop::collection::vec(prop::bool::ANY, 128)) {
        let layout = FrameLayout::default();
        let ldpc_config = LDPCConfig::default();
        let suite = LDPCSuite::new(&layout, &ldpc_config);
        
        let message: Vec<u8> = message_bits.iter().map(|&b| if b { 1 } else { 0 }).collect();
        
        // Encode
        let mut codeword = vec![0u8; suite.matrices.codeword_bits];
        for (row_idx, &bit) in message.iter().enumerate() {
            if bit == 0 {
                continue;
            }
            for (col_idx, codeword_bit) in codeword.iter_mut().enumerate() {
                *codeword_bit ^= suite.matrices.generator[(row_idx, col_idx)] & 1;
            }
        }
        
        // Decode
        let decoded = decode_ldpc(&suite.matrices, &codeword, 0.0);
        
        prop_assert_eq!(decoded, message);
    }
    
    /// Test that configuration validation catches invalid values
    #[test]
    fn test_config_validation_catches_invalid_snr(snr_db in prop::num::f64::ANY) {
        let mut config = chimera_core::config::SimulationConfig::default();
        config.snr_db = snr_db;
        
        if snr_db.is_finite() {
            prop_assert!(config.validate().is_ok());
        } else {
            prop_assert!(config.validate().is_err());
        }
    }
    
    /// Test frame layout validation
    #[test]
    fn test_frame_layout_validation(
        sync in 1usize..32,
        target in 1usize..32,
        command in 1usize..32,
        data in 1usize..128,
        ecc in 1usize..32,
    ) {
        let total = sync + target + command + data + ecc;
        let layout = FrameLayout {
            total_symbols: total,
            sync_symbols: sync,
            target_id_symbols: target,
            command_type_symbols: command,
            data_payload_symbols: data,
            ecc_symbols: ecc,
        };
        
        // Should always validate when sum matches
        prop_assert!(layout.validate().is_ok());
        
        // Should fail when sum doesn't match
        let bad_layout = FrameLayout {
            total_symbols: total + 1,
            ..layout
        };
        prop_assert!(bad_layout.validate().is_err());
    }
    
    /// Test that symbol generation is deterministic
    #[test]
    fn test_encoder_determinism(message in "[a-zA-Z]{10,50}", seed in prop::num::u64::ANY) {
        let protocol = ProtocolConfig::default();
        let mut ldpc_config = LDPCConfig::default();
        ldpc_config.seed = Some(seed);
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
        
        let payload_bits = string_to_bitstream(&message);
        
        // Encode twice with same config
        let mut encoder1 = StreamingFrameEncoder::new(
            &payload_bits,
            protocol.clone(),
            ldpc_suite.matrices.clone(),
        );
        let (symbols1, _, _, _, _) = encoder1.get_next_symbols(64);
        
        let mut encoder2 = StreamingFrameEncoder::new(
            &payload_bits,
            protocol.clone(),
            ldpc_suite.matrices.clone(),
        );
        let (symbols2, _, _, _, _) = encoder2.get_next_symbols(64);
        
        // Should produce identical symbols
        prop_assert_eq!(symbols1, symbols2);
    }
    
    /// Test DSP utility functions produce reasonable results
    #[test]
    fn test_fast_atan2_approximation(y in -10.0f32..10.0, x in -10.0f32..10.0) {
        use chimera_core::utils::dsp::fast_atan2_approx;
        
        if x.abs() > 0.001 || y.abs() > 0.001 {  // Avoid very small values
            let fast_result = fast_atan2_approx(y, x);
            let std_result = y.atan2(x);
            let error = (fast_result - std_result).abs();
            
            // Approximation should be within 0.1 radians (~5.7 degrees)
            // This is reasonable for real-time DSP where speed matters more than precision
            prop_assert!(error < 0.1, "Error too large: {} vs {} (error: {})", fast_result, std_result, error);
        }
    }
    
    /// Test that phase unwrapping handles discontinuities
    #[test]
    fn test_phase_unwrap(phases in prop::collection::vec(-3.14f32..3.14, 10..50)) {
        use chimera_core::utils::dsp::phase_unwrap;
        
        let mut unwrapped = phases.clone();
        phase_unwrap(&mut unwrapped);
        
        // Check that differences are smooth (no 2π jumps)
        for i in 1..unwrapped.len() {
            let diff = (unwrapped[i] - unwrapped[i-1]).abs();
            prop_assert!(diff < std::f32::consts::PI, "Large discontinuity detected: {}", diff);
        }
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    
    #[test]
    fn test_empty_message_encoding() {
        let protocol = ProtocolConfig::default();
        let ldpc_config = LDPCConfig::default();
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
        
        let payload_bits = vec![];
        let mut encoder = StreamingFrameEncoder::new(
            &payload_bits,
            protocol.clone(),
            ldpc_suite.matrices.clone(),
        );
        
        // Should still generate at least one frame
        assert!(encoder.total_frames >= 1);
        
        let (symbols, _, _, _, _) = encoder.get_next_symbols(16);
        assert!(!symbols.is_empty());
    }
    
    #[test]
    fn test_sincos_lut_consistency() {
        use chimera_core::utils::dsp::SinCosLut;
        use std::f32::consts::PI;
        
        let lut = SinCosLut::new(1024);
        
        // Test at known angles
        let (sin0, cos0) = lut.lookup(0.0);
        assert!((sin0 - 0.0).abs() < 0.01);
        assert!((cos0 - 1.0).abs() < 0.01);
        
        let (sin90, cos90) = lut.lookup(PI / 2.0);
        assert!((sin90 - 1.0).abs() < 0.01);
        assert!((cos90 - 0.0).abs() < 0.01);
        
        let (sin180, cos180) = lut.lookup(PI);
        assert!((sin180 - 0.0).abs() < 0.01);
        assert!((cos180 - (-1.0)).abs() < 0.01);
    }
}
