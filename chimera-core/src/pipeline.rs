//! Real-time DSP pipeline for audio processing
//!
//! This module provides a real-time capable pipeline suitable for both real-time
//! and batch processing operations. It supports chunk-by-chunk processing for
//! real-time audio applications and can also be used for offline batch processing.

use crate::config::{LDPCConfig, InternalProtocolConfig, UserSimulationConfig};
use crate::ldpc::LDPCSuite;
use crate::thz_carriers::{ThzCarrierProcessor, ThzCarrierConfig};
use crate::signal_processing::{
    modulation::{ModulationConfig, symbols_to_carrier_signal},
    demodulation::DemodulationConfig,
    spectrum::compute_baseband_spectrum,
};
use crate::diagnostics::{
    metrics::compute_constellation_evm,
    constellation::normalize_constellation,
};
use num_complex::Complex;
use rand::rngs::StdRng;
use rand::SeedableRng;

// Cross-platform timing abstraction
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

/// Cross-platform high-precision timing
/// 
/// Reserved for future rate-limiting implementation in real-time streaming mode.
/// Currently unused but kept for when chunk-by-chunk processing with timing
/// constraints is needed.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct PrecisionTimer {
    #[cfg(not(target_arch = "wasm32"))]
    start: Instant,
    #[cfg(target_arch = "wasm32")]
    start_ms: f64,
}

#[allow(dead_code)]
impl PrecisionTimer {
    fn now() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self { start: Instant::now() }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self { start_ms: now() }
        }
    }
    
    fn elapsed_ms(&self) -> f64 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.start.elapsed().as_secs_f64() * 1000.0
        }
        #[cfg(target_arch = "wasm32")]
        {
            now() - self.start_ms
        }
    }
}

/// Frame layout information
#[derive(Debug, Clone, Default)]
pub struct FrameLayoutInfo {
    pub sync_bytes: usize,
    pub data_bytes: usize,
    pub parity_bytes: usize,
    pub total_bytes: usize,
    pub target_id_bytes: usize,
    pub command_type_bytes: usize,
}

/// Pre-channel (transmitter) diagnostics
#[derive(Debug, Clone, Default)]
pub struct PreChannelDiagnostics {
    pub frame_count: usize,
    pub total_frames: usize,
    pub symbol_count: usize,
    pub tx_constellation_i: Vec<f32>,
    pub tx_constellation_q: Vec<f32>,
    pub tx_spectrum_magnitude: Vec<f32>,
    pub spectrum_freq_start_hz: f32,
    pub spectrum_freq_end_hz: f32,
    pub carrier_freq_hz: f64,
    pub symbol_rate_hz: u32,
    pub modulation_type: String,
    pub fec_rate: String,
    pub frame_layout: FrameLayoutInfo,
}

/// Post-channel (receiver) diagnostics
#[derive(Debug, Clone, Default)]
pub struct PostChannelDiagnostics {
    pub rx_constellation_i: Vec<f32>,
    pub rx_constellation_q: Vec<f32>,
    pub rx_spectrum_magnitude: Vec<f32>,
    pub spectrum_freq_start_hz: f32,
    pub spectrum_freq_end_hz: f32,
    pub timing_error: Vec<f32>,
    pub frequency_offset_hz: f32,
    pub phase_offset_rad: f32,
    pub evm_percent: f32,
    pub ber_instantaneous: f32,
    pub ber_average: f32,
    pub sync_status: bool,
    pub lock_status: String,
}

/// Frame data for real-time display
#[derive(Debug, Clone, Default)]
pub struct FrameData {
    pub frame_number: usize,
    pub sync_data: Vec<u8>,
    pub payload_data: Vec<u8>,
    pub parity_data: Vec<u8>,
    pub decoded_text: String,
    pub symbol_progress: usize,
}

