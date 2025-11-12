//! The canonical Chimera data processor
//!
//! This is THE ONLY way data flows through the system. All other interfaces
//! (batch, streaming, etc.) are thin wrappers around this processor.
//!
//! CRITICAL: The modulator/demodulator are the source of truth. We wrap them
//! but NEVER modify their behavior.

pub mod config;
pub mod output;
pub mod modulator_wrapper;
pub mod demodulator_wrapper;

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::encoder::StreamingFrameEncoder;
use crate::decoder::StreamingSymbolDecoder;
use crate::utils::string_to_bitstream;
use crate::channel::apply_audio_noise;
use crate::config::InternalProtocolConfig;
use crate::ldpc::LDPCSuite;

pub use config::{ProcessorConfig, ChannelConfig};
pub use output::{ProcessorOutput, BatchOutput};
use modulator_wrapper::ModulatorWrapper;
use demodulator_wrapper::DemodulatorWrapper;

/// The canonical Chimera data processor
///
/// This respects the modulator/demodulator as the source of truth:
/// - Modulator adds preamble and creates properly formatted frames
/// - Demodulator consumes preamble for synchronization
/// - We handle the high-level flow but don't change their behavior
pub struct ChimeraProcessor {
    config: ProcessorConfig,
    
    // Components (wrappers around existing code)
    modulator: ModulatorWrapper,
    demodulator: DemodulatorWrapper,
    
    // LDPC encoding/decoding
    ldpc_suite: LDPCSuite,
    protocol: InternalProtocolConfig,
    
    // Decoder state (maintained across calls for incremental processing)
    decoder: Option<StreamingSymbolDecoder>,
    
    // Buffering for incremental processing
    input_buffer: Vec<u8>,
    audio_buffer: Vec<f32>,
    
    // RNG for channel effects
    rng: StdRng,
    
    // Diagnostics
    diagnostics_enabled: bool,
}

impl ChimeraProcessor {
    pub fn new(config: ProcessorConfig) -> Self {
        // Create protocol config from processor config - use default and override
        let mut protocol = InternalProtocolConfig::default();
        protocol.carrier_freq_hz = config.carrier_freq;
        protocol.qpsk_symbol_rate = config.symbol_rate;
        protocol.enable_qpsk = config.enable_qpsk;
        protocol.enable_fsk = config.enable_fsk;
        
        // Create LDPC suite
        let ldpc_config = crate::config::LDPCConfig::default();
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
        
        // Create modulator/demodulator wrappers
        let modulator = ModulatorWrapper::new(
            config.sample_rate,
            config.symbol_rate,
            config.carrier_freq,
            config.enable_qpsk,
            config.enable_fsk,
        );
        
        let demodulator = DemodulatorWrapper::new(
            config.sample_rate,
            config.symbol_rate,
            config.carrier_freq,
        );
        
        Self {
            config,
            modulator,
            demodulator,
            ldpc_suite,
            protocol,
            decoder: None,
            input_buffer: Vec::new(),
            audio_buffer: Vec::new(),
            rng: StdRng::seed_from_u64(42),
            diagnostics_enabled: false,
        }
    }
    
    pub fn new_with_defaults() -> Self {
        Self::new(ProcessorConfig::default())
    }
    
    pub fn enable_diagnostics(&mut self) {
        self.diagnostics_enabled = true;
    }
    
    /// Process a chunk of input data (realtime mode)
    ///
    /// Can be called with any amount of data - handles buffering internally.
    /// Returns output as soon as complete frames are available.
    ///
    /// This is the PRIMARY method for realtime streaming.
    pub fn process_chunk(&mut self, input: &[u8]) -> ProcessorOutput {
        // Add to input buffer
        self.input_buffer.extend_from_slice(input);
        
        // Process what we can based on current state
        self.process_available()
    }
    
    /// Flush any remaining buffered data (end of stream)
    ///
    /// Call this when no more input will arrive to process partial frames.
    pub fn flush(&mut self) -> ProcessorOutput {
        // Process any remaining data
        if self.input_buffer.is_empty() {
            return ProcessorOutput::empty();
        }
        
        // For flush, we process whatever we have
        let input_copy = self.input_buffer.clone();
        self.input_buffer.clear();
        
        self.process_internal(&input_copy, true)
    }
    
    /// Reset processor state (reusable for new streams)
    pub fn reset(&mut self) {
        self.input_buffer.clear();
        self.audio_buffer.clear();
        self.decoder = None;
        self.rng = StdRng::seed_from_u64(42);
    }
    
    /// Process available data based on minimum chunk requirements
    fn process_available(&mut self) -> ProcessorOutput {
        // Only process if we have enough data for at least one frame
        let min_bytes = self.min_frame_size();
        
        if self.input_buffer.len() >= min_bytes {
            // Extract what we can process
            let to_process = self.input_buffer.drain(..).collect::<Vec<_>>();
            self.process_internal(&to_process, false)
        } else {
            ProcessorOutput::empty()
        }
    }
    
