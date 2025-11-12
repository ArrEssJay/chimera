//! Pipeline Test Harness
//!
//! A comprehensive test binary that runs the full Chimera pipeline with various
//! configurations and provides detailed diagnostics and reporting.
//!
//! Usage:
//!   cargo run --example pipeline_test
//!   cargo run --example pipeline_test -- --config test_simple.toml
//!   cargo run --example pipeline_test -- --all

use chimera_core::config::{ConfigBuilder, UserConfig, LDPCConfig};
use chimera_core::{run_simulation, generate_audio_batch};
use std::path::PathBuf;
use std::fs;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

/// Test result tracking
#[derive(Debug)]
struct TestResult {
    name: String,
    passed: bool,
    message: String,
    pre_fec_ber: f64,
    post_fec_ber: f64,
    recovered_length: usize,
    expected_length: usize,
}

impl TestResult {
    fn print(&self) {
        let status = if self.passed {
            format!("{}✓ PASS{}", GREEN, RESET)
        } else {
            format!("{}✗ FAIL{}", RED, RESET)
        };
        
        println!("\n{}{}{}", BOLD, self.name, RESET);
        println!("  Status: {}", status);
        println!("  {}", self.message);
        println!("  Pre-FEC BER:  {:.6}", self.pre_fec_ber);
        println!("  Post-FEC BER: {:.6}", self.post_fec_ber);
        println!("  Recovered: {}/{} bytes ({:.1}%)", 
                 self.recovered_length, 
                 self.expected_length,
                 (self.recovered_length as f64 / self.expected_length as f64) * 100.0);
    }
}

/// Helper to load test config
fn load_config(name: &str) -> Result<UserConfig, Box<dyn std::error::Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("configs")
        .join(name);
    
    let content = fs::read_to_string(&path)?;
    let config: UserConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Test a configuration file
fn test_config(config_name: &str) -> TestResult {
    let config = match load_config(config_name) {
        Ok(c) => c,
        Err(e) => {
            return TestResult {
                name: config_name.to_string(),
                passed: false,
                message: format!("Failed to load config: {}", e),
                pre_fec_ber: 0.0,
                post_fec_ber: 0.0,
                recovered_length: 0,
                expected_length: 0,
            };
        }
    };
    
    let system = match ConfigBuilder::from_user_config(config.clone()).build() {
        Ok(s) => s,
        Err(e) => {
            return TestResult {
                name: config_name.to_string(),
                passed: false,
                message: format!("Failed to build system config: {}", e),
                pre_fec_ber: 0.0,
                post_fec_ber: 0.0,
                recovered_length: 0,
                expected_length: 0,
            };
        }
    };
    
    let ldpc = LDPCConfig::default();
    let sim = match system.simulation.as_ref() {
        Some(s) => s,
        None => {
            return TestResult {
                name: config_name.to_string(),
                passed: false,
                message: "Config has no simulation section".to_string(),
                pre_fec_ber: 0.0,
                post_fec_ber: 0.0,
                recovered_length: 0,
                expected_length: 0,
            };
        }
    };
    
    let expected_message = &sim.message;
    let result = run_simulation(sim, &system.protocol, &ldpc);
    
    let recovered = result.report.recovered_message.trim_end_matches('\u{0}');
    let recovered_len = recovered.len();
    let expected_len = expected_message.len();
    
    // Determine pass/fail criteria
    let min_recovery_rate = 0.25; // Must recover at least 25% of message
    let max_ber = 0.45; // Post-FEC BER must be < 45%
    
    let recovery_rate = recovered_len as f64 / expected_len as f64;
    let ber_ok = result.report.post_fec_ber <= max_ber;
    let recovery_ok = recovery_rate >= min_recovery_rate;
    
    let passed = ber_ok && recovery_ok;
    
    let message = if passed {
        format!("Pipeline correctly processed message from config")
    } else {
        let mut reasons = Vec::new();
        if !ber_ok {
            reasons.push(format!("BER too high: {:.3} > {:.3}", 
                               result.report.post_fec_ber, max_ber));
        }
        if !recovery_ok {
            reasons.push(format!("Recovery too low: {:.1}% < {:.1}%", 
                               recovery_rate * 100.0, min_recovery_rate * 100.0));
        }
        format!("Issues: {}", reasons.join(", "))
    };
    
    TestResult {
        name: config_name.to_string(),
        passed,
        message,
        pre_fec_ber: result.report.pre_fec_ber,
        post_fec_ber: result.report.post_fec_ber,
        recovered_length: recovered_len,
        expected_length: expected_len,
    }
}