/// FSK layer state information (nested 1 bit/second modulation)
#[derive(Debug, Clone, Default)]
pub struct FSKState {
    /// Current FSK frequency in Hz (12000 ± 1 Hz)
    pub current_frequency_hz: f64,
    /// Frequency deviation from center (12 kHz) in Hz
    pub frequency_deviation_hz: f64,
    /// Current FSK bit value (0 or 1)
    pub current_bit: u8,
    /// Current bit index in FSK stream
    pub bit_index: usize,
    /// History of recent FSK bits for visualization
    pub bit_history: Vec<u8>,
    /// Number of QPSK symbols per FSK bit (16 symbols at 16 sym/s = 1 second)
    pub symbols_per_bit: usize,
    /// FSK bit rate in Hz
    pub bit_rate_hz: f64,
}

/// Output from a single processing chunk
#[derive(Clone, Debug, Default)]
pub struct RealtimeOutput {
    /// Audio samples (modulated carrier)
    /// Always Float32 at 48 kHz for Web Audio API compatibility
    pub audio_samples: Vec<f32>,
    
    /// Pre-channel diagnostics
    pub pre_channel: PreChannelDiagnostics,
    
    /// Post-channel diagnostics  
    pub post_channel: PostChannelDiagnostics,
    
    /// Decoded data bytes
    pub decoded_data: Vec<u8>,
    /// Decoded text
    pub decoded_text: String,
    
    /// Performance metrics
    pub frames_processed: usize,
    pub symbols_decoded: usize,
    pub fec_corrections: usize,
    
    /// Current frame data
    pub current_frame_data: FrameData,
    
    /// FSK layer state (nested 1 bit/second modulation)
    pub fsk_state: Option<FSKState>,
}

/// Real-time capable DSP pipeline
/// Supports both real-time chunk processing and batch mode operation
pub struct RealtimePipeline {
    config: UserSimulationConfig,
    protocol: InternalProtocolConfig,
    ldpc_config: LDPCConfig,
    ldpc_suite: LDPCSuite,
    
    // Runtime adjustable channel parameters
    snr_db: f64,
    link_loss_db: f64,
    
    // Streaming encoder/decoder
    encoder: Option<crate::encoder::StreamingFrameEncoder>,
    decoder: Option<crate::decoder::StreamingSymbolDecoder>,
    
    // THz carrier processor for AID effect
    thz_processor: ThzCarrierProcessor,
    is_active_mode: bool,
    
    // State tracking
    frame_count: usize,
    total_frames: usize,
    total_symbols_generated: usize,
    total_symbols_decoded: usize,
    total_errors: usize,
    ber_accumulator: f32,
    symbols_per_update: usize,
    
    // Accumulators for diagnostics
    tx_symbols_buffer: Vec<Complex<f64>>,
    rx_symbols_buffer: Vec<Complex<f64>>,
    
    // Noise generation
    rng: StdRng,
    signal_power: f64,
    noise_std: f64,
}

impl RealtimePipeline {
    /// Create a new real-time capable pipeline with the given configuration
    /// Default SNR=20dB, link_loss=0dB - use update_channel_params to adjust at runtime
    pub fn new(
        sim: UserSimulationConfig,
        protocol: InternalProtocolConfig,
        ldpc: LDPCConfig,
    ) -> Self {
        Self::with_channel_params(sim, protocol, ldpc, 20.0, 0.0)
    }
    
    /// Create pipeline with specific channel parameters
    pub fn with_channel_params(
        sim: UserSimulationConfig,
        protocol: InternalProtocolConfig,
        ldpc: LDPCConfig,
        snr_db: f64,
        link_loss_db: f64,
    ) -> Self {
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
        
        // Update less frequently to accumulate more samples for better spectrum resolution
        // At 16 sym/s, updating every 16 symbols = 1 second updates
        let symbols_per_update = 16;
        
        // Use hardware entropy instead of RNG seed
        let rng = StdRng::from_entropy();
        
        // Pre-calculate noise parameters
        // QPSK has signal power of 1.0 (normalized)
        let signal_power = 1.0;
        let channel = crate::utils::ChannelParams::from_db(snr_db, link_loss_db, signal_power);
        let noise_std = channel.noise_std;
        
        // Initialize THz carrier processor
        let mut thz_config = ThzCarrierConfig::default();
        thz_config.bypass_simulation = sim.bypass_thz_simulation;
        let thz_processor = ThzCarrierProcessor::new(thz_config, crate::config::SystemConfig::SAMPLE_RATE as f64);
        
        Self {
            config: sim,
            protocol,
            ldpc_config: ldpc,
            ldpc_suite,
            snr_db,
            link_loss_db,
            encoder: None,
            decoder: None,
            thz_processor,
            is_active_mode: false,
            frame_count: 0,
            total_frames: 0,
            total_symbols_generated: 0,
            total_symbols_decoded: 0,
            total_errors: 0,
            ber_accumulator: 0.0,
            symbols_per_update,
            tx_symbols_buffer: Vec::new(),
            rx_symbols_buffer: Vec::new(),
            rng,
            signal_power,
            noise_std,
        }
    }
    
