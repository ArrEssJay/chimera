/// Encodes a message vector using an LDPC generator matrix.
///
/// This function performs the core LDPC encoding operation by multiplying the
/// message vector with the generator matrix over the Galois Field GF(2). In this
/// field, addition is equivalent to the XOR operation. The resulting vector is
/// the codeword, which includes the original message bits and the appended
/// parity check bits.
///
/// The operation is equivalent to `codeword = message * G`, where `G` is the
/// generator matrix.
///
/// # Arguments
///
/// * `generator` - A 2D array representing the generator matrix (`G`). Its
///   dimensions should be `k x n`, where `k` is the message length and `n` is
///   the codeword length.
/// * `message` - A slice of `u8` representing the message bits to be encoded.
///   Its length must be equal to the number of rows in the `generator` matrix.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded codeword of length `n`.
///
/// # Panics
///
/// This function will panic if the length of the `message` does not match the
/// number of rows in the `generator` matrix.
use std::f64::consts::FRAC_1_SQRT_2;

use ndarray::Array1;
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::StandardNormal;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::diagnostics::FrameDescriptor;
use crate::ldpc::LDPCMatrices;
use crate::utils::{
    hex_to_bitstream, int_to_bitstream, pack_bits, string_to_bitstream, LogCollector,
};

pub struct FrameStream {
    pub frames_bitstream: Vec<u8>,
    pub frame_count: usize,
    pub logs: Vec<String>,
    pub frames: Vec<FrameDescriptor>,
}

impl FrameStream {
    pub fn empty() -> Self {
        Self {
            frames_bitstream: Vec::new(),
            frame_count: 0,
            logs: Vec::new(),
            frames: Vec::new(),
        }
    }
}

/// Incremental frame encoder for symbol-by-symbol streaming
pub struct StreamingFrameEncoder {
    protocol: ProtocolConfig,
    matrices: LDPCMatrices,
    payload_bits: Vec<u8>,
    current_frame_index: usize,
    pub total_frames: usize,
    current_symbol_in_frame: usize,
    current_frame_bitstream: Vec<u8>,
    logger: LogCollector,
}

impl StreamingFrameEncoder {
    pub fn new(
        payload_bits: &[u8],
        protocol: ProtocolConfig,
        matrices: LDPCMatrices,
    ) -> Self {
        let message_bits = matrices.message_bits;
        let total_frames = if payload_bits.is_empty() {
            1
        } else {
            payload_bits.len().div_ceil(message_bits)
        };
        
        let mut logger = LogCollector::new();
        logger.log(format!("Initializing streaming encoder for {total_frames} frame(s)."));
        
        Self {
            protocol,
            matrices,
            payload_bits: payload_bits.to_vec(),
            current_frame_index: 0,
            total_frames,
            current_symbol_in_frame: 0,
            current_frame_bitstream: Vec::new(),
            logger,
        }
    }
    
    /// Get the next batch of symbols (returns up to requested count)
    /// Returns (symbols, frame_changed, frame_index, symbol_index_in_frame, is_complete)
    pub fn get_next_symbols(&mut self, symbol_count: usize) -> (Vec<Complex64>, bool, usize, usize, bool) {
        let mut symbols = Vec::new();
        let mut frame_changed = false;
        let start_frame = self.current_frame_index;
        
        while symbols.len() < symbol_count {
            // Generate current frame if needed
            if self.current_symbol_in_frame == 0 {
                if self.current_frame_index > start_frame {
                    frame_changed = true;
                }
                self.generate_current_frame();
            }
            
            // Get symbols from current frame
            let symbols_available = self.current_frame_bitstream.len() / 2 - self.current_symbol_in_frame;
            let symbols_to_take = (symbol_count - symbols.len()).min(symbols_available);
            
            for _ in 0..symbols_to_take {
                let bit_index = self.current_symbol_in_frame * 2;
                let bits = &self.current_frame_bitstream[bit_index..bit_index + 2];
                
                let symbol = match (bits[0], bits[1]) {
                    (0, 0) => Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                    (0, 1) => Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                    (1, 1) => Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                    (1, 0) => Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                    _ => unreachable!("bits are constrained to 0/1"),
                };
                
                symbols.push(symbol);
                self.current_symbol_in_frame += 1;
            }
            
            // Check if frame is complete
            let symbols_per_frame = self.protocol.frame_layout.total_symbols;
            if self.current_symbol_in_frame >= symbols_per_frame {
                self.current_frame_index += 1;
                // Loop back to start when all frames transmitted
                if self.current_frame_index >= self.total_frames {
                    self.current_frame_index = 0;
                }
                self.current_symbol_in_frame = 0;
                self.current_frame_bitstream.clear();
            }
        }
        
        (symbols, frame_changed, self.current_frame_index, self.current_symbol_in_frame, false)
    }
    
