use chimera_web::{run_pipeline, SimulationInput};

#[test]
fn inspect_constellation_values() {
    let input = SimulationInput {
        plaintext: "Test".into(),
        ..Default::default()
    };
    let output = run_pipeline(input);
    
    println!("TX symbols I count: {}", output.diagnostics.tx_symbols_i.len());
    println!("TX symbols Q count: {}", output.diagnostics.tx_symbols_q.len());
    println!("RX symbols I count: {}", output.diagnostics.demodulation.received_symbols_i.len());
    println!("RX symbols Q count: {}", output.diagnostics.demodulation.received_symbols_q.len());
    
    if !output.diagnostics.tx_symbols_i.is_empty() {
        println!("\nFirst 10 TX I values:");
        for (i, &val) in output.diagnostics.tx_symbols_i.iter().take(10).enumerate() {
            println!("  [{}] = {} (finite: {})", i, val, val.is_finite());
        }
    }
    
    if !output.diagnostics.tx_symbols_q.is_empty() {
        println!("\nFirst 10 TX Q values:");
        for (i, &val) in output.diagnostics.tx_symbols_q.iter().take(10).enumerate() {
            println!("  [{}] = {} (finite: {})", i, val, val.is_finite());
        }
    }
    
    if !output.diagnostics.demodulation.received_symbols_i.is_empty() {
        println!("\nFirst 10 RX I values:");
        for (i, &val) in output.diagnostics.demodulation.received_symbols_i.iter().take(10).enumerate() {
            println!("  [{}] = {} (finite: {})", i, val, val.is_finite());
        }
    }
}
