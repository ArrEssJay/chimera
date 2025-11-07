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
pub mod ldpc;
pub mod node_graph;
pub mod utils;

use std::f64::consts::TAU;

use config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use decoder::demodulate_and_decode;
use diagnostics::{DiagnosticsBundle, ModulationAudio, SimulationReport};
use encoder::{generate_modulated_signal, EncodingResult};
use ldpc::LDPCSuite;

/// High-level handle returned by `run_simulation`.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationOutput {
    pub report: SimulationReport,
    pub diagnostics: DiagnosticsBundle,
    pub ldpc: LDPCSuite,
}

/// Execute an end-to-end simulation with the provided configuration set.
pub fn run_simulation(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    ldpc: &LDPCConfig,
) -> SimulationOutput {
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, ldpc);
    let encoding = generate_modulated_signal(sim, protocol, &ldpc_suite.matrices);
    let demodulation = demodulate_and_decode(&encoding, &ldpc_suite.matrices, sim, protocol);

    let modulation_audio =
        build_modulation_audio(&encoding, sim.sample_rate, protocol.carrier_freq_hz);

    let diagnostics = DiagnosticsBundle {
        encoding_logs: encoding.logs.clone(),
        decoding_logs: demodulation.logs.clone(),
        demodulation: demodulation.diagnostics.clone(),
        modulation_audio,
        tx_bits: encoding.qpsk_bitstream.clone(),
        tx_symbols_i: encoding.qpsk_symbols.iter().map(|c| c.re).collect(),
        tx_symbols_q: encoding.qpsk_symbols.iter().map(|c| c.im).collect(),
        clean_baseband: encoding.clean_signal.to_vec(),
        noisy_baseband: encoding.noisy_signal.to_vec(),
        frames: encoding.frame_descriptors.clone(),
    };

    SimulationOutput {
        report: demodulation.report.clone(),
        diagnostics,
        ldpc: ldpc_suite,
    }
}

fn build_modulation_audio(
    encoding: &EncodingResult,
    fallback_sample_rate: usize,
    carrier_freq_hz: f64,
) -> Option<ModulationAudio> {
    let sample_rate = if encoding.sample_rate > 0 {
        encoding.sample_rate
    } else {
        fallback_sample_rate
    };

    if sample_rate == 0 {
        return None;
    }

    let clean = encoding
        .clean_signal
        .as_slice()
        .map(|iq| iq_to_audio(iq, sample_rate, carrier_freq_hz))
        .unwrap_or_default();
    let noisy = encoding
        .noisy_signal
        .as_slice()
        .map(|iq| iq_to_audio(iq, sample_rate, carrier_freq_hz))
        .unwrap_or_default();

    if clean.is_empty() && noisy.is_empty() {
        None
    } else {
        Some(ModulationAudio {
            sample_rate,
            carrier_freq_hz,
            clean,
            noisy,
        })
    }
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
