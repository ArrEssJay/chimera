//! Chimera core library
//!
//! This crate provides a Rust-native implementation of the Raman Whisper
//! modulation and decoding pipeline. The modules are organized to mirror
//! the former Python reference implementation, but expose a fully typed
//! and testable API.

pub mod audio_generator;
pub mod channel;
pub mod config;
pub mod decoder;
pub mod diagnostics;
pub mod encoder;
pub mod errors;
pub mod external_audio;
pub mod ldpc;
pub mod pipeline;
pub mod processor;
pub mod protocol;
pub mod signal_processing;
pub mod thz_carriers;
pub mod utils;

// Re-export the canonical processor interface
pub use processor::{ChimeraProcessor, ProcessorConfig, BatchResult};

use config::{LDPCConfig, InternalProtocolConfig, UserSimulationConfig};
use diagnostics::{DiagnosticsBundle, ModulationAudio, SimulationReport};
use ldpc::LDPCSuite;

/// High-level handle returned by `run_simulation`.
/// 
/// DEPRECATED: Use ChimeraProcessor::process_batch() instead.
/// This function is kept for backward compatibility with existing tests.
#[deprecated(since = "0.2.0", note = "Use ChimeraProcessor::process_batch() instead")]
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
/// Signal path: encode → modulate → channel → demodulate → decode
/// 
/// DEPRECATED: Use ChimeraProcessor::process_batch() instead.
/// This function is kept for backward compatibility with existing tests.
#[deprecated(since = "0.2.0", note = "Use ChimeraProcessor::process_batch() instead")]
pub fn run_simulation(
    sim: &UserSimulationConfig,
    protocol: &InternalProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    use pipeline::RealtimePipeline;
    
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    
    // Create unified pipeline - it will use sim.message internally
    let mut pipeline = RealtimePipeline::new(sim.clone(), protocol.clone(), ldpc.clone());
    pipeline.set_modulation_mode(true); // Active mode for data transmission
    
    // Calculate expected number of frames
    let payload_bits = utils::string_to_bitstream(&sim.message);
    let message_bits = ldpc_suite.matrices.message_bits;
    let total_frames = if payload_bits.is_empty() {
        1
    } else {
        payload_bits.len().div_ceil(message_bits)
    };
    
    // Collect all audio and the final decoded output
    let mut all_audio = Vec::new();
    let mut final_output = None;
    let mut all_pre_channel = Vec::new();
    let mut all_post_channel = Vec::new();
    
    // Process chunks until we've completed all frames
    // At 16 symbols/sec, each frame is 128 symbols = 8 seconds
    // Process enough iterations to encode, transmit, and decode all frames
    let iterations_per_frame = 20; // Extra margin for sync and processing
    let max_iterations = total_frames * iterations_per_frame;
    
    let dummy_input = b""; // Input is unused - encoder uses config.plaintext_source
    
    for i in 0..max_iterations {
        let output = pipeline.process_chunk(dummy_input);
        
        // Collect audio samples
        all_audio.extend_from_slice(&output.audio_samples);
        
        // Store diagnostics
        all_pre_channel.push(output.pre_channel.clone());
        all_post_channel.push(output.post_channel.clone());
        
        // Keep the last output
        final_output = Some(output);
        
        // Check if we've decoded the complete message
        if let Some(ref out) = final_output {
            let decoded_len = out.decoded_text.trim_end_matches('\u{0}').len();
            if decoded_len >= sim.message.len() && i > total_frames {
                break;
            }
        }
    }
    
    let output = final_output.expect("Pipeline should produce output");
    
    // Extract results from final pipeline output
    let decoded_payload = output.decoded_data;
    let recovered_message = output.decoded_text
        .trim_end_matches('\u{0}')
        .to_string();
    
    // Calculate BER - compare original bits to decoded bits
    // Use the actual payload length (bytes), not the bit array length
    let original_bytes = sim.message.as_bytes();
    let comparison_length = decoded_payload.len().min(original_bytes.len());
    
    let post_fec_errors = if comparison_length > 0 {
        let decoded_bits: Vec<u8> = decoded_payload[..comparison_length].iter()
            .flat_map(|byte| {
                (0..8).map(move |i| ((byte >> (7 - i)) & 1) as u8)
            })
            .collect();
        
        let original_bits: Vec<u8> = original_bytes[..comparison_length].iter()
            .flat_map(|byte| {
                (0..8).map(move |i| ((byte >> (7 - i)) & 1) as u8)
            })
            .collect();
        
        decoded_bits.iter()
            .zip(original_bits.iter())
            .filter(|(rx, tx)| rx != tx)
            .count()
    } else {
        0
    };
    
    let post_fec_ber = if comparison_length > 0 {
        post_fec_errors as f64 / (comparison_length * 8) as f64
    } else {
        0.0
    };
    
    // Estimate pre-FEC BER from EVM
    let pre_fec_ber = if let Some(last_post) = all_post_channel.last() {
        let evm = last_post.evm_percent;
        if evm < 5.0 {
            0.0
        } else {
            (evm / 100.0 * 0.5).min(0.5) as f64
        }
    } else {
        post_fec_ber
    };
    
    let pre_fec_errors = (pre_fec_ber * payload_bits.len() as f64) as usize;
    
    let report = SimulationReport {
        pre_fec_errors,
        pre_fec_ber,
        post_fec_errors,
        post_fec_ber,
        recovered_message: recovered_message.clone(),
    };

    // Generate diagnostic data from pipeline output
    let modulation_audio = Some(ModulationAudio {
        sample_rate: crate::config::SystemConfig::SAMPLE_RATE,
        carrier_freq_hz: protocol.carrier_freq_hz,
        clean: all_audio.clone(),
        noisy: all_audio.clone(),
    });
    
    // Aggregate TX symbols from all pre_channel diagnostics
    let mut tx_symbols_i = Vec::new();
    let mut tx_symbols_q = Vec::new();
    for pre in &all_pre_channel {
        tx_symbols_i.extend(pre.tx_constellation_i.iter().map(|&v| v as f64));
        tx_symbols_q.extend(pre.tx_constellation_q.iter().map(|&v| v as f64));
    }
    
    let diagnostics_bundle = DiagnosticsBundle {
        encoding_logs: Vec::new(),
        decoding_logs: Vec::new(),
        demodulation: diagnostics::DemodulationDiagnostics::default(),
        modulation_audio,
        tx_bits: payload_bits.clone(),
        tx_symbols_i,
        tx_symbols_q,
        clean_baseband: Vec::new(),
        noisy_baseband: Vec::new(),
        frames: Vec::new(),
    };

    SimulationOutput {
        report,
        diagnostics: diagnostics_bundle,
        ldpc: ldpc_suite,
    }
}

/// Generate audio for a message without real-time streaming delays
/// Returns the complete audio waveform for the entire message
/// 
/// DEPRECATED: Use ChimeraProcessor::process_batch() instead and access the .audio field.
#[deprecated(since = "0.2.0", note = "Use ChimeraProcessor::process_batch() instead")]
pub fn generate_audio_batch(
    message: &str,
    protocol: &InternalProtocolConfig,
    ldpc: &LDPCConfig,
) -> Vec<f32> {
    use signal_processing::modulation::{ModulationConfig, symbols_to_carrier_signal};
    
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    
    // Encode message to symbols
    let payload_bits = utils::string_to_bitstream(message);
    let mut encoder = encoder::StreamingFrameEncoder::new(
        &payload_bits,
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    // Generate all symbols at once
    let total_symbols = protocol.frame_layout.total_symbols * encoder.total_frames;
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(total_symbols);
    
    // Convert symbols to audio using modular signal processing
    let mod_config = ModulationConfig {
        sample_rate: crate::config::SystemConfig::SAMPLE_RATE,
        symbol_rate: protocol.qpsk_symbol_rate,
        carrier_freq: protocol.carrier_freq_hz,
    };
    
    symbols_to_carrier_signal(&tx_symbols, &mod_config)
}