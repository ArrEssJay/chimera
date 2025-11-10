//! Chimera core library
//!
//! This crate provides a Rust-native implementation of the Raman Whisper
//! modulation and decoding pipeline. The modules are organized to mirror
//! the former Python reference implementation, but expose a fully typed
//! and testable API.

pub mod config;
pub mod decoder;
pub mod diagnostics;
pub mod encoder;
pub mod errors;
pub mod external_audio;
pub mod ldpc;
pub mod streaming;
pub mod thz_carriers;
pub mod utils;

use std::f64::consts::TAU;

use config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use diagnostics::{DiagnosticsBundle, ModulationAudio, SimulationReport};
use ldpc::LDPCSuite;
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::StandardNormal;

/// High-level handle returned by `run_simulation`.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
/// Now uses streaming encoder/decoder internally.
pub fn run_simulation(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    
    // Use streaming encoder
    let payload_bits = utils::string_to_bitstream(&sim.plaintext_source);
    let mut encoder = encoder::StreamingFrameEncoder::new(
        &payload_bits,
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    // Generate all symbols
    let total_symbols = protocol.frame_layout.total_symbols * encoder.total_frames;
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(total_symbols);
    
    // Apply channel effects (attenuation + AWGN)
    let mut rng = match sim.rng_seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };
    
    let signal_power = 1.0; // QPSK normalized power
    let channel = utils::ChannelParams::from_db(sim.snr_db, sim.link_loss_db, signal_power);
    
    let normal = StandardNormal;
    let mut rx_symbols = Vec::with_capacity(tx_symbols.len());
    
    for tx_symbol in &tx_symbols {
        let attenuated = tx_symbol * channel.attenuation_factor;
        let noise_i: f64 = rng.sample::<f64, _>(normal) * channel.noise_std;
        let noise_q: f64 = rng.sample::<f64, _>(normal) * channel.noise_std;
        let rx_symbol = Complex64::new(
            attenuated.re + noise_i,
            attenuated.im + noise_q
        );
        rx_symbols.push(rx_symbol);
    }
    
    // Use streaming decoder
    let mut decoder = decoder::StreamingSymbolDecoder::new(
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    let (_, _, _, _, diagnostics) = decoder.process_symbols(&rx_symbols);
    let decoded_payload = decoder.get_decoded_payload();
    let recovered_bytes = utils::pack_bits(&decoded_payload);
    let recovered_message = String::from_utf8_lossy(&recovered_bytes)
        .trim_end_matches('\u{0}')
        .to_string();
    
    // Calculate BER
    let pre_fec_errors = 0; // TODO: track in streaming decoder
    let pre_fec_ber = 0.0;
    
    // Ensure both vectors are same length for comparison
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

    // Generate audio for diagnostics
    let samples_per_symbol = usize::max(1, SimulationConfig::SAMPLE_RATE / protocol.qpsk_symbol_rate);
    
    // Generate clean IQ samples
    let mut clean_iq = Vec::with_capacity(tx_symbols.len() * 2 * samples_per_symbol);
    for symbol in &tx_symbols {
        for _ in 0..samples_per_symbol {
            clean_iq.push(symbol.re);
            clean_iq.push(symbol.im);
        }
    }
    
    // Generate noisy IQ samples
    let mut noisy_iq = Vec::with_capacity(rx_symbols.len() * 2 * samples_per_symbol);
    for symbol in &rx_symbols {
        for _ in 0..samples_per_symbol {
            noisy_iq.push(symbol.re);
            noisy_iq.push(symbol.im);
        }
    }
    
    let modulation_audio = Some(ModulationAudio {
        sample_rate: SimulationConfig::SAMPLE_RATE,
        carrier_freq_hz: protocol.carrier_freq_hz,
        clean: iq_to_audio(&clean_iq, SimulationConfig::SAMPLE_RATE, protocol.carrier_freq_hz),
        noisy: iq_to_audio(&noisy_iq, SimulationConfig::SAMPLE_RATE, protocol.carrier_freq_hz),
    });
    
    // Build QPSK bitstream for diagnostics
    let mut qpsk_bitstream = Vec::with_capacity(tx_symbols.len() * 2);
    for symbol in &tx_symbols {
        let bits = decoder::demodulate_qpsk_symbol(*symbol);
        qpsk_bitstream.push(bits[0]);
        qpsk_bitstream.push(bits[1]);
    }

    let diagnostics_bundle = DiagnosticsBundle {
        encoding_logs: encoder.get_logs().to_vec(),
        decoding_logs: decoder.get_logs().to_vec(),
        demodulation: diagnostics,
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
    
    // Convert symbols to audio using the same function as streaming
    let sample_rate = SimulationConfig::SAMPLE_RATE;
    let symbol_rate = protocol.qpsk_symbol_rate;
    let carrier_freq = protocol.carrier_freq_hz;
    
    symbols_to_carrier_signal(&tx_symbols, sample_rate, symbol_rate, carrier_freq, protocol.enable_qpsk, protocol.enable_fsk)
}

/// Convert QPSK symbols to modulated audio carrier
fn symbols_to_carrier_signal(
    symbols: &[Complex64],
    sample_rate: usize,
    symbol_rate: usize,
    carrier_freq: f64,
    enable_qpsk: bool,
    enable_fsk: bool,
) -> Vec<f32> {
    if sample_rate == 0 || symbols.is_empty() {
        return Vec::new();
    }
    
    let samples_per_symbol = sample_rate / symbol_rate;
    let samples_per_fsk_bit = sample_rate; // FSK at 1 bit/second
    let num_samples = symbols.len() * samples_per_symbol;
    let mut audio = Vec::with_capacity(num_samples);
    
    // Pre-compute QPSK phases for all symbols
    let mut qpsk_phases = Vec::with_capacity(symbols.len());
    for symbol in symbols {
        let phase = symbol.arg();
        qpsk_phases.push(phase);
    }
    
    // Apply low-pass filter to QPSK phases (~20 Hz bandwidth)
    let filter_window = (sample_rate as f64 / 40.0) as usize;
    let mut filtered_phases = vec![0.0; qpsk_phases.len()];
    for i in 0..qpsk_phases.len() {
        let start = i.saturating_sub(filter_window / 2);
        let end = (i + filter_window / 2 + 1).min(qpsk_phases.len());
        let sum: f64 = qpsk_phases[start..end].iter().sum();
        filtered_phases[i] = sum / (end - start) as f64;
    }
    
    // Generate FSK bit pattern
    let num_fsk_bits = (num_samples + samples_per_fsk_bit - 1) / samples_per_fsk_bit;
    let mut fsk_bits = Vec::with_capacity(num_fsk_bits);
    for i in 0..num_fsk_bits {
        fsk_bits.push((i % 2) as u8);
    }
    
    // Phase accumulator for carrier
    let mut carrier_phase = 0.0f64;
    
    // Generate audio samples
    for sample_idx in 0..num_samples {
        // FSK frequency (if enabled)
        let fsk_bit_idx = sample_idx / samples_per_fsk_bit;
        let fsk_freq = if enable_fsk && fsk_bit_idx < fsk_bits.len() && fsk_bits[fsk_bit_idx] == 1 {
            12001.0
        } else if enable_fsk {
            11999.0
        } else {
            carrier_freq
        };
        
        carrier_phase += TAU * fsk_freq / sample_rate as f64;
        
        // QPSK phase (if enabled)
        let symbol_idx = sample_idx / samples_per_symbol;
        let qpsk_phase = if enable_qpsk && symbol_idx < filtered_phases.len() {
            filtered_phases[symbol_idx]
        } else {
            0.0
        };
        
        let total_phase = carrier_phase + qpsk_phase;
        audio.push(total_phase.sin() as f32);
        
        if carrier_phase > TAU {
            carrier_phase -= TAU;
        }
    }
    
    normalize_audio(&mut audio);
    audio
}

fn iq_to_audio(iq: &[f64], sample_rate: usize, carrier_freq_hz: f64) -> Vec<f32> {
    if sample_rate == 0 || iq.len() < 2 {
        return Vec::new();
    }

    let dt = 1.0 / sample_rate as f64;
    let mut t = 0.0_f64;
    let mut audio = Vec::with_capacity(iq.len() / 2);

    for chunk in iq.chunks_exact(2) {
        let i = chunk[0];
        let q = chunk[1];
        let angle = TAU * carrier_freq_hz * t;
        let sample = i * angle.cos() - q * angle.sin();
        audio.push(sample as f32);
        t += dt;
    }

    normalize_audio(&mut audio);
    audio
}

fn normalize_audio(samples: &mut [f32]) {
    let mut max_amp = 0.0_f32;
    for &value in samples.iter() {
        max_amp = max_amp.max(value.abs());
    }

    if max_amp > 1.0 {
        let scale = 1.0 / max_amp;
        for value in samples.iter_mut() {
            *value *= scale;
        }
    }
}