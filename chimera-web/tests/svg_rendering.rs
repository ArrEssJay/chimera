use chimera_web::{run_pipeline, SimulationInput};

#[test]
fn test_svg_generation_with_real_data() {
    let input = SimulationInput {
        plaintext: "Test".into(),
        ..Default::default()
    };
    let output = run_pipeline(input);
    
    // Verify we have data
    assert!(!output.diagnostics.tx_symbols_i.is_empty());
    assert!(!output.diagnostics.tx_symbols_q.is_empty());
    assert!(!output.diagnostics.demodulation.received_symbols_i.is_empty());
    assert!(!output.diagnostics.demodulation.received_symbols_q.is_empty());
    
    // Verify all values are finite
    for &val in &output.diagnostics.tx_symbols_i {
        assert!(val.is_finite(), "TX I value {} is not finite", val);
    }
    for &val in &output.diagnostics.tx_symbols_q {
        assert!(val.is_finite(), "TX Q value {} is not finite", val);
    }
    for &val in &output.diagnostics.demodulation.received_symbols_i {
        assert!(val.is_finite(), "RX I value {} is not finite", val);
    }
    for &val in &output.diagnostics.demodulation.received_symbols_q {
        assert!(val.is_finite(), "RX Q value {} is not finite", val);
    }
    
    println!("All constellation values are finite and valid");
    println!("TX symbols: {} points", output.diagnostics.tx_symbols_i.len());
    println!("RX symbols: {} points", output.diagnostics.demodulation.received_symbols_i.len());
}
