//! Encoding and modulation stage.
use std::f64::consts::FRAC_1_SQRT_2;

use ndarray::Array1;
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::StandardNormal;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::ldpc::LDPCMatrices;
use crate::utils::{hex_to_bitstream, int_to_bitstream, string_to_bitstream, LogCollector};

pub struct FrameStream {
    pub frames_bitstream: Vec<u8>,
    pub frame_count: usize,
    pub logs: Vec<String>,
}

impl FrameStream {
    pub fn empty() -> Self {
        Self {
            frames_bitstream: Vec::new(),
            frame_count: 0,
            logs: Vec::new(),
        }
    }
}

pub struct EncodingResult {
    pub noisy_signal: Array1<f64>,
    pub clean_signal: Array1<f64>,
    pub qpsk_symbols: Vec<Complex64>,
    pub qpsk_bitstream: Vec<u8>,
    pub payload_bits: Vec<u8>,
    pub total_frames: usize,
    pub samples_per_symbol: usize,
    pub sample_rate: usize,
    pub logs: Vec<String>,
}

impl EncodingResult {
    pub fn new() -> Self {
        Self {
            noisy_signal: Array1::from_vec(vec![]),
            clean_signal: Array1::from_vec(vec![]),
            qpsk_symbols: Vec::new(),
            qpsk_bitstream: Vec::new(),
            payload_bits: Vec::new(),
            total_frames: 0,
            samples_per_symbol: 1,
            sample_rate: 0,
            logs: Vec::new(),
        }
    }
}

impl Default for EncodingResult {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_modulated_signal(
    sim: &SimulationConfig,
    protocol: &ProtocolConfig,
    matrices: &LDPCMatrices,
) -> EncodingResult {
    let mut logger = LogCollector::new();

    let plaintext = &sim.plaintext_source;
    let payload_bits = string_to_bitstream(plaintext);
    logger.log(format!(
        "Source plaintext length: {} characters ({} bits).",
        plaintext.len(),
        payload_bits.len()
    ));

    let frame_stream = build_frame_stream(&payload_bits, protocol, matrices, &mut logger);

    let qpsk_bitstream = frame_stream.frames_bitstream.clone();
    assert!(
        qpsk_bitstream.len() % 2 == 0,
        "QPSK bitstream must have even length"
    );

    let samples_per_symbol = usize::max(1, sim.sample_rate / protocol.qpsk_symbol_rate);
    let mut qpsk_symbols = Vec::with_capacity(qpsk_bitstream.len() / 2);

    for bits in qpsk_bitstream.chunks_exact(2) {
        let symbol = match (bits[0], bits[1]) {
            (0, 0) => Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (0, 1) => Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (1, 1) => Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            (1, 0) => Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            _ => unreachable!("bits are constrained to 0/1"),
        };
        qpsk_symbols.push(symbol);
    }

    let mut clean_iq = Vec::with_capacity(qpsk_symbols.len() * 2 * samples_per_symbol);
    for symbol in &qpsk_symbols {
        for _ in 0..samples_per_symbol {
            clean_iq.push(symbol.re);
            clean_iq.push(symbol.im);
        }
    }

    let mut rng = match sim.rng_seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    let signal_power: f64 =
        qpsk_symbols.iter().map(|c| c.norm_sqr()).sum::<f64>() / qpsk_symbols.len().max(1) as f64;

    let snr_linear = 10f64.powf(sim.snr_db / 10.0);
    let noise_variance = if snr_linear > 0.0 {
        signal_power / snr_linear
    } else {
        0.0
    };
    let noise_std = (noise_variance / 2.0).sqrt();

    let mut noisy_iq = Vec::with_capacity(clean_iq.len());
    let normal = StandardNormal;
    for chunk in clean_iq.chunks_exact(2) {
        let noise_i: f64 = rng.sample::<f64, _>(normal) * noise_std;
        let noise_q: f64 = rng.sample::<f64, _>(normal) * noise_std;
        noisy_iq.push(chunk[0] + noise_i);
        noisy_iq.push(chunk[1] + noise_q);
    }

    logger.log(format!(
        "Generated {} QPSK symbols across {} frame(s).",
        qpsk_symbols.len(),
        frame_stream.frame_count
    ));
    logger.log(format!(
        "Applied AWGN channel with SNR = {:.2} dB.",
        sim.snr_db
    ));

    EncodingResult {
        noisy_signal: Array1::from_vec(noisy_iq),
        clean_signal: Array1::from_vec(clean_iq),
        qpsk_symbols,
        qpsk_bitstream,
        payload_bits,
        total_frames: frame_stream.frame_count,
        samples_per_symbol,
        sample_rate: sim.sample_rate,
        logs: logger.entries().to_vec(),
    }
}

pub fn build_frame_stream(
    payload_bits: &[u8],
    protocol: &ProtocolConfig,
    matrices: &LDPCMatrices,
    logger: &mut LogCollector,
) -> FrameStream {
    let layout = &protocol.frame_layout;
    let message_bits = matrices.message_bits;
    let frame_bits = layout.frame_bits();

    let total_frames = if payload_bits.is_empty() {
        1
    } else {
        (payload_bits.len() + message_bits - 1) / message_bits
    };
    logger.log(format!("Payload requires {total_frames} frame(s)."));

    let sync_bits = hex_to_bitstream(&protocol.sync_sequence_hex, layout.sync_symbols * 2);
    let target_bits = hex_to_bitstream(&protocol.target_id_hex, layout.target_id_symbols * 2);
    let command_bits_len = layout.command_type_symbols * 2;

    let mut frames_bitstream = Vec::with_capacity(total_frames * frame_bits);

    for frame_idx in 0..total_frames {
        let command_value = (protocol.command_opcode as u32)
            | ((frame_idx as u32) << protocol.current_frame_shift)
            | ((total_frames as u32) << protocol.total_frames_shift);
        let command_bits = int_to_bitstream(command_value as u64, command_bits_len);

        let start = frame_idx * message_bits;
        let end = usize::min(start + message_bits, payload_bits.len());
        let mut message_chunk = vec![0u8; message_bits];
        if start < end {
            message_chunk[..(end - start)].copy_from_slice(&payload_bits[start..end]);
        }

        let codeword = encode_with_generator(&matrices.generator, &message_chunk);
        let payload_section = &codeword[..message_bits];
        let ecc_section = &codeword[message_bits..];

        if frame_idx < 3 {
            logger.log(format!(
                "[TX] Frame {}/{total_frames}: command=0x{command_value:08X}",
                frame_idx + 1
            ));
        }

        frames_bitstream.extend_from_slice(&sync_bits);
        frames_bitstream.extend_from_slice(&target_bits);
        frames_bitstream.extend_from_slice(&command_bits);
        frames_bitstream.extend_from_slice(payload_section);
        frames_bitstream.extend_from_slice(ecc_section);
    }

    FrameStream {
        frames_bitstream,
        frame_count: total_frames,
        logs: logger.entries().to_vec(),
    }
}

fn encode_with_generator(generator: &ndarray::Array2<u8>, message: &[u8]) -> Vec<u8> {
    assert_eq!(
        generator.nrows(),
        message.len(),
        "message length must match generator rank"
    );
    let mut codeword = vec![0u8; generator.ncols()];

    for (row_idx, &bit) in message.iter().enumerate() {
        if bit == 0 {
            continue;
        }
        for col_idx in 0..generator.ncols() {
            codeword[col_idx] ^= generator[(row_idx, col_idx)] & 1;
        }
    }

    codeword
}
