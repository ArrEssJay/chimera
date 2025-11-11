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
use rand::rngs::StdRng;
use rand::SeedableRng;

/// High-level handle returned by `run_simulation`.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
/// Signal path: encode → modulate → channel → demodulate → decode
pub fn run_simulation(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    use signal_processing::modulation::{ModulationConfig, symbols_to_carrier_signal};
    use signal_processing::demodulation::{DemodulationConfig, audio_to_symbols};
    
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    
    // STEP 1: ENCODE - Convert plaintext to QPSK symbols
    let payload_bits = utils::string_to_bitstream(&sim.plaintext_source);
    let mut encoder = encoder::StreamingFrameEncoder::new(
        &payload_bits,
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    let total_symbols = protocol.frame_layout.total_symbols * encoder.total_frames;
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(total_symbols);
    
    // STEP 2: MODULATE - Convert symbols to audio carrier
    let mod_config = ModulationConfig {
        sample_rate: SimulationConfig::SAMPLE_RATE,
        symbol_rate: protocol.qpsk_symbol_rate,
        carrier_freq: protocol.carrier_freq_hz,
        enable_qpsk: protocol.enable_qpsk,
        enable_fsk: protocol.enable_fsk,
    };
    let clean_audio = symbols_to_carrier_signal(&tx_symbols, &mod_config);
    
    // STEP 3: CHANNEL - Apply attenuation and noise to audio
    let mut rng = match sim.rng_seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };
    
    let channel_params = utils::ChannelParams::from_db(
        sim.snr_db, 
        sim.link_loss_db, 
        1.0  // Audio signal power (normalized)
    );
    
    let mut noisy_audio = clean_audio.clone();
    // Apply attenuation
    for sample in noisy_audio.iter_mut() {
        *sample *= channel_params.attenuation_factor as f32;
    }
    // Apply AWGN
    noisy_audio = channel::apply_audio_noise(&noisy_audio, channel_params.noise_std, &mut rng);
    
    // STEP 4: DEMODULATE - Extract symbols from noisy audio with carrier recovery
    let demod_config = DemodulationConfig {
        sample_rate: SimulationConfig::SAMPLE_RATE,
        symbol_rate: protocol.qpsk_symbol_rate,
        carrier_freq: protocol.carrier_freq_hz,
    };
    let rx_symbols = audio_to_symbols(&noisy_audio, &demod_config);
    
    // STEP 5: DECODE - Recover payload from symbols
    let mut decoder = decoder::StreamingSymbolDecoder::new(
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    let (_, _, _, _, demod_diagnostics) = decoder.process_symbols(&rx_symbols);
    let decoded_payload = decoder.get_decoded_payload();
    let recovered_bytes = utils::pack_bits(&decoded_payload);
    let recovered_message = String::from_utf8_lossy(&recovered_bytes)
        .trim_end_matches('\u{0}')
        .to_string();
    
    // Calculate BER
    let pre_fec_errors = 0; // TODO: track in streaming decoder
    let pre_fec_ber = 0.0;
    
    let comparison_length = decoded_payload.len().min(payload_bits.len());
    let post_fec_errors = if comparison_length > 0 {
        decoded_payload[..comparison_length]
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

    // Generate diagnostic data
    let modulation_audio = Some(ModulationAudio {
        sample_rate: SimulationConfig::SAMPLE_RATE,
        carrier_freq_hz: protocol.carrier_freq_hz,
        clean: clean_audio,
        noisy: noisy_audio,
    });
    
    // Build QPSK bitstream for diagnostics
    let mut qpsk_bitstream = Vec::with_capacity(tx_symbols.len() * 2);
    for symbol in &tx_symbols {
        let bits = decoder::demodulate_qpsk_symbol(*symbol);
        qpsk_bitstream.push(bits[0]);
        qpsk_bitstream.push(bits[1]);
    }
    
    // Generate IQ baseband for diagnostics (from symbols, not audio)
    let samples_per_symbol = usize::max(1, SimulationConfig::SAMPLE_RATE / protocol.qpsk_symbol_rate);
    let mut clean_iq = Vec::with_capacity(tx_symbols.len() * 2 * samples_per_symbol);
    for symbol in &tx_symbols {
        for _ in 0..samples_per_symbol {
            clean_iq.push(symbol.re);
            clean_iq.push(symbol.im);
        }
    }
    
    let mut noisy_iq = Vec::with_capacity(rx_symbols.len() * 2 * samples_per_symbol);
    for symbol in &rx_symbols {
        for _ in 0..samples_per_symbol {
            noisy_iq.push(symbol.re);
            noisy_iq.push(symbol.im);
        }
    }

    let diagnostics_bundle = DiagnosticsBundle {
        encoding_logs: encoder.get_logs().to_vec(),
        decoding_logs: decoder.get_logs().to_vec(),
        demodulation: demod_diagnostics,
        modulation_audio,
        tx_bits: qpsk_bitstream,
        tx_symbols_i: tx_symbols.iter().map(|c| c.re).collect(),
        tx_symbols_q: tx_symbols.iter().map(|c| c.im).collect(),
        clean_baseband: clean_iq,
        noisy_baseband: noisy_iq,
        frames: Vec::new(), // TODO: extract from streaming encoder
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