//! Demodulation and decoding stage implementations.
use std::f64::consts::FRAC_1_SQRT_2;

use ndarray::Array1;
use num_complex::Complex64;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::diagnostics::{DemodulationDiagnostics, SimulationReport};
use crate::encoder::EncodingResult;
use crate::ldpc::{decode_ldpc, LDPCMatrices};
use crate::utils::{complex_from_interleaved, hex_to_bitstream, pack_bits, LogCollector};

pub struct DemodulationResult {
    pub demodulated_bitstream: Vec<u8>,
    pub decoded_bitstream: Vec<u8>,
    pub recovered_message: String,
    pub diagnostics: DemodulationDiagnostics,
    pub report: SimulationReport,
    pub logs: Vec<String>,
}

impl DemodulationResult {
    pub fn empty() -> Self {
        DemodulationResult {
            demodulated_bitstream: Vec::new(),
            decoded_bitstream: Vec::new(),
            recovered_message: String::new(),
            diagnostics: DemodulationDiagnostics::default(),
            report: SimulationReport::default(),
            logs: Vec::new(),
        }
    }
}

pub fn demodulate_and_decode(
    encoding: &EncodingResult,
    matrices: &LDPCMatrices,
    _sim: &SimulationConfig,
    protocol: &ProtocolConfig,
) -> DemodulationResult {
    let mut logger = LogCollector::new();

    if encoding.qpsk_bitstream.is_empty() {
        logger.log("Encoding bitstream was empty; nothing to demodulate.");
        return DemodulationResult {
            logs: logger.entries().to_vec(),
            ..DemodulationResult::empty()
        };
    }

    let noisy_symbols = baseband_to_symbols(&encoding.noisy_signal);
    let samples_per_symbol = encoding.samples_per_symbol.max(1);
    let symbol_count = noisy_symbols.len() / samples_per_symbol;

    let reference = qpsk_constellation();
    let mut demodulated_bits = Vec::with_capacity(symbol_count * 2);
    let mut decision_symbols = Vec::with_capacity(symbol_count);

    for symbol_idx in 0..symbol_count {
        let start = symbol_idx * samples_per_symbol;
        let end = start + samples_per_symbol;
        let sample_slice = &noisy_symbols[start..end];
        let avg_symbol = sample_slice.iter().copied().sum::<Complex64>()
            / Complex64::new(samples_per_symbol as f64, 0.0);

        let closest_bits = reference
            .iter()
            .map(|(candidate, bits)| {
                let distance = (avg_symbol - *candidate).norm_sqr();
                (bits, candidate, distance)
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            .map(|(bits, _, _)| *bits)
            .expect("reference constellation is non-empty");

    demodulated_bits.extend_from_slice(&closest_bits);
    decision_symbols.push(avg_symbol);
    }

    if demodulated_bits.len() < encoding.qpsk_bitstream.len() {
        demodulated_bits.resize(encoding.qpsk_bitstream.len(), 0);
    } else {
        demodulated_bits.truncate(encoding.qpsk_bitstream.len());
    }

    let pre_fec_errors = demodulated_bits
        .iter()
        .zip(&encoding.qpsk_bitstream)
        .filter(|(rx, tx)| rx != tx)
        .count();
    let pre_fec_ber = pre_fec_errors as f64 / encoding.qpsk_bitstream.len() as f64;
    logger.log(format!("Pre-FEC BER: {:.6} ({} errors).", pre_fec_ber, pre_fec_errors));

    let sync_bit_len = protocol.frame_layout.sync_symbols * 2;
    let sync_bits = hex_to_bitstream(&protocol.sync_sequence_hex, sync_bit_len);
    let mut sync_index: Option<usize> = None;
    if demodulated_bits.len() >= sync_bits.len() {
        for idx in 0..=(demodulated_bits.len() - sync_bits.len()) {
            if demodulated_bits[idx..idx + sync_bits.len()] == sync_bits {
                sync_index = Some(idx);
                break;
            }
        }
    }

    if sync_index.is_none() {
        logger.log("Frame sync sequence not found; aborting decode.");
        let diagnostics = diagnostics_from_symbols(decision_symbols);
        let report = SimulationReport {
            pre_fec_errors,
            pre_fec_ber,
            ..SimulationReport::default()
        };
        return DemodulationResult {
            demodulated_bitstream: demodulated_bits,
            decoded_bitstream: Vec::new(),
            recovered_message: String::new(),
            diagnostics,
            report,
            logs: logger.entries().to_vec(),
        };
    }

    let sync_index = sync_index.unwrap();
    let aligned_bits = &demodulated_bits[sync_index..];
    let frame_bits = protocol.frame_layout.frame_bits();
    let frames_available = aligned_bits.len() / frame_bits;
    let frames_to_process = frames_available.min(encoding.total_frames.max(1));

    let prefix_bits = (protocol.frame_layout.sync_symbols
        + protocol.frame_layout.target_id_symbols
        + protocol.frame_layout.command_type_symbols)
        * 2;
    let codeword_bits = matrices.codeword_bits;
    let payload_start = prefix_bits;
    let payload_end = payload_start + codeword_bits;

    let mut decoded_payload_bits = Vec::with_capacity(frames_to_process * matrices.message_bits);
    for frame_idx in 0..frames_to_process {
        let start = frame_idx * frame_bits;
        let end = start + frame_bits;
        if end > aligned_bits.len() {
            break;
        }
        let frame_slice = &aligned_bits[start..end];
        if frame_slice.len() < payload_end {
            continue;
        }
        let noisy_codeword = &frame_slice[payload_start..payload_end];
        let decoded = decode_ldpc(matrices, noisy_codeword, 0.0);

        if frame_idx < 3 {
            logger.log(format!(
                "[RX] Frame {}/{} decoded ({} payload bits).",
                frame_idx + 1,
                frames_to_process,
                decoded.len()
            ));
        }

        decoded_payload_bits.extend_from_slice(&decoded);
    }

    let trimmed_length = encoding.payload_bits.len();
    let mut decoded_bitstream = decoded_payload_bits;
    if decoded_bitstream.len() < trimmed_length {
        decoded_bitstream.resize(trimmed_length, 0);
    } else {
        decoded_bitstream.truncate(trimmed_length);
    }

    let post_fec_errors = decoded_bitstream
        .iter()
        .zip(&encoding.payload_bits)
        .filter(|(rx, tx)| rx != tx)
        .count();
    let post_fec_ber = if trimmed_length > 0 {
        post_fec_errors as f64 / trimmed_length as f64
    } else {
        0.0
    };
    logger.log(format!(
        "Post-FEC BER: {:.6} ({} errors).",
        post_fec_ber, post_fec_errors
    ));

    let recovered_bytes = pack_bits(&decoded_bitstream);
    let recovered_message = String::from_utf8_lossy(&recovered_bytes)
        .trim_end_matches('\u{0}')
        .to_string();

    let diagnostics = diagnostics_from_symbols(decision_symbols);
    let report = SimulationReport {
        pre_fec_errors,
        pre_fec_ber,
        post_fec_errors,
        post_fec_ber,
        recovered_message: recovered_message.clone(),
    };

    DemodulationResult {
        demodulated_bitstream: demodulated_bits,
        decoded_bitstream,
        recovered_message,
        diagnostics,
        report,
        logs: logger.entries().to_vec(),
    }
}

pub fn baseband_to_symbols(signal: &Array1<f64>) -> Vec<Complex64> {
    let slice = signal
        .as_slice()
        .expect("baseband_to_symbols expects contiguous data");
    complex_from_interleaved(slice)
}

fn qpsk_constellation() -> [(Complex64, [u8; 2]); 4] {
    [
        (Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2), [0, 0]),
        (Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2), [0, 1]),
        (Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2), [1, 1]),
        (Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2), [1, 0]),
    ]
}

fn diagnostics_from_symbols(symbols: Vec<Complex64>) -> DemodulationDiagnostics {
    DemodulationDiagnostics {
        received_symbols_i: symbols.iter().map(|c| c.re).collect(),
        received_symbols_q: symbols.iter().map(|c| c.im).collect(),
        timing_error: vec![0.0; symbols.len()],
        nco_freq_offset: vec![0.0; symbols.len()],
    }
}
