//! Simple decode diagnostic test to understand the actual problem

use chimera_core::config::{UserSimulationConfig, LDPCConfig, InternalProtocolConfig};
use chimera_core::{run_simulation};

#[test]
fn diagnose_decode_failure() {
    // Setup
    let mut sim = UserSimulationConfig::default();
    sim.message = "Test".to_string();
    sim.bypass_thz_simulation = true; // Simplify - no THz effects
    
    let protocol = InternalProtocolConfig::default();
    let ldpc = LDPCConfig::default();
    
    println!("\n=== DECODE DIAGNOSTIC ===");
    println!("Input message: '{}'", sim.message);
    println!("Message bytes: {:?}", sim.message.as_bytes());
    println!("Message length: {} bytes", sim.message.len());
    
    // Run simulation
    let result = run_simulation(&sim, &protocol, &ldpc);
    
    println!("\n=== RESULTS ===");
    println!("Recovered: '{}'", result.report.recovered_message);
    println!("Recovered (trimmed): '{}'", result.report.recovered_message.trim_end_matches('\0'));
    println!("Recovered bytes: {:?}", result.report.recovered_message.as_bytes());
    println!("Pre-FEC BER: {:.6}", result.report.pre_fec_ber);
    println!("Post-FEC BER: {:.6}", result.report.post_fec_ber);
    println!("Pre-FEC errors: {}", result.report.pre_fec_errors);
    println!("Post-FEC errors: {}", result.report.post_fec_errors);
    
    println!("\n=== ANALYSIS ===");
    let trimmed = result.report.recovered_message.trim_end_matches('\0');
    println!("Match: {}", trimmed == sim.message);
    println!("Length match: {} vs {}", trimmed.len(), sim.message.len());
    
    // Character by character comparison
    if !trimmed.is_empty() {
        println!("\nCharacter comparison:");
        for (i, (sent, recv)) in sim.message.chars().zip(trimmed.chars()).enumerate() {
            let match_str = if sent == recv { "✓" } else { "✗" };
            println!("  [{}] sent: '{}' (0x{:02X}) recv: '{}' (0x{:02X}) {}", 
                     i, sent, sent as u8, recv, recv as u8, match_str);
        }
    }
    
    // This test is diagnostic - it always "passes" but shows what's wrong
    assert!(true, "Diagnostic test - check output above");
}
