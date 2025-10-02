use chimera_web::{run_pipeline, FramePreset, SimulationInput};

#[test]
fn pipeline_runs_with_defaults() {
    let mut input = SimulationInput::default();
    input.plaintext = "Test message".into();
    let output = run_pipeline(input);

    assert_eq!(output.report.post_fec_errors, 0);
    assert_eq!(output.report.recovered_message, "Test message");
    let audio = output
        .diagnostics
        .modulation_audio
        .expect("modulation audio absent");
    assert_eq!(audio.sample_rate, SimulationInput::default().sample_rate);
    assert!(!audio.noisy.is_empty());
}

#[test]
fn pipeline_respects_preset_configuration() {
    let mut input = SimulationInput::with_preset(FramePreset::DeepSpaceProbe);
    input.plaintext = "Probe telemetry".into();
    let expected_sample_rate = input.sample_rate;
    let output = run_pipeline(input);

    assert_eq!(output.report.recovered_message, "Probe telemetry");
    assert_eq!(output.report.post_fec_errors, 0);
    assert_eq!(expected_sample_rate, 48_000);
    assert!(output
        .diagnostics
        .modulation_audio
        .as_ref()
        .map(|audio| !audio.clean.is_empty())
        .unwrap_or(false));
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