    /// Set modulation mode (idle vs active)
    /// - Idle mode: Low modulation depth (<5%) for baseline carrier
    /// - Active mode: High modulation depth (70-80%) for data transmission
    pub fn set_modulation_mode(&mut self, active: bool) {
        self.is_active_mode = active;
        let depth = if active { 0.75 } else { 0.03 };
        self.thz_processor.set_modulation_depth(depth);
    }
    
    /// Set custom modulation depth (0.0 to 1.0)
    /// Typical values:
    /// - Idle: 0.01 to 0.05 (1-5%)
    /// - Active: 0.70 to 0.80 (70-80%)
    pub fn set_modulation_depth(&mut self, depth: f32) {
        self.thz_processor.set_modulation_depth(depth);
    }
    
    /// Set mixing coefficient for third-order intermodulation
    /// Higher values increase the non-linear mixing effect
    pub fn set_mixing_coefficient(&mut self, coefficient: f32) {
        self.thz_processor.set_mixing_coefficient(coefficient);
    }
    
    /// Enable/disable QPSK modulation (debug control)
    /// When false, outputs unmodulated carrier (pure sine wave)
    pub fn set_qpsk_enabled(&mut self, enabled: bool) {
        self.protocol.enable_qpsk = enabled;
    }
    
    /// Enable/disable FSK frequency dithering (debug control)
    /// When false, uses fixed carrier frequency (12 kHz instead of 11999/12001 Hz)
    pub fn set_fsk_enabled(&mut self, enabled: bool) {
        self.protocol.enable_fsk = enabled;
    }
    
