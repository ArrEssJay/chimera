//! Chimera core library
//!
//! This crate provides a Rust-native implementation of the Raman Whisper
//! modulation and decoding pipeline. The modules are organized to mirror
//! the former Python reference implementation, but expose a fully typed
//! and testable API.

pub mod channel;
pub mod config;
pub mod decoder;
pub mod diagnostics;
pub mod encoder;
pub mod errors;
pub mod external_audio;
pub mod ldpc;
pub mod pipeline;
pub mod signal_processing;
pub mod thz_carriers;
pub mod utils;

use config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use diagnostics::{DiagnosticsBundle, ModulationAudio, SimulationReport};
use ldpc::LDPCSuite;

/// High-level handle returned by `run_simulation`.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
/// Signal path: encode → modulate → channel → demodulate → decode
/// 
/// This now uses the unified RealtimePipeline internally, processing all data
/// in a single large chunk for batch mode operation.
pub fn run_simulation(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    use pipeline::RealtimePipeline;
    
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    
    // Create unified pipeline
    let mut pipeline = RealtimePipeline::new(sim.clone(), protocol.clone(), ldpc.clone());
    pipeline.set_modulation_mode(true); // Active mode for data transmission
    
    // For batch processing, process everything at once by calling process_chunk
    // multiple times until we've completed all frames
    let payload_bits = utils::string_to_bitstream(&sim.plaintext_source);
    let message_bits = ldpc_suite.matrices.message_bits;
    let total_frames = if payload_bits.is_empty() {
        1
    } else {
        payload_bits.len().div_ceil(message_bits)
    };
    
    // Process enough chunks to complete all frames
    // At 16 symbols/sec, each frame is 128 symbols = 8 seconds
    // Process 10 seconds per frame to ensure completion
    let chunks_per_frame = 10;
    let total_chunks = total_frames * chunks_per_frame;
    
    let mut final_output = None;
    let test_data = b""; // Unused - encoder uses config.plaintext_source
    
    for _ in 0..total_chunks {
        let output = pipeline.process_chunk(test_data);
        final_output = Some(output);
    }
    
    let output = final_output.expect("Pipeline should produce output");
    
    // Extract results from pipeline output
    let decoded_payload = output.decoded_data;
    let recovered_message = output.decoded_text
        .trim_end_matches('\u{0}')
        .to_string();
    
    // Calculate BER
    let pre_fec_errors = 0; // TODO: track in streaming decoder
    let pre_fec_ber = 0.0;
    
    let comparison_length = decoded_payload.len().min(payload_bits.len());
    let post_fec_errors = if comparison_length > 0 {
        let decoded_bits: Vec<u8> = decoded_payload.iter()
            .flat_map(|byte| {
                (0..8).map(move |i| ((byte >> (7 - i)) & 1) as u8)
            })
            .collect();
        
        decoded_bits[..comparison_length.min(decoded_bits.len())]
            .iter()
            .zip(&payload_bits[..comparison_length])
            .filter(|(rx, tx)| rx != tx)
            .count()
    } else {
        0
    };
    let post_fec_ber = if comparison_length > 0 {
        post_fec_errors as f64 / comparison_length as f64
    } else {
        0.0
    };
    
    let report = SimulationReport {
        pre_fec_errors,
        pre_fec_ber,
        post_fec_errors,
        post_fec_ber,
        recovered_message: recovered_message.clone(),
    };

    // Generate diagnostic data from pipeline output
    let modulation_audio = Some(ModulationAudio {
        sample_rate: SimulationConfig::SAMPLE_RATE,
        carrier_freq_hz: protocol.carrier_freq_hz,
        clean: output.audio_samples.clone(), // In this implementation, we only have final audio
        noisy: output.audio_samples.clone(),
    });
    
    // Build diagnostics from pipeline output
    let tx_symbols_i = output.pre_channel.tx_constellation_i.iter().map(|&v| v as f64).collect();
    let tx_symbols_q = output.pre_channel.tx_constellation_q.iter().map(|&v| v as f64).collect();
    
    let diagnostics_bundle = DiagnosticsBundle {
        encoding_logs: Vec::new(), // Not exposed from pipeline
        decoding_logs: Vec::new(), // Not exposed from pipeline
        demodulation: diagnostics::DemodulationDiagnostics::default(),
        modulation_audio,
        tx_bits: Vec::new(), // Not needed for this interface
        tx_symbols_i,
        tx_symbols_q,
        clean_baseband: Vec::new(), // Not needed
        noisy_baseband: Vec::new(), // Not needed
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
pub fn generate_audio_batch(
    message: &str,
    protocol: &ProtocolConfig,
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
        sample_rate: SimulationConfig::SAMPLE_RATE,
        symbol_rate: protocol.qpsk_symbol_rate,
        carrier_freq: protocol.carrier_freq_hz,
        enable_qpsk: protocol.enable_qpsk,
        enable_fsk: protocol.enable_fsk,
    };
    
    symbols_to_carrier_signal(&tx_symbols, &mod_config)
}