/// Test audio generation
fn test_audio_generation(config_name: &str) -> TestResult {
    let config = match load_config(config_name) {
        Ok(c) => c,
        Err(e) => {
            return TestResult {
                name: format!("{} (audio)", config_name),
                passed: false,
                message: format!("Failed to load config: {}", e),
                pre_fec_ber: 0.0,
                post_fec_ber: 0.0,
                recovered_length: 0,
                expected_length: 0,
            };
        }
    };
    
    let system = match ConfigBuilder::from_user_config(config).build() {
        Ok(s) => s,
        Err(e) => {
            return TestResult {
                name: format!("{} (audio)", config_name),
                passed: false,
                message: format!("Failed to build system config: {}", e),
                pre_fec_ber: 0.0,
                post_fec_ber: 0.0,
                recovered_length: 0,
                expected_length: 0,
            };
        }
    };
    
    let ldpc = LDPCConfig::default();
    let sim = match system.simulation.as_ref() {
        Some(s) => s,
        None => {
            return TestResult {
                name: format!("{} (audio)", config_name),
                passed: false,
                message: "Config has no simulation section".to_string(),
                pre_fec_ber: 0.0,
                post_fec_ber: 0.0,
                recovered_length: 0,
                expected_length: 0,
            };
        }
    };
    
    let audio = generate_audio_batch(&sim.message, &system.protocol, &ldpc);
    
    // Check audio properties
    let has_audio = !audio.is_empty();
    let sufficient_length = audio.len() >= 48000; // At least 1 second
    let valid_amplitude = audio.iter().all(|&s| s >= -1.0 && s <= 1.0);
    let has_signal = audio.iter().any(|&s| s.abs() > 0.01);
    
    // Calculate max amplitude for diagnostics
    let max_amp = audio.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    
    let passed = has_audio && sufficient_length && valid_amplitude && has_signal;
    
    let message = if passed {
        format!("Generated {} samples ({:.2}s) with valid amplitude (max: {:.4})", 
                audio.len(), audio.len() as f32 / 48000.0, max_amp)
    } else {
        let mut issues = Vec::new();
        if !has_audio { issues.push("no audio".to_string()); }
        if !sufficient_length { issues.push("too short".to_string()); }
        if !valid_amplitude { 
            issues.push(format!("invalid amplitude (max: {:.2})", max_amp));
        }
        if !has_signal { issues.push("no signal".to_string()); }
        format!("Audio generation issues: {}", issues.join(", "))
    };
    
    TestResult {
        name: format!("{} (audio)", config_name),
        passed,
        message,
        pre_fec_ber: 0.0,
        post_fec_ber: 0.0,
        recovered_length: audio.len() / 48000, // Duration in seconds as a proxy
        expected_length: (sim.message.len() / 16 * 8).max(1), // Rough estimate: frames * seconds/frame
    }
}

/// Print summary of results
fn print_summary(results: &[TestResult]) {
    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = total - passed;
    
    println!("\n{}{}", BOLD, "=".repeat(70));
    println!("TEST SUMMARY");
    println!("{}{}", "=".repeat(70), RESET);
    
    println!("\nTotal tests:  {}", total);
    println!("{}Passed:       {}{}", GREEN, passed, RESET);
    if failed > 0 {
        println!("{}Failed:       {}{}", RED, failed, RESET);
    }
    println!("Success rate: {:.1}%", (passed as f64 / total as f64) * 100.0);
    
    if failed > 0 {
        println!("\n{}Failed tests:{}", YELLOW, RESET);
        for result in results.iter().filter(|r| !r.passed) {
            println!("  - {}: {}", result.name, result.message);
        }
    }
}

fn main() {
    println!("{}{}Chimera Pipeline Test Harness{}{}\n", BOLD, BLUE, RESET, RESET);
    
    let args: Vec<String> = std::env::args().collect();
    
    // Determine which tests to run
    let test_configs = if args.len() > 1 && args[1] == "--all" {
        vec![
            "test_simple.toml",
            "test_audio_mixing.toml",
            "test_high_noise.toml",
            "test_long_message.toml",
            "test_clean_channel.toml",
            "test_tone_mixing.toml",
        ]
    } else if args.len() > 2 && args[1] == "--config" {
        vec![args[2].as_str()]
    } else {
        // Default test suite
        vec![
            "test_simple.toml",
            "test_clean_channel.toml",
            "test_long_message.toml",
        ]
    };
    
    let mut results = Vec::new();
    
    println!("{}Running pipeline tests...{}\n", BOLD, RESET);
    
    for config_name in &test_configs {
        println!("{}Testing: {}{}", BOLD, config_name, RESET);
        
        // Test 1: Configuration loading and pipeline execution
        let result1 = test_config(config_name);
        result1.print();
        results.push(result1);
        
        // Test 2: Audio generation
        let result2 = test_audio_generation(config_name);
        result2.print();
        results.push(result2);
    }
    
    // Print final summary
    print_summary(&results);
    
    // Exit with appropriate code
    let all_passed = results.iter().all(|r| r.passed);
    std::process::exit(if all_passed { 0 } else { 1 });
}
