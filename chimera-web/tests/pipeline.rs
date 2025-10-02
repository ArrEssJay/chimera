use chimera_web::model::FIXED_SAMPLE_RATE;
use chimera_web::{run_pipeline, FramePreset, SimulationInput};

#[test]
fn pipeline_runs_with_defaults() {
    let input = SimulationInput {
        plaintext: "Test message".into(),
        ..Default::default()
    };
    let output = run_pipeline(input);

    assert_eq!(output.report.post_fec_errors, 0);
    assert_eq!(output.report.recovered_message, "Test message");
    let audio = output
        .diagnostics
        .modulation_audio
        .expect("modulation audio absent");
    assert_eq!(audio.sample_rate, FIXED_SAMPLE_RATE);
    assert!(!audio.noisy.is_empty());
    assert!(!output.diagnostics.tx_symbols_i.is_empty());
    assert!(!output.diagnostics.tx_symbols_q.is_empty());
    assert!(!output.diagnostics.clean_baseband.is_empty());
    assert!(!output.diagnostics.noisy_baseband.is_empty());
    assert!(!output
        .diagnostics
        .demodulation
        .received_symbols_i
        .is_empty());
    assert!(!output
        .diagnostics
        .demodulation
        .received_symbols_q
        .is_empty());
}

#[test]
fn pipeline_respects_preset_configuration() {
    let mut input = SimulationInput::with_preset(FramePreset::DeepSpaceProbe);
    input.plaintext = "Probe telemetry".into();
    let output = run_pipeline(input);

    assert_eq!(output.report.recovered_message, "Probe telemetry");
    assert_eq!(output.report.post_fec_errors, 0);
    assert!(output
        .diagnostics
        .modulation_audio
        .as_ref()
        .map(|audio| !audio.clean.is_empty())
        .unwrap_or(false));
    assert_eq!(output.diagnostics.tx_bits.len() % 2, 0);
}

#[test]
fn burst_telemetry_layout_matches_spec() {
    let protocol = FramePreset::BurstTelemetry.protocol_config();
    let layout = protocol.frame_layout;

    assert_eq!(layout.total_symbols, 96);
    assert_eq!(layout.sync_symbols, 24);
    assert_eq!(layout.data_payload_symbols, 40);
    assert_eq!(layout.ecc_symbols, 8);
}