    /// Process a chunk of data in real-time or batch mode
    /// Returns diagnostics and audio output for the processed chunk
    /// 
    /// This unified method handles any chunk size:
    /// - Small chunks (1-100 symbols): Real-time streaming mode
    /// - Medium chunks (100-1000 symbols): Interactive processing
    /// - Large chunks (1000+ symbols): Batch processing mode
    /// 
    /// The pipeline automatically adapts to the chunk size and maintains
    /// consistent behavior across all modes.
    pub fn process_chunk(&mut self, _input: &[u8]) -> RealtimeOutput {
        // No rate limiting - process and emit symbols as fast as possible for real-time visualization
        
        // Initialize encoder on first call
        if self.encoder.is_none() {
            let payload_bits = crate::utils::string_to_bitstream(&self.config.message);
            let encoder = crate::encoder::StreamingFrameEncoder::new(
                &payload_bits,
                self.protocol.clone(),
                self.ldpc_suite.matrices.clone(),
            );
            self.total_frames = encoder.total_frames;
            self.encoder = Some(encoder);
        }
        
        // Initialize decoder on first call
        if self.decoder.is_none() {
            let decoder = crate::decoder::StreamingSymbolDecoder::new(
                self.protocol.clone(),
                self.ldpc_suite.matrices.clone(),
            );
            self.decoder = Some(decoder);
        }
        
        let encoder = self.encoder.as_mut().unwrap();
        let decoder = self.decoder.as_mut().unwrap();
        
        // Cache values we'll need later (before mutable borrows)
        let sample_rate = crate::config::SystemConfig::SAMPLE_RATE;
        let symbol_rate = self.protocol.qpsk_symbol_rate;
        let carrier_freq = self.protocol.carrier_freq_hz;
        
        // Generate next batch of symbols
        let (tx_symbols, frame_changed, current_frame, symbol_in_frame, _is_complete) = 
            encoder.get_next_symbols(self.symbols_per_update);
        
        if frame_changed {
            self.frame_count += 1;
        }
        
        if tx_symbols.is_empty() {
            return RealtimeOutput::default();
        }
        
        // ===== END-TO-END AUDIO PATH =====
        // Generate audio from TX symbols with FSK+QPSK modulation
        let modulation_config = ModulationConfig {
            sample_rate,
            symbol_rate,
            carrier_freq,
        };
        let base_audio = symbols_to_carrier_signal(&tx_symbols, &modulation_config);
        
        // Apply THz carrier modulation and mixing to simulate AID effect
        // Check bypass_simulation flag to skip THz processing for validation
        let mixed_audio = if !self.thz_processor.config().bypass_simulation {
            let modulated_thz = self.thz_processor.modulate_data_carrier(&base_audio);
            self.thz_processor.nonlinear_mixing(&modulated_thz)
        } else {
            // Bypass THz processing - use audio directly
            base_audio.clone()
        };
        
        
        // Demodulate audio back to IQ symbols for decoding
        let demod_config = DemodulationConfig {
            sample_rate,
            symbol_rate,
            carrier_freq,
        };
        let rx_symbols: Vec<Complex<f64>> = crate::signal_processing::demodulation::audio_to_symbols(&base_audio, &demod_config);
        // Note: Using simple demodulation without SNR measurement from demod itself
        
        // Process through decoder
        let (decoded_bits, frame_complete, dec_frame_index, _symbols_in_dec_frame, diagnostics) = 
            decoder.process_symbols(&rx_symbols);
        
        // Track decoder frame completion
        if frame_complete {
            // Decoder completed a frame - this is what tests expect
            // Don't rely on encoder's frame count
        }
        
        // Extract FSK state information from decoder (not encoder!)
        // The decoder actually demodulates the FSK layer from the received signal
        // Do this RIGHT AFTER decoder.process_symbols() while we still have the mutable borrow
        let _fsk_frequency = decoder.get_fsk_frequency();
        let _fsk_current_bit = decoder.get_fsk_bit();
        let fsk_detected_bits = decoder.get_fsk_bits().to_vec(); // Copy to owned vec
        
        // Get FSK bit history for display (last 16 bits, centered on current)
        let mut fsk_bit_history = Vec::new();
        if fsk_detected_bits.len() >= 16 {
            fsk_bit_history.extend_from_slice(&fsk_detected_bits[fsk_detected_bits.len() - 16..]);
        } else {
            // Pad with zeros if we don't have enough history yet
            fsk_bit_history.resize(16 - fsk_detected_bits.len(), 0);
            fsk_bit_history.extend_from_slice(&fsk_detected_bits);
        }
        
        // Update counters
        self.total_symbols_generated += tx_symbols.len();
        self.total_symbols_decoded += rx_symbols.len();
        
        // Buffer symbols for spectrum (keep last 2048 for good frequency resolution)
        self.tx_symbols_buffer.extend(tx_symbols.iter().copied());
        if self.tx_symbols_buffer.len() > 2048 {
            self.tx_symbols_buffer.drain(0..self.tx_symbols_buffer.len() - 2048);
        }
        
        self.rx_symbols_buffer.extend(rx_symbols.iter().copied());
        if self.rx_symbols_buffer.len() > 2048 {
            self.rx_symbols_buffer.drain(0..self.rx_symbols_buffer.len() - 2048);
        }
        
        // For spectrum: compute directly from IQ symbols (baseband spectrum)
        // Use sliding window of last 128 symbols (8 seconds at 16 sym/s)
        // This updates continuously giving real-time feedback
        const SPECTRUM_SYMBOLS: usize = 128; // Much smaller - only 8 seconds worth
        
        // Always compute spectrum if we have at least 32 symbols (2 seconds)
        let tx_spectrum = if self.tx_symbols_buffer.len() >= 32 {
            let symbols_to_use = self.tx_symbols_buffer.len().min(SPECTRUM_SYMBOLS);
            let start_idx = self.tx_symbols_buffer.len() - symbols_to_use;
            compute_baseband_spectrum(&self.tx_symbols_buffer[start_idx..])
        } else {
            Vec::new()
        };
        
        let rx_spectrum = if self.rx_symbols_buffer.len() >= 32 {
            let symbols_to_use = self.rx_symbols_buffer.len().min(SPECTRUM_SYMBOLS);
            let start_idx = self.rx_symbols_buffer.len() - symbols_to_use;
            compute_baseband_spectrum(&self.rx_symbols_buffer[start_idx..])
        } else {
            Vec::new()
        };
        
        // Build output
        let mut output = RealtimeOutput::default();
        
        // For constellation display: use only recent 256 symbols for clarity
        let tx_constellation_symbols = if self.tx_symbols_buffer.len() > 256 {
            &self.tx_symbols_buffer[self.tx_symbols_buffer.len() - 256..]
        } else {
            &self.tx_symbols_buffer[..]
        };
        
        let rx_constellation_symbols = if self.rx_symbols_buffer.len() > 256 {
            &self.rx_symbols_buffer[self.rx_symbols_buffer.len() - 256..]
        } else {
            &self.rx_symbols_buffer[..]
        };
        
        let (tx_i_norm, tx_q_norm) = normalize_constellation(tx_constellation_symbols);
        let (rx_i_norm, rx_q_norm) = normalize_constellation(rx_constellation_symbols);
        
        // Pre-channel diagnostics
        output.pre_channel = PreChannelDiagnostics {
            frame_count: current_frame + 1,
            total_frames: self.total_frames,
            symbol_count: tx_symbols.len(),
            tx_constellation_i: tx_i_norm,
            tx_constellation_q: tx_q_norm,
            tx_spectrum_magnitude: tx_spectrum,
            spectrum_freq_start_hz: 11900.0, // 12 kHz - 100 Hz
            spectrum_freq_end_hz: 12100.0,   // 12 kHz + 100 Hz (200 Hz span for 32 Hz signal)
            carrier_freq_hz: self.protocol.carrier_freq_hz,
            symbol_rate_hz: self.protocol.qpsk_symbol_rate as u32,
            modulation_type: "QPSK".to_string(),
            fec_rate: format!("{}/{}", self.ldpc_suite.matrices.message_bits, self.ldpc_suite.matrices.codeword_bits),
            frame_layout: FrameLayoutInfo {
                sync_bytes: (self.protocol.frame_layout.sync_symbols * 2) / 8,
                data_bytes: (self.protocol.frame_layout.data_payload_symbols * 2) / 8,
                parity_bytes: (self.protocol.frame_layout.ecc_symbols * 2) / 8,
                target_id_bytes: (self.protocol.frame_layout.target_id_symbols * 2) / 8,
                command_type_bytes: (self.protocol.frame_layout.command_type_symbols * 2) / 8,
                total_bytes: (self.protocol.frame_layout.total_symbols * 2) / 8,
            },
        };
        
        // Calculate EVM from current chunk
        // Use constellation-based EVM since TX/RX symbols may not be perfectly aligned
        // Skip first 10 symbols to allow demodulator to stabilize
        let evm_percent = if rx_symbols.len() > 10 {
            compute_constellation_evm(&rx_symbols[10..])
        } else if !rx_symbols.is_empty() {
            compute_constellation_evm(&rx_symbols)
        } else {
            0.0
        };
        
        // Post-channel diagnostics
        output.post_channel = PostChannelDiagnostics {
            rx_constellation_i: rx_i_norm,
            rx_constellation_q: rx_q_norm,
            rx_spectrum_magnitude: rx_spectrum,
            spectrum_freq_start_hz: 11900.0,
            spectrum_freq_end_hz: 12100.0,
            timing_error: diagnostics.timing_error.iter().map(|&v| v as f32).collect(),
            frequency_offset_hz: 0.0,
            phase_offset_rad: 0.0,
            evm_percent,
            ber_instantaneous: 0.0, // TODO: calculate from decoder
            ber_average: self.ber_accumulator,
            sync_status: decoder.is_synced(),
            lock_status: if !decoder.is_synced() { "SEARCHING".to_string() }
                        else if self.frame_count < 3 { "ACQUIRING".to_string() }
                        else { "LOCKED".to_string() },
        };
        
        // Decoded text - convert bits to bytes first
        let all_decoded_bits = decoder.get_decoded_payload();
        let all_decoded_bytes = crate::utils::pack_bits(&all_decoded_bits);
        output.decoded_text = String::from_utf8_lossy(&all_decoded_bytes)
            .trim_end_matches('\u{0}')
            .to_string();
        
        // Decoded data from this chunk (if frame was completed)
        if !decoded_bits.is_empty() {
            // Convert bits to bytes
            output.decoded_data = crate::utils::pack_bits(&decoded_bits);
        }
        
        // Performance metrics
        output.frames_processed = dec_frame_index; // Use decoder's frame count, not encoder's
        output.symbols_decoded = self.total_symbols_decoded;
        output.fec_corrections = 0;
        
        // Frame data for display
        let frame_bits = encoder.get_current_frame_bits();
        let sync_bit_count = self.protocol.frame_layout.sync_symbols * 2;
        let sync_bytes: Vec<u8> = frame_bits.iter()
            .take(sync_bit_count)
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|bits| {
                bits.iter().fold(0u8, |acc, &&bit| (acc << 1) | bit)
            })
            .collect();
        
