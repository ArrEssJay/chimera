use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::run_simulation;

// Note: Tests that relied on batch mode functions (build_frame_stream, generate_modulated_signal,
// demodulate_and_decode) have been removed as we now only support streaming mode.
// The run_simulation function internally uses streaming encoder/decoder.

#[test]
fn run_simulation_emits_audio_waveforms() {
    let protocol = ProtocolConfig::default();
    let ldpc_cfg = LDPCConfig::default();
    let sim = SimulationConfig::default();

    let output = run_simulation(&sim, &protocol, &ldpc_cfg);

    let audio = output.diagnostics.modulation_audio.as_ref()
        .expect("simulation should produce modulation audio");

    assert!(!audio.clean.is_empty(), "clean audio should be non-empty");
    assert!(!audio.noisy.is_empty(), "noisy audio should be non-empty");
    assert_eq!(audio.sample_rate, SimulationConfig::SAMPLE_RATE);
}

#[test]
fn run_simulation_produces_valid_output() {
    let protocol = ProtocolConfig::default();
    let ldpc_cfg = LDPCConfig::default();
    let mut sim = SimulationConfig::default();
    sim.snr_db = 15.0; // Use high SNR for reliable sync detection
    sim.plaintext_source = "Test".into();

    let output = run_simulation(&sim, &protocol, &ldpc_cfg);

    // Should produce TX symbols
    assert!(!output.diagnostics.tx_symbols_i.is_empty(), "Should produce TX symbols (I component)");
    assert!(!output.diagnostics.tx_symbols_q.is_empty(), "Should produce TX symbols (Q component)");
    
    // Should produce audio
    assert!(output.diagnostics.modulation_audio.is_some(), "Should produce audio");
    
    // Should attempt to recover the message (even if imperfect at this SNR)
    assert!(!output.report.recovered_message.is_empty(), "Should recover some message");
}
