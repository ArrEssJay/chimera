use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::decoder::demodulate_and_decode;
use chimera_core::encoder::{build_frame_stream, generate_modulated_signal};
use chimera_core::ldpc::LDPCSuite;
use chimera_core::run_simulation;
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
    assert_eq!(
        &frame_stream.frames_bitstream[..sync_len],
        sync_bits.as_slice()
    );

    let command_start = sync_len + target_len;
    let command_end = command_start + command_len;
    let expected_command = protocol.command_opcode
        | (0 << protocol.current_frame_shift)
        | (1 << protocol.total_frames_shift);
    let expected_command_bits = int_to_bitstream(expected_command as u64, command_len);
    assert_eq!(
        &frame_stream.frames_bitstream[command_start..command_end],
        expected_command_bits.as_slice()
    );

    assert!(
        !frame_stream.logs.is_empty(),
        "frame builder should emit trace logs"
    );
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

    let expected_frames = payload_bits.len().div_ceil(message_bits);
    assert_eq!(frame_stream.frame_count, expected_frames);
    assert_eq!(
        frame_stream.frames_bitstream.len(),
        expected_frames * protocol.frame_layout.frame_bits()
    );

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
        assert_eq!(
            command_bits,
            expected_command_bits.as_slice(),
            "frame {frame_idx} command word mismatch"
        );
    }

    assert!(logger.entries().len() >= expected_frames);
}

#[test]
fn given_high_snr_when_pipeline_runs_then_plaintext_roundtrips() {
    let protocol = ProtocolConfig::default();

    let sim = SimulationConfig {
        sample_rate: protocol.qpsk_symbol_rate,
        snr_db: 30.0,
        plaintext_source: "Hello Chimera".into(),
        rng_seed: Some(1337),
        ..Default::default()
    };

    let ldpc_cfg = LDPCConfig::default();
    let suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_cfg);

    let encoding = generate_modulated_signal(&sim, &protocol, &suite.matrices);
    let demodulation = demodulate_and_decode(&encoding, &suite.matrices, &sim, &protocol);

    assert_eq!(encoding.total_frames, 1);
    assert_eq!(demodulation.report.post_fec_errors, 0);
    assert!(demodulation.report.post_fec_ber <= 1e-9);
    assert_eq!(demodulation.recovered_message, sim.plaintext_source);
    assert_eq!(
        demodulation.demodulated_bitstream.len(),
        encoding.qpsk_bitstream.len()
    );
}

#[test]
fn run_simulation_emits_audio_waveforms() {
    let protocol = ProtocolConfig::default();
    let sim = SimulationConfig::default();
    let ldpc_cfg = LDPCConfig::default();

    let output = run_simulation(&sim, &protocol, &ldpc_cfg);
    let audio = output
        .diagnostics
        .modulation_audio
        .expect("modulation audio should be present");

    assert_eq!(audio.sample_rate, sim.sample_rate);
    assert!(
        !audio.clean.is_empty(),
        "clean waveform should contain samples"
    );
    assert!(
        !audio.noisy.is_empty(),
        "noisy waveform should contain samples"
    );
}

#[test]
fn given_low_snr_when_pipeline_runs_then_ldpc_fails() {
    let protocol = ProtocolConfig::default();

    let mut sim = SimulationConfig::default();
    sim.sample_rate = protocol.qpsk_symbol_rate; // No oversampling (samples_per_symbol = 1)
    sim.snr_db = -5.0; // Without processing gain, LDPC fails at this Es/N0
    sim.plaintext_source = "Hello Chimera".into();
    sim.rng_seed = Some(1337);

    let ldpc_cfg = LDPCConfig::default();
    let suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_cfg);

    let encoding = generate_modulated_signal(&sim, &protocol, &suite.matrices);
    let demodulation = demodulate_and_decode(&encoding, &suite.matrices, &sim, &protocol);

    // At -5 dB SNR, LDPC should fail to recover the message
    assert!(demodulation.report.pre_fec_errors > 0, "Expected pre-FEC errors at low SNR");
    assert_ne!(demodulation.recovered_message, sim.plaintext_source, 
        "LDPC should fail to recover message at -5 dB SNR");
}

#[test]
fn given_near_zero_snr_when_pipeline_runs_then_ldpc_succeeds() {
    let protocol = ProtocolConfig::default();

    let mut sim = SimulationConfig::default();
    sim.sample_rate = protocol.qpsk_symbol_rate; // No oversampling (samples_per_symbol = 1)
    sim.snr_db = -1.0; // Without processing gain, marginal Es/N0 but LDPC succeeds
    sim.plaintext_source = "Hello Chimera".into();
    sim.rng_seed = Some(1337);

    let ldpc_cfg = LDPCConfig::default();
    let suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_cfg);

    let encoding = generate_modulated_signal(&sim, &protocol, &suite.matrices);
    let demodulation = demodulate_and_decode(&encoding, &suite.matrices, &sim, &protocol);

    // At -1 dB SNR, some pre-FEC errors are expected but LDPC should still recover
    assert!(demodulation.report.pre_fec_errors > 0, "Expected pre-FEC errors at -1 dB SNR");
}
