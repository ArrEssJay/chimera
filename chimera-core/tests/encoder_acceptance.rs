use chimera_core::config::{LDPCConfig, ProtocolConfig};
use chimera_core::encoder::build_frame_stream;
use chimera_core::ldpc::LDPCSuite;
use chimera_core::utils::{hex_to_bitstream, int_to_bitstream, string_to_bitstream, LogCollector};

fn default_suite() -> (ProtocolConfig, LDPCConfig, LDPCSuite) {
    let protocol = ProtocolConfig::default();
    let ldpc_cfg = LDPCConfig::default();
    let suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_cfg);
    (protocol, ldpc_cfg, suite)
}

#[test]
fn given_small_payload_when_build_frames_then_single_frame_is_emitted() {
    let (protocol, _ldpc_cfg, suite) = default_suite();
    let payload_bits = string_to_bitstream("Hi");
    let mut logger = LogCollector::new();

    let frame_stream = build_frame_stream(&payload_bits, &protocol, &suite.matrices, &mut logger);

    let layout = &protocol.frame_layout;
    assert_eq!(frame_stream.frame_count, 1);
    assert_eq!(frame_stream.frames_bitstream.len(), layout.frame_bits());

    let sync_len = layout.sync_symbols * 2;
    let target_len = layout.target_id_symbols * 2;
    let command_len = layout.command_type_symbols * 2;

    let sync_bits = hex_to_bitstream(&protocol.sync_sequence_hex, sync_len);
    assert_eq!(&frame_stream.frames_bitstream[..sync_len], sync_bits.as_slice());

    let command_start = sync_len + target_len;
    let command_end = command_start + command_len;
    let expected_command = protocol.command_opcode | (0 << protocol.current_frame_shift) | (1 << protocol.total_frames_shift);
    let expected_command_bits = int_to_bitstream(expected_command as u64, command_len);
    assert_eq!(&frame_stream.frames_bitstream[command_start..command_end], expected_command_bits.as_slice());

    assert!(!frame_stream.logs.is_empty(), "frame builder should emit trace logs");
}

#[test]
fn given_large_payload_when_build_frames_then_metadata_tracks_frame_progression() {
    let (protocol, _ldpc_cfg, suite) = default_suite();
    let message_bits = suite.matrices.message_bits;
    let payload_bits = (0..(message_bits * 2 + message_bits / 2))
        .map(|i| (i % 2) as u8)
        .collect::<Vec<_>>();
    let mut logger = LogCollector::new();

    let frame_stream = build_frame_stream(&payload_bits, &protocol, &suite.matrices, &mut logger);

    let expected_frames = (payload_bits.len() + message_bits - 1) / message_bits;
    assert_eq!(frame_stream.frame_count, expected_frames);
    assert_eq!(frame_stream.frames_bitstream.len(), expected_frames * protocol.frame_layout.frame_bits());

    let sync_len = protocol.frame_layout.sync_symbols * 2;
    let target_len = protocol.frame_layout.target_id_symbols * 2;
    let command_len = protocol.frame_layout.command_type_symbols * 2;
    let frame_size = protocol.frame_layout.frame_bits();

    for frame_idx in 0..expected_frames {
        let start = frame_idx * frame_size;
        let end = start + frame_size;
        let frame = &frame_stream.frames_bitstream[start..end];

        let command_start = sync_len + target_len;
        let command_end = command_start + command_len;
        let command_bits = &frame[command_start..command_end];
        let expected_command = protocol.command_opcode
            | ((frame_idx as u32) << protocol.current_frame_shift)
            | ((expected_frames as u32) << protocol.total_frames_shift);
        let expected_command_bits = int_to_bitstream(expected_command as u64, command_len);
        assert_eq!(command_bits, expected_command_bits.as_slice(), "frame {frame_idx} command word mismatch");
    }

    assert!(logger.entries().len() >= expected_frames);
}
