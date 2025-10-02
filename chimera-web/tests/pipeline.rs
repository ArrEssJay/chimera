use chimera_web::model::{run_pipeline, SimulationInput};

#[test]
fn pipeline_runs_with_defaults() {
    let mut input = SimulationInput::default();
    input.plaintext = "Test message".into();
    let output = run_pipeline(input);

    assert_eq!(output.report.post_fec_errors, 0);
    assert_eq!(output.report.recovered_message, "Test message");
}
