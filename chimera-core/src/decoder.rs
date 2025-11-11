//! Demodulation and decoding stage implementations.
use std::f64::consts::FRAC_1_SQRT_2;

use num_complex::Complex64;

use crate::config::ProtocolConfig;
use crate::diagnostics::{DemodulationDiagnostics, SymbolDecision};
use crate::ldpc::{decode_ldpc, LDPCMatrices};
use crate::utils::{hex_to_bitstream, LogCollector};

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
    
    // FSK demodulator state (detects ±1 Hz shifts)
    fsk_symbol_history: Vec<Complex64>, // Symbols for FSK analysis
    fsk_phase_history: Vec<f64>,        // Instantaneous phase for frequency estimation
    fsk_detected_bits: Vec<u8>,         // Decoded FSK bits
    fsk_current_bit: u8,                // Current FSK bit estimate
    fsk_frequency_estimate: f64,        // Current frequency estimate in Hz
    symbols_since_fsk_update: usize,    // Counter for FSK bit transitions
    
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
            fsk_symbol_history: Vec::new(),
            fsk_phase_history: Vec::new(),
            fsk_detected_bits: Vec::new(),
            fsk_current_bit: 0,
            fsk_frequency_estimate: 12000.0,
            symbols_since_fsk_update: 0,
            logger,
        }
    }
    
    /// Add received symbols and process them incrementally
    /// Returns (new_decoded_bits, frame_complete, current_frame_index, symbols_in_frame)
    pub fn process_symbols(&mut self, symbols: &[Complex64]) -> (Vec<u8>, bool, usize, usize, DemodulationDiagnostics) {
        // Add symbols to buffer
        self.symbol_buffer.extend_from_slice(symbols);
        
        // Process FSK layer (frequency shift keying)
        self.demodulate_fsk(symbols);
        
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
            // Convert sync pattern to expected symbol sequence for correlation
            let sync_symbols: Vec<Complex64> = sync_bits
                .chunks(2)
                .map(|pair| {
                    let bits = [pair[0], pair.get(1).copied().unwrap_or(0)];
                    match (bits[0], bits[1]) {
                        (0, 0) => Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                        (0, 1) => Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                        (1, 1) => Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                        (1, 0) => Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                        _ => Complex64::new(0.0, 0.0),
                    }
                })
                .collect();
            
            // Convert demodulated bits back to symbols for correlation
            let num_demod_symbols = self.demodulated_bits.len() / 2;
            let demod_symbols: Vec<Complex64> = (0..num_demod_symbols)
                .map(|i| {
                    let b0 = self.demodulated_bits[i * 2];
                    let b1 = self.demodulated_bits.get(i * 2 + 1).copied().unwrap_or(0);
                    match (b0, b1) {
                        (0, 0) => Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                        (0, 1) => Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                        (1, 1) => Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                        (1, 0) => Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                        _ => Complex64::new(0.0, 0.0),
                    }
                })
                .collect();
            
            // Find best correlation over all possible phase rotations and positions
            let mut best_match_index = None;
            let mut best_correlation = 0.0f64;
            let mut best_phase_rotation = 0;
            
            // Try all 4 possible QPSK phase rotations (0°, 90°, 180°, 270°)
            for phase_idx in 0..4 {
                let phase_rotation = Complex64::from_polar(1.0, phase_idx as f64 * std::f64::consts::PI / 2.0);
                
                // Slide sync pattern across demodulated symbols
                for idx in 0..=(demod_symbols.len().saturating_sub(sync_symbols.len())) {
                    let window = &demod_symbols[idx..idx + sync_symbols.len()];
                    
                    // Compute correlation with phase rotation
                    let mut correlation_sum = Complex64::new(0.0, 0.0);
                    let mut energy_sum = 0.0f64;
                    
                    for (demod_sym, sync_sym) in window.iter().zip(sync_symbols.iter()) {
                        let rotated_demod = demod_sym * phase_rotation;
                        correlation_sum += rotated_demod * sync_sym.conj();
                        energy_sum += rotated_demod.norm_sqr();
                    }
                    
                    // Normalized correlation (0 to 1)
                    let correlation = if energy_sum > 0.0 {
                        correlation_sum.norm() / (energy_sum.sqrt() * (sync_symbols.len() as f64).sqrt())
                    } else {
                        0.0
                    };
                    
                    if correlation > best_correlation {
                        best_correlation = correlation;
                        best_match_index = Some(idx * 2); // Convert back to bit index
                        best_phase_rotation = phase_idx;
                    }
                }
            }
            
            // Accept sync if correlation is above threshold
            // Lowered to 0.50 for streaming symbol processing where symbols arrive incrementally
            // Note: Batch processing at very low SNR (-3 dB) may still have issues
            let sync_threshold = 0.50;
            if best_correlation >= sync_threshold {
                if let Some(idx) = best_match_index {
                    self.sync_index = Some(idx);
                    self.sync_found = true;
                    self.logger.log(format!(
                        "Frame sync found at bit index {} with correlation {:.2}% (phase rotation {})",
                        idx, best_correlation * 100.0, best_phase_rotation * 90
                    ));
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
    
    /// Demodulate FSK layer by estimating instantaneous frequency
    /// Detects ±1 Hz shifts in the carrier frequency
    fn demodulate_fsk(&mut self, symbols: &[Complex64]) {
        // Add symbols to FSK history (keep last 32 for frequency estimation)
        self.fsk_symbol_history.extend_from_slice(symbols);
        if self.fsk_symbol_history.len() > 32 {
            self.fsk_symbol_history.drain(0..self.fsk_symbol_history.len() - 32);
        }
        
        // Need at least 2 symbols to compute phase difference
        if self.fsk_symbol_history.len() < 2 {
            return;
        }
        
        // Compute instantaneous frequency using phase derivative
        // After I/Q demodulation at 12 kHz, the residual phase rotation reveals FSK:
        // - 11999 Hz carrier → -1 Hz residual → negative phase rotation
        // - 12001 Hz carrier → +1 Hz residual → positive phase rotation
        for i in 1..self.fsk_symbol_history.len() {
            let prev = self.fsk_symbol_history[i - 1];
            let curr = self.fsk_symbol_history[i];
            
            // Compute phase difference (unwrapped)
            // This represents the phase rotation between consecutive symbols
            let phase_diff = (curr / prev).arg();
            self.fsk_phase_history.push(phase_diff);
        }
        
        // Keep history manageable
        if self.fsk_phase_history.len() > 64 {
            self.fsk_phase_history.drain(0..self.fsk_phase_history.len() - 64);
        }
        
        // Update FSK state every 16 symbols (1 second at 16 sym/s)
        self.symbols_since_fsk_update += symbols.len();
        if self.symbols_since_fsk_update >= 16 && self.fsk_phase_history.len() >= 16 {
            // Average phase differences over the last 16 samples
            let recent_phases: Vec<f64> = self.fsk_phase_history
                .iter()
                .rev()
                .take(16)
                .copied()
                .collect();
            
            let avg_phase_diff = recent_phases.iter().sum::<f64>() / recent_phases.len() as f64;
            
            // Convert phase difference per symbol to residual frequency
            // symbol_rate = 16 Hz, so residual_freq = phase_diff * symbol_rate / (2π)
            // This residual is the difference from the 12 kHz demodulation frequency
            let symbol_rate = 16.0; // Hz
            let residual_freq = avg_phase_diff * symbol_rate / (2.0 * std::f64::consts::PI);
            
            // Estimate absolute carrier frequency (12 kHz + residual)
            // Note: After testing, the residual appears inverted, so we negate it
            self.fsk_frequency_estimate = 12000.0 - residual_freq;
            
            // Decode FSK bit: frequency > 12000 Hz → bit 1, else → bit 0
            let threshold = 12000.0;
            let new_bit = if self.fsk_frequency_estimate > threshold { 1 } else { 0 };
            
            // Update FSK bit every interval (1 bit/second)
            self.fsk_current_bit = new_bit;
            self.fsk_detected_bits.push(new_bit);
            
            // Keep history manageable
            if self.fsk_detected_bits.len() > 32 {
                self.fsk_detected_bits.remove(0);
            }
            
            self.symbols_since_fsk_update = 0;
        }
    }
    
    /// Get current FSK frequency estimate in Hz
    pub fn get_fsk_frequency(&self) -> f64 {
        self.fsk_frequency_estimate
    }
    
    /// Get current FSK bit estimate
    pub fn get_fsk_bit(&self) -> u8 {
        self.fsk_current_bit
    }
    
    /// Get FSK bit history for visualization
    pub fn get_fsk_bits(&self) -> &[u8] {
        &self.fsk_detected_bits
    }
    
    pub fn get_logs(&self) -> &[String] {
        self.logger.entries()
    }
    
    pub fn get_demodulated_bits(&self) -> &[u8] {
        &self.demodulated_bits
    }
}