    fn generate_current_frame(&mut self) {
        if self.current_frame_index >= self.total_frames {
            return;
        }
        
        let layout = &self.protocol.frame_layout;
        let message_bits = self.matrices.message_bits;
        
        let sync_bits = hex_to_bitstream(&self.protocol.sync_sequence_hex, layout.sync_symbols * 2);
        let target_bits = hex_to_bitstream(&self.protocol.target_id_hex, layout.target_id_symbols * 2);
        let command_bits_len = layout.command_type_symbols * 2;
        
        let frame_idx = self.current_frame_index;
        let command_value = self.protocol.command_opcode
            | ((frame_idx as u32) << self.protocol.current_frame_shift)
            | ((self.total_frames as u32) << self.protocol.total_frames_shift);
        let command_bits = int_to_bitstream(command_value as u64, command_bits_len);
        
        let start = frame_idx * message_bits;
        let end = usize::min(start + message_bits, self.payload_bits.len());
        let mut message_chunk = vec![0u8; message_bits];
        if start < end {
            message_chunk[..(end - start)].copy_from_slice(&self.payload_bits[start..end]);
        }
        
        let codeword = encode_with_generator(&self.matrices.generator, &message_chunk);
        let payload_section = &codeword[..message_bits];
        let ecc_section = &codeword[message_bits..];
        
        self.current_frame_bitstream.clear();
        self.current_frame_bitstream.extend_from_slice(&sync_bits);
        self.current_frame_bitstream.extend_from_slice(&target_bits);
        self.current_frame_bitstream.extend_from_slice(&command_bits);
        self.current_frame_bitstream.extend_from_slice(payload_section);
        self.current_frame_bitstream.extend_from_slice(ecc_section);
        
        if frame_idx < 3 {
            self.logger.log(format!(
                "[TX] Frame {}/{}: command=0x{command_value:08X}",
                frame_idx + 1, self.total_frames
            ));
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_frame_index >= self.total_frames
    }
    
    pub fn get_current_frame_bits(&self) -> &[u8] {
        &self.current_frame_bitstream
    }
    
    pub fn get_logs(&self) -> &[String] {
        self.logger.entries()
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
    pub frame_descriptors: Vec<FrameDescriptor>,
}

/// Creates a new `EncodingResult` with default values.
///
/// All vector and array fields are initialized as empty. Numerical fields
/// are set to their default state (e.g., 0 or 1). This is useful for
/// creating a placeholder result before the encoding process begins.
///
/// # Examples
///
/// ```
/// # use ndarray::Array1;
/// # use chimera_core::encoder::EncodingResult;
/// let result = EncodingResult::new();
///
/// assert!(result.noisy_signal.is_empty());
/// assert!(result.clean_signal.is_empty());
/// assert_eq!(result.total_frames, 0);
/// assert_eq!(result.samples_per_symbol, 1);
/// ```
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
            frame_descriptors: Vec::new(),
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
    let frame_descriptors = frame_stream.frames.clone();

    let qpsk_bitstream = frame_stream.frames_bitstream.clone();
    assert!(
        qpsk_bitstream.len().is_multiple_of(2),
        "QPSK bitstream must have even length"
    );

    assert!(
        protocol.qpsk_symbol_rate > 0,
        "QPSK symbol rate must be non-zero"
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

    // Apply link loss (signal attenuation) - reduces signal power
    // Link loss represents path loss, free-space loss, antenna gains, etc.
    let link_loss_linear = 10f64.powf(sim.link_loss_db / 10.0);
    let attenuated_signal_power = signal_power / link_loss_linear;

    // Apply link loss to clean signal
    let attenuation_factor = if link_loss_linear > 0.0 {
        1.0 / link_loss_linear.sqrt()
    } else {
        1.0
    };
    let attenuated_iq: Vec<f64> = clean_iq.iter().map(|&x| x * attenuation_factor).collect();

    // Calculate noise based on SNR (Es/N0) and attenuated signal power
    // Note: sim.snr_db represents Es/N0 (symbol energy to noise density ratio), not Eb/N0.
    // For QPSK with 2 bits/symbol: Eb/N0 = Es/N0 - 3.01 dB.
    // Processing gain from oversampling: ~34.77 dB with sample_rate=48kHz, symbol_rate=16.
    let snr_linear = 10f64.powf(sim.snr_db / 10.0);
    let noise_variance = if snr_linear > 0.0 {
        attenuated_signal_power / snr_linear
    } else {
        0.0
    };
    let noise_std = (noise_variance / 2.0).sqrt();

    // Add AWGN to attenuated signal
    let mut noisy_iq = Vec::with_capacity(attenuated_iq.len());
    let normal = StandardNormal;
    for chunk in attenuated_iq.chunks_exact(2) {
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
    if sim.link_loss_db != 0.0 {
        logger.log(format!(
            "Applied link loss: {:.2} dB (signal attenuation).",
            sim.link_loss_db
        ));
    }
    logger.log(format!(
        "Applied AWGN with SNR = {:.2} dB (Es/N₀).",
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
        frame_descriptors,
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
        payload_bits.len().div_ceil(message_bits)
    };
    logger.log(format!("Payload requires {total_frames} frame(s)."));

    let sync_bits = hex_to_bitstream(&protocol.sync_sequence_hex, layout.sync_symbols * 2);
    let target_bits = hex_to_bitstream(&protocol.target_id_hex, layout.target_id_symbols * 2);
    let command_bits_len = layout.command_type_symbols * 2;

    let mut frames_bitstream = Vec::with_capacity(total_frames * frame_bits);
    let mut frames = Vec::with_capacity(total_frames);

    for frame_idx in 0..total_frames {
        let command_value = protocol.command_opcode
            | ((frame_idx as u32) << protocol.current_frame_shift)
            | ((total_frames as u32) << protocol.total_frames_shift);
        let command_bits = int_to_bitstream(command_value as u64, command_bits_len);

        let start = frame_idx * message_bits;
        let end = usize::min(start + message_bits, payload_bits.len());
        let mut message_chunk = vec![0u8; message_bits];
        if start < end {
            message_chunk[..(end - start)].copy_from_slice(&payload_bits[start..end]);
        }

        let payload_window = if start < end {
            &payload_bits[start..end]
        } else {
            &[] as &[u8]
        };

        let payload_preview = if !payload_window.is_empty() {
            let mut preview_bits = payload_window.to_vec();
            while preview_bits.len() % 8 != 0 {
                preview_bits.push(0);
            }
            let preview_bytes = pack_bits(&preview_bits);
            let mut preview = preview_bytes
                .iter()
                .take(8)
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<_>>()
                .join(" ");
            if preview_bytes.len() > 8 {
                preview.push_str(" …");
            }
            preview
        } else {
            String::from("(padding)")
        };

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

        frames.push(FrameDescriptor {
            frame_index: frame_idx,
            total_frames,
            command_opcode: protocol.command_opcode,
            command_value,
            frame_label: format!("Frame {} of {}", frame_idx + 1, total_frames),
            payload_preview,
        });
    }

    FrameStream {
        frames_bitstream,
        frame_count: total_frames,
        logs: logger.entries().to_vec(),
        frames,
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
