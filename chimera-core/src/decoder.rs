//! Demodulation and decoding stage implementations.
use std::f64::consts::FRAC_1_SQRT_2;

use ndarray::Array1;
use num_complex::Complex64;

use crate::config::{ProtocolConfig, SimulationConfig};
use crate::diagnostics::{DemodulationDiagnostics, SimulationReport, SymbolDecision};
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
    let mut symbol_decisions = Vec::with_capacity(symbol_count);

    for symbol_idx in 0..symbol_count {
        let start = symbol_idx * samples_per_symbol;
        let end = start + samples_per_symbol;
        let sample_slice = &noisy_symbols[start..end];
        let avg_symbol = sample_slice.iter().copied().sum::<Complex64>()
            / Complex64::new(samples_per_symbol as f64, 0.0);

        let mut distances = [0.0_f64; 4];
        let mut best_distance = f64::INFINITY;
        let mut best_index = 0usize;

        for (idx, (candidate, _)) in reference.iter().enumerate() {
            let distance = (avg_symbol - *candidate).norm_sqr();
            distances[idx] = distance;
            if distance < best_distance {
                best_distance = distance;
                best_index = idx;
            }
        }

        let closest_bits = reference[best_index].1;

        let mut bit_min_distance = [[f64::INFINITY; 2]; 2];
        for (idx, (_, bits)) in reference.iter().enumerate() {
            for bit_pos in 0..2 {
                let bit_val = bits[bit_pos] as usize;
                if distances[idx] < bit_min_distance[bit_pos][bit_val] {
                    bit_min_distance[bit_pos][bit_val] = distances[idx];
                }
            }
        }

        let mut soft_metrics = [0.0_f64; 2];
        for bit_pos in 0..2 {
            let zero = bit_min_distance[bit_pos][0];
            let one = bit_min_distance[bit_pos][1];
            if zero.is_finite() && one.is_finite() {
                soft_metrics[bit_pos] = one - zero;
            }
        }

        demodulated_bits.extend_from_slice(&closest_bits);
        symbol_decisions.push(SymbolDecision {
            index: symbol_idx,
            decided_bits: closest_bits,
            average_i: avg_symbol.re,
            average_q: avg_symbol.im,
            min_distance: best_distance,
            distances,
            soft_metrics,
        });
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
    logger.log(format!(
        "Pre-FEC BER: {:.6} ({} errors).",
        pre_fec_ber, pre_fec_errors
    ));

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
        let diagnostics = diagnostics_from_decisions(symbol_decisions.clone());
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

    let diagnostics = diagnostics_from_decisions(symbol_decisions);
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

pub fn qpsk_constellation() -> [(Complex64, [u8; 2]); 4] {
    [
        (Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2), [0, 0]),
        (Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2), [0, 1]),
        (Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2), [1, 1]),
        (Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2), [1, 0]),
    ]
}

/// Demodulate a single QPSK symbol to bits
pub fn demodulate_qpsk_symbol(symbol: Complex64) -> [u8; 2] {
    let reference = qpsk_constellation();
    let mut best_distance = f64::INFINITY;
    let mut best_bits = [0, 0];
    
    for (const_point, bits) in reference.iter() {
        let distance = (symbol - const_point).norm_sqr();
        if distance < best_distance {
            best_distance = distance;
            best_bits = *bits;
        }
    }
    
    best_bits
}

/// Incremental symbol-by-symbol decoder
pub struct StreamingSymbolDecoder {
    protocol: ProtocolConfig,
    matrices: LDPCMatrices,
    
    // Buffer for received symbols
    symbol_buffer: Vec<Complex64>,
    demodulated_bits: Vec<u8>,
    
    // Frame synchronization state
    sync_found: bool,
    sync_index: Option<usize>,
    
    // Current frame decoding state
    current_frame_index: usize,
    symbols_in_current_frame: usize,
    decoded_frames: Vec<Vec<u8>>,
    
    logger: LogCollector,
}

impl StreamingSymbolDecoder {
    pub fn new(protocol: ProtocolConfig, matrices: LDPCMatrices) -> Self {
        let mut logger = LogCollector::new();
        logger.log("Initializing streaming symbol decoder.".to_string());
        
        Self {
            protocol,
            matrices,
            symbol_buffer: Vec::new(),
            demodulated_bits: Vec::new(),
            sync_found: false,
            sync_index: None,
            current_frame_index: 0,
            symbols_in_current_frame: 0,
            decoded_frames: Vec::new(),
            logger,
        }
    }
    