    /// Internal processing: bytes -> frames -> symbols -> audio -> channel -> demodulate -> decode -> bytes
    /// 
    /// This follows the proven pattern from generate_audio_batch and run_simulation
    fn process_internal(&mut self, input: &[u8], _is_flush: bool) -> ProcessorOutput {
        if input.is_empty() {
            return ProcessorOutput::empty();
        }
        
        // Convert input bytes to string then to bit stream (following existing pattern)
        let message = String::from_utf8_lossy(input);
        let payload_bits = string_to_bitstream(&message);
        
        if self.diagnostics_enabled {
            eprintln!("[PROCESSOR] Input: {:?}", message);
            eprintln!("[PROCESSOR] Payload bits: {} bits", payload_bits.len());
        }
        
        // Create encoder (following generate_audio_batch pattern)
        let mut encoder = StreamingFrameEncoder::new(
            &payload_bits,
            self.protocol.clone(),
            self.ldpc_suite.matrices.clone(),
        );
        
        if self.diagnostics_enabled {
            eprintln!("[PROCESSOR] Total frames: {}", encoder.total_frames);
        }
        
        // Generate all symbols for the message (INCLUDES PREAMBLE)
        let total_symbols = self.protocol.frame_layout.total_symbols * encoder.total_frames;
        let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(total_symbols);
        
        if self.diagnostics_enabled {
            eprintln!("[PROCESSOR] TX symbols: {} symbols (with preamble)", tx_symbols.len());
        }
        
        // Modulate symbols to audio using wrapper (calls existing modulator)
        let audio = self.modulator.modulate(&tx_symbols);
        
        if self.diagnostics_enabled {
            eprintln!("[PROCESSOR] Audio samples: {} samples", audio.len());
        }
        
        // Apply channel effects if configured
        let noisy_audio = if self.config.channel.enable_noise {
            let noise_std = Self::snr_to_noise_std(self.config.channel.snr_db);
            apply_audio_noise(&audio, noise_std, &mut self.rng)
        } else {
            audio.clone()
        };
        
        // Demodulate audio to symbols using wrapper (PREAMBLE STRIPPED HERE)
        let demod_result = self.demodulator.demodulate(&noisy_audio);
        let rx_symbols = demod_result.symbols;
        
        if self.diagnostics_enabled {
            eprintln!("[PROCESSOR] RX symbols: {} symbols (preamble stripped by demod)", rx_symbols.len());
            eprintln!("[PROCESSOR] SNR: {} dB", demod_result.snr_db);
        }
        
        // Initialize decoder if needed (maintain state for incremental processing)
        if self.decoder.is_none() {
            self.decoder = Some(StreamingSymbolDecoder::new(
                self.protocol.clone(),
                self.ldpc_suite.matrices.clone(),
            ));
        }
        
        let decoder = self.decoder.as_mut().unwrap();
        
        // Decode synchronized symbols (NO SYNC SEARCH - demod already did it)
        let decoded_bytes = decoder.process_synchronized_symbols(&rx_symbols);
        
        if self.diagnostics_enabled {
            eprintln!("[PROCESSOR] Decoded bytes: {} bytes", decoded_bytes.len());
            if !decoded_bytes.is_empty() {
                eprintln!("[PROCESSOR] Decoded string: {:?}", String::from_utf8_lossy(&decoded_bytes));
            }
        }
        
        let success = !decoded_bytes.is_empty();
        
        ProcessorOutput {
            decoded_bytes,
            ready: true,
            tx_symbols,
            rx_symbols,
            audio: noisy_audio,
            snr_db: demod_result.snr_db,
            success,
            error: None,
        }
    }
    
    /// Calculate noise standard deviation from SNR in dB
    fn snr_to_noise_std(snr_db: f32) -> f64 {
        let snr_linear = 10.0_f64.powf(snr_db as f64 / 10.0);
        let signal_power = 1.0; // Normalized
        let noise_power = signal_power / snr_linear;
        noise_power.sqrt()
    }
    
    /// Get minimum frame size in bytes
    fn min_frame_size(&self) -> usize {
        // Each frame carries message_bits of data
        let bits_per_frame = self.ldpc_suite.matrices.message_bits;
        (bits_per_frame + 7) / 8 // Convert to bytes, rounding up
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_processor_creation() {
        let processor = ChimeraProcessor::new_with_defaults();
        assert!(!processor.diagnostics_enabled);
    }
    
    #[test]
    fn test_processor_reset() {
        let mut processor = ChimeraProcessor::new_with_defaults();
        processor.input_buffer.push(1);
        processor.reset();
        assert!(processor.input_buffer.is_empty());
    }
}