        let payload_start = (self.protocol.frame_layout.sync_symbols + 
                           self.protocol.frame_layout.target_id_symbols +
                           self.protocol.frame_layout.command_type_symbols) * 2;
        let payload_bit_count = self.protocol.frame_layout.data_payload_symbols * 2;
        let payload_bytes: Vec<u8> = if frame_bits.len() >= payload_start + payload_bit_count {
            frame_bits[payload_start..payload_start + payload_bit_count]
                .chunks(8)
                .map(|bits| {
                    bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit)
                })
                .collect()
        } else {
            Vec::new()
        };
        
        let parity_start = payload_start + payload_bit_count;
        let parity_bit_count = self.protocol.frame_layout.ecc_symbols * 2;
        let parity_bytes: Vec<u8> = if frame_bits.len() >= parity_start + parity_bit_count {
            frame_bits[parity_start..parity_start + parity_bit_count]
                .chunks(8)
                .map(|bits| {
                    bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit)
                })
                .collect()
        } else {
            Vec::new()
        };
        
        output.current_frame_data = FrameData {
            frame_number: current_frame + 1,
            sync_data: sync_bytes,
            payload_data: payload_bytes,
            parity_data: parity_bytes,
            decoded_text: if !decoded_bits.is_empty() {
                String::from_utf8_lossy(&decoded_bits).to_string()
            } else {
                String::new()
            },
            symbol_progress: symbol_in_frame,
        };
        
        // Use the audio that was already generated in the end-to-end path above
        output.audio_samples = mixed_audio;
        
        // Extract FSK state information from decoder (not encoder!)
        // The decoder actually demodulates the FSK layer from the received signal
        let fsk_frequency = decoder.get_fsk_frequency();
        let fsk_current_bit = decoder.get_fsk_bit();
        let fsk_detected_bits = decoder.get_fsk_bits();
        
        // Get FSK bit history for display (last 16 bits, centered on current)
        let mut fsk_bit_history = Vec::new();
        if fsk_detected_bits.len() >= 16 {
            fsk_bit_history.extend_from_slice(&fsk_detected_bits[fsk_detected_bits.len() - 16..]);
        } else {
            // Pad with zeros if we don't have enough history yet
            fsk_bit_history.resize(16 - fsk_detected_bits.len(), 0);
            fsk_bit_history.extend_from_slice(fsk_detected_bits);
        }
        
        // Add FSK layer state (from decoder - actual demodulated FSK)
        output.fsk_state = Some(FSKState {
            current_frequency_hz: fsk_frequency,
            frequency_deviation_hz: fsk_frequency - 12000.0,
            current_bit: fsk_current_bit,
            bit_index: fsk_detected_bits.len(),
            bit_history: fsk_bit_history,
            symbols_per_bit: 16,
            bit_rate_hz: 1.0,
        });
        
        output
    }
    
    /// Get current pipeline configuration
    pub fn get_config(&self) -> PipelineConfig {
        PipelineConfig {
            simulation: self.config.clone(),
            protocol: self.protocol.clone(),
        }
    }
    
    /// Reconfigure the pipeline
    /// Note: snr_db and link_loss_db should be passed separately or use update_channel_params
    pub fn reconfigure(&mut self, sim: UserSimulationConfig, protocol: InternalProtocolConfig, ldpc: LDPCConfig) {
        // Keep existing channel parameters
        let snr_db = self.snr_db;
        let link_loss_db = self.link_loss_db;
        
        // Pre-calculate noise parameters
        let signal_power = 1.0;
        let channel = crate::utils::ChannelParams::from_db(snr_db, link_loss_db, signal_power);
        let noise_std = channel.noise_std;
        
        // Use hardware entropy
        let rng = StdRng::from_entropy();
        
        self.config = sim;
        self.protocol = protocol.clone();
        self.ldpc_config = ldpc;
        self.ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &self.ldpc_config);
        self.encoder = None;
        self.decoder = None;
        self.frame_count = 0;
        self.total_frames = 0;
        self.total_symbols_generated = 0;
        self.total_symbols_decoded = 0;
        self.total_errors = 0;
        self.ber_accumulator = 0.0;
        self.symbols_per_update = 4;
        self.tx_symbols_buffer.clear();
        self.rx_symbols_buffer.clear();
        self.rng = rng;
        self.signal_power = signal_power;
        self.noise_std = noise_std;
    }
    
    /// Update channel parameters (SNR and link loss) without resetting the pipeline
    pub fn update_channel_params(&mut self, snr_db: f64, link_loss_db: f64) {
        // Update stored channel params
        self.snr_db = snr_db;
        self.link_loss_db = link_loss_db;
        
        // Recalculate noise parameters
        let link_loss_linear = 10f64.powf(link_loss_db / 10.0);
        let attenuated_signal_power = self.signal_power / link_loss_linear;
        let snr_linear = 10f64.powf(snr_db / 10.0);
        let noise_variance = if snr_linear > 0.0 {
            attenuated_signal_power / snr_linear
        } else {
            0.0
        };
        self.noise_std = (noise_variance / 2.0).sqrt();
    }
    
    /// Get current SNR
    pub fn get_snr(&self) -> f64 {
        self.snr_db
    }
    
    /// Get current link loss
    pub fn get_link_loss(&self) -> f64 {
        self.link_loss_db
    }
    
    /// Update message (placeholder - needs proper implementation to wait for transmission)
    pub fn update_message(&mut self, _message: String) -> Result<(), String> {
        // TODO: Implement message queue that waits for current transmission to complete
        Err("Message updates not yet implemented".to_string())
    }
    
    /// Update command (placeholder)
    pub fn update_command(&mut self, _command: String) -> Result<(), String> {
        // TODO: Implement command updates
        Err("Command updates not yet implemented".to_string())
    }
    
    /// Update target ID (placeholder)
    pub fn update_target_id(&mut self, _target_id: String) -> Result<(), String> {
        // TODO: Implement target ID updates
        Err("Target ID updates not yet implemented".to_string())
    }
    
    /// Set TX gain (placeholder)
    pub fn set_tx_gain(&mut self, _gain: f32) {
        // TODO: Implement TX gain control
    }
    
    /// Set RX gain (placeholder)
    pub fn set_rx_gain(&mut self, _gain: f32) {
        // TODO: Implement RX gain control
    }
}

/// Configuration for real-time pipeline
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    pub simulation: UserSimulationConfig,
    pub protocol: InternalProtocolConfig,
}