    /// Add received symbols and process them incrementally
    /// Returns (new_decoded_bits, frame_complete, current_frame_index, symbols_in_frame)
    pub fn process_symbols(&mut self, symbols: &[Complex64]) -> (Vec<u8>, bool, usize, usize, DemodulationDiagnostics) {
        // Add symbols to buffer
        self.symbol_buffer.extend_from_slice(symbols);
        
        let mut new_decoded_bits = Vec::new();
        let mut frame_complete = false;
        
        // Demodulate new symbols
        for symbol in symbols {
            let bits = demodulate_qpsk_symbol(*symbol);
            self.demodulated_bits.push(bits[0]);
            self.demodulated_bits.push(bits[1]);
        }
        
        // Try to find sync if not yet found
        if !self.sync_found {
            self.search_for_sync();
        }
        
        // Process frame if sync is found
        if self.sync_found {
            let sync_offset = self.sync_index.unwrap_or(0);
            let frame_bits = self.protocol.frame_layout.frame_bits();
            let symbols_per_frame = self.protocol.frame_layout.total_symbols;
            
            // Check if we have enough symbols for current position
            let bits_available = self.demodulated_bits.len().saturating_sub(sync_offset);
            let symbols_available = bits_available / 2;
            let frame_start_symbol = self.current_frame_index * symbols_per_frame;
            
            self.symbols_in_current_frame = symbols_available.saturating_sub(frame_start_symbol)
                .min(symbols_per_frame);
            
            // Check if current frame is complete
            if symbols_available >= frame_start_symbol + symbols_per_frame {
                // Decode the complete frame
                let frame_bit_start = sync_offset + (frame_start_symbol * 2);
                let frame_bit_end = frame_bit_start + frame_bits;
                
                if frame_bit_end <= self.demodulated_bits.len() {
                    let frame_slice = &self.demodulated_bits[frame_bit_start..frame_bit_end];
                    
                    // Extract and decode payload
                    let prefix_bits = (self.protocol.frame_layout.sync_symbols
                        + self.protocol.frame_layout.target_id_symbols
                        + self.protocol.frame_layout.command_type_symbols) * 2;
                    let codeword_bits = self.matrices.codeword_bits;
                    let payload_start = prefix_bits;
                    let payload_end = payload_start + codeword_bits;
                    
                    if frame_slice.len() >= payload_end {
                        let noisy_codeword = &frame_slice[payload_start..payload_end];
                        let decoded = decode_ldpc(&self.matrices, noisy_codeword, 0.0);
                        
                        new_decoded_bits = decoded.clone();
                        self.decoded_frames.push(decoded);
                        
                        if self.current_frame_index < 3 {
                            self.logger.log(format!(
                                "[RX] Frame {} decoded ({} payload bits).",
                                self.current_frame_index + 1,
                                new_decoded_bits.len()
                            ));
                        }
                        
                        self.current_frame_index += 1;
                        self.symbols_in_current_frame = 0;
                        frame_complete = true;
                    }
                }
            }
        }
        
        // Create diagnostics from recent symbols
        let diagnostics = self.create_diagnostics(symbols);
        
        (new_decoded_bits, frame_complete, self.current_frame_index, self.symbols_in_current_frame, diagnostics)
    }
    
    fn search_for_sync(&mut self) {
        let sync_bit_len = self.protocol.frame_layout.sync_symbols * 2;
        let sync_bits = hex_to_bitstream(&self.protocol.sync_sequence_hex, sync_bit_len);
        
        if self.demodulated_bits.len() >= sync_bits.len() {
            for idx in 0..=(self.demodulated_bits.len() - sync_bits.len()) {
                if self.demodulated_bits[idx..idx + sync_bits.len()] == sync_bits {
                    self.sync_index = Some(idx);
                    self.sync_found = true;
                    self.logger.log(format!("Frame sync found at bit index {}", idx));
                    break;
                }
            }
        }
    }
    
    fn create_diagnostics(&self, recent_symbols: &[Complex64]) -> DemodulationDiagnostics {
        let (received_symbols_i, received_symbols_q): (Vec<f64>, Vec<f64>) =
            recent_symbols.iter().map(|s| (s.re, s.im)).unzip();
        
        let symbol_decisions: Vec<SymbolDecision> = recent_symbols.iter().enumerate().map(|(i, s)| {
            let bits = demodulate_qpsk_symbol(*s);
            let reference = qpsk_constellation();
            let mut distances = [0.0_f64; 4];
            let mut best_distance = f64::INFINITY;
            
            for (idx, (candidate, _)) in reference.iter().enumerate() {
                let distance = (*s - *candidate).norm_sqr();
                distances[idx] = distance;
                if distance < best_distance {
                    best_distance = distance;
                }
            }
            
            SymbolDecision {
                index: i,
                decided_bits: bits,
                average_i: s.re,
                average_q: s.im,
                min_distance: best_distance,
                distances,
                soft_metrics: [0.0, 0.0],
            }
        }).collect();
        
        DemodulationDiagnostics {
            received_symbols_i,
            received_symbols_q,
            symbol_decisions,
            timing_error: vec![0.0; recent_symbols.len()],
            nco_freq_offset: vec![0.0; recent_symbols.len()],
        }
    }
    
    pub fn get_decoded_payload(&self) -> Vec<u8> {
        self.decoded_frames.iter().flatten().copied().collect()
    }
    
    pub fn is_synced(&self) -> bool {
        self.sync_found
    }
    
    pub fn get_logs(&self) -> &[String] {
        self.logger.entries()
    }
}

fn diagnostics_from_decisions(decisions: Vec<SymbolDecision>) -> DemodulationDiagnostics {
    let (received_symbols_i, received_symbols_q): (Vec<f64>, Vec<f64>) =
        decisions.iter().map(|d| (d.average_i, d.average_q)).unzip();

    let len = decisions.len();

    DemodulationDiagnostics {
        received_symbols_i,
        received_symbols_q,
        symbol_decisions: decisions,
        timing_error: vec![0.0; len],
        nco_freq_offset: vec![0.0; len],
    }
}
