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
use std::borrow::Cow;

use num_complex::Complex64;

use crate::config::ProtocolConfig;
use crate::ldpc::LDPCMatrices;
use crate::utils::{
    hex_to_bitstream, int_to_bitstream, LogCollector,
};

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
    // FSK layer state (1 bit/second nested modulation)
    fsk_bit_stream: Vec<u8>,
    fsk_bit_index: usize,
    symbols_since_fsk_transition: usize,
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
        
        // Generate FSK bit stream - nested 1 bit/second layer
        let fsk_bit_stream = Self::generate_fsk_pattern(payload_bits);
        
        Self {
            protocol,
            matrices,
            payload_bits: payload_bits.to_vec(),
            current_frame_index: 0,
            total_frames,
            current_symbol_in_frame: 0,
            current_frame_bitstream: Vec::new(),
            logger,
            fsk_bit_stream,
            fsk_bit_index: 0,
            symbols_since_fsk_transition: 0,
        }
    }
    
    /// Generate FSK bit pattern - could be message checksum, sync pattern, etc.
    /// Creates a slow 1 bit/second nested modulation layer (±1 Hz from carrier)
    fn generate_fsk_pattern(payload_bits: &[u8]) -> Vec<u8> {
        let mut pattern = vec![];
        
        // Start with sync pattern: 10101010
        pattern.extend_from_slice(&[1, 1, 0, 0, 1, 1, 0, 0]);
        
        // Add a simple 8-bit checksum of the payload
        if !payload_bits.is_empty() {
            let checksum = payload_bits.iter()
                .fold(0u8, |acc, &bit| acc ^ bit);
            for i in 0..8 {
                pattern.push((checksum >> (7 - i)) & 1);
            }
        }
        
        // Pad with alternating bits for visibility
        while pattern.len() < 32 {
            pattern.push((pattern.len() % 2) as u8);
        }
        
        pattern
    }
    
    /// Get current FSK frequency in Hz (12000 ± 1 Hz)
    pub fn get_current_fsk_frequency(&self) -> f64 {
        if self.fsk_bit_index < self.fsk_bit_stream.len() {
            if self.fsk_bit_stream[self.fsk_bit_index] == 1 {
                12001.0  // Binary "1"
            } else {
                11999.0  // Binary "0"
            }
        } else {
            12000.0  // Default to center frequency
        }
    }
    
    /// Get FSK bit stream for visualization
    pub fn get_fsk_bits(&self) -> &[u8] {
        &self.fsk_bit_stream
    }
    
    /// Get current FSK bit index
    pub fn get_fsk_bit_index(&self) -> usize {
        self.fsk_bit_index
    }
    
    /// Get symbols without copying - returns a borrowed slice when possible
    /// More efficient than get_next_symbols when you don't need ownership
    pub fn get_next_symbols_borrowed(&mut self, symbol_count: usize) -> (Cow<'_, [Complex64]>, bool, usize, usize, bool) {
        let (symbols, frame_changed, frame_index, symbol_index, is_complete) = 
            self.get_next_symbols(symbol_count);
        
        // For now, we return owned data, but this API allows for future zero-copy optimizations
        // when we add a ring buffer implementation
        (Cow::Owned(symbols), frame_changed, frame_index, symbol_index, is_complete)
    }
    
    /// Update FSK state based on symbols transmitted
    fn update_fsk_state(&mut self, symbol_count: usize) {
        // FSK changes at 1 bit/second, QPSK is 16 symbols/second
        // So FSK bit changes every 16 QPSK symbols
        self.symbols_since_fsk_transition += symbol_count;
        if self.symbols_since_fsk_transition >= 16 {
            self.symbols_since_fsk_transition = 0;
            self.fsk_bit_index = (self.fsk_bit_index + 1) % self.fsk_bit_stream.len();
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
        
        // Update FSK state
        self.update_fsk_state(symbols.len());
        
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
