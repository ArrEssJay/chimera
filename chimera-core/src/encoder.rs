//! Encoding and modulation stage.
use ndarray::Array1;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::ldpc::LDPCMatrices;
use crate::utils::{hex_to_bitstream, int_to_bitstream, LogCollector};

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
    pub qpsk_bitstream: Vec<u8>,
    pub payload_bits: Vec<u8>,
    pub total_frames: usize,
    pub logs: Vec<String>,
}

impl EncodingResult {
    pub fn new() -> Self {
        Self {
            noisy_signal: Array1::from_vec(vec![]),
            clean_signal: Array1::from_vec(vec![]),
            qpsk_bitstream: Vec::new(),
            payload_bits: Vec::new(),
            total_frames: 0,
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
    logger.log("generate_modulated_signal: unimplemented");
    let _ = (sim, protocol, matrices);
    EncodingResult {
        logs: logger.entries().to_vec(),
        ..EncodingResult::default()
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
