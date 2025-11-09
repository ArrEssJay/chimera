//! Streaming DSP pipeline for real-time audio processing
//!
//! This module provides a streaming version of the batch-mode simulation,
//! allowing chunk-by-chunk processing suitable for real-time audio applications.

use crate::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use crate::ldpc::{LDPCSuite};
use num_complex::Complex;
use rustfft::{FftPlanner, num_complex::Complex32};
use std::f64::consts::TAU;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
#[derive(Debug, Clone, Copy)]
struct PrecisionTimer {
    #[cfg(not(target_arch = "wasm32"))]
    start: Instant,
    #[cfg(target_arch = "wasm32")]
    start_ms: f64,
}

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
    pub snr_estimate_db: f32,
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

/// Output from a single processing chunk
#[derive(Clone, Debug, Default)]
pub struct StreamingOutput {
    /// Audio samples (modulated carrier)
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
}

/// Streaming DSP pipeline
pub struct StreamingPipeline {
    config: SimulationConfig,
    protocol: ProtocolConfig,
    ldpc_config: LDPCConfig,
    ldpc_suite: LDPCSuite,
    
    // Streaming encoder/decoder
    encoder: Option<crate::encoder::StreamingFrameEncoder>,
    decoder: Option<crate::decoder::StreamingSymbolDecoder>,
    
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

impl StreamingPipeline {
    /// Create a new streaming pipeline with the given configuration
    pub fn new(
        sim: SimulationConfig,
        protocol: ProtocolConfig,
        ldpc: LDPCConfig,
    ) -> Self {
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
        
        // Update symbols per batch - emit more frequently for smoother UI updates
        let symbols_per_update = 4;
        
        let rng = match sim.rng_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
        // Pre-calculate noise parameters
        // QPSK has signal power of 1.0 (normalized)
        let signal_power = 1.0;
        let link_loss_linear = 10f64.powf(sim.link_loss_db / 10.0);
        let attenuated_signal_power = signal_power / link_loss_linear;
        let snr_linear = 10f64.powf(sim.snr_db / 10.0);
        let noise_variance = if snr_linear > 0.0 {
            attenuated_signal_power / snr_linear
        } else {
            0.0
        };
        let noise_std = (noise_variance / 2.0).sqrt();
        
        Self {
            config: sim,
            protocol,
            ldpc_config: ldpc,
            ldpc_suite,
            encoder: None,
            decoder: None,
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
    
    /// Process a chunk - now emits updates every N symbols instead of every frame
    pub fn process_chunk(&mut self, _input: &[u8]) -> StreamingOutput {
        // No rate limiting - process and emit symbols as fast as possible for real-time visualization
        
        // Initialize encoder on first call
        if self.encoder.is_none() {
            let payload_bits = crate::utils::string_to_bitstream(&self.config.plaintext_source);
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
        
        // Generate next batch of symbols
        let (tx_symbols, frame_changed, current_frame, symbol_in_frame, _is_complete) = 
            encoder.get_next_symbols(self.symbols_per_update);
        
        if frame_changed {
            self.frame_count += 1;
        }
        
        if tx_symbols.is_empty() {
            return StreamingOutput::default();
        }
        
        // Apply channel effects (attenuation + noise)
        let link_loss_linear = 10f64.powf(self.config.link_loss_db / 10.0);
        let attenuation_factor = if link_loss_linear > 0.0 {
            1.0 / link_loss_linear.sqrt()
        } else {
            1.0
        };
        
        let normal = rand_distr::StandardNormal;
        let mut rx_symbols = Vec::with_capacity(tx_symbols.len());
        
        for tx_symbol in &tx_symbols {
            // Apply attenuation
            let attenuated = tx_symbol * attenuation_factor;
            
            // Add AWGN
            let noise_i: f64 = self.rng.sample::<f64, _>(normal) * self.noise_std;
            let noise_q: f64 = self.rng.sample::<f64, _>(normal) * self.noise_std;
            let rx_symbol = Complex::new(
                attenuated.re + noise_i,
                attenuated.im + noise_q
            );
            
            rx_symbols.push(rx_symbol);
        }
        
        // Process through decoder
        let (decoded_bits, _frame_complete, _dec_frame, _symbols_in_dec_frame, diagnostics) = 
            decoder.process_symbols(&rx_symbols);
        
        // Update counters
        self.total_symbols_generated += tx_symbols.len();
        self.total_symbols_decoded += rx_symbols.len();
        
        // Buffer symbols for constellation display (keep last 256)
        self.tx_symbols_buffer.extend(tx_symbols.iter().copied());
        if self.tx_symbols_buffer.len() > 256 {
            self.tx_symbols_buffer.drain(0..self.tx_symbols_buffer.len() - 256);
        }
        
        self.rx_symbols_buffer.extend(rx_symbols.iter().copied());
        if self.rx_symbols_buffer.len() > 256 {
            self.rx_symbols_buffer.drain(0..self.rx_symbols_buffer.len() - 256);
        }
        
        // For spectrum: use only recent symbols (limit to prevent performance degradation)
        // 256 symbols at 16 sym/s = 16 seconds of integration, plenty for good spectrum
        const MAX_SYMBOLS_FOR_SPECTRUM: usize = 256;
        
        let tx_symbols_for_spectrum = if self.tx_symbols_buffer.len() > MAX_SYMBOLS_FOR_SPECTRUM {
            &self.tx_symbols_buffer[self.tx_symbols_buffer.len() - MAX_SYMBOLS_FOR_SPECTRUM..]
        } else {
            &self.tx_symbols_buffer[..]
        };
        
        let rx_symbols_for_spectrum = if self.rx_symbols_buffer.len() > MAX_SYMBOLS_FOR_SPECTRUM {
            &self.rx_symbols_buffer[self.rx_symbols_buffer.len() - MAX_SYMBOLS_FOR_SPECTRUM..]
        } else {
            &self.rx_symbols_buffer[..]
        };
        
        // Generate modulated carrier samples for spectrum analysis
        let sample_rate = self.config.sample_rate;
        let symbol_rate = self.protocol.qpsk_symbol_rate;
        let carrier_freq = self.protocol.carrier_freq_hz;
        let tx_carrier_samples = Self::symbols_to_carrier_signal(tx_symbols_for_spectrum, sample_rate, symbol_rate, carrier_freq);
        let rx_carrier_samples = Self::symbols_to_carrier_signal(rx_symbols_for_spectrum, sample_rate, symbol_rate, carrier_freq);
        
        // Build output
        let mut output = StreamingOutput::default();
        
        // Normalize constellation points
        let normalize_constellation = |symbols: &[Complex<f64>]| -> (Vec<f32>, Vec<f32>) {
            if symbols.is_empty() {
                return (Vec::new(), Vec::new());
            }
            
            let scale = 1.0 / std::f64::consts::SQRT_2;
            let i_vals: Vec<f32> = symbols.iter().map(|c| (c.re * scale) as f32).collect();
            let q_vals: Vec<f32> = symbols.iter().map(|c| (c.im * scale) as f32).collect();
            
            (i_vals, q_vals)
        };
        
        let (tx_i_norm, tx_q_norm) = normalize_constellation(&self.tx_symbols_buffer);
        let (rx_i_norm, rx_q_norm) = normalize_constellation(&self.rx_symbols_buffer);
        
        // Compute spectra with frequency information
        let (tx_spectrum, tx_freq_start, tx_freq_end) = Self::compute_carrier_spectrum(&tx_carrier_samples, self.config.sample_rate, self.protocol.carrier_freq_hz);
        
        // Pre-channel diagnostics
        output.pre_channel = PreChannelDiagnostics {
            frame_count: current_frame + 1,
            total_frames: self.total_frames,
            symbol_count: tx_symbols.len(),
            tx_constellation_i: tx_i_norm,
            tx_constellation_q: tx_q_norm,
            tx_spectrum_magnitude: tx_spectrum,
            spectrum_freq_start_hz: tx_freq_start,
            spectrum_freq_end_hz: tx_freq_end,
            carrier_freq_hz: self.protocol.carrier_freq_hz,
            symbol_rate_hz: self.protocol.qpsk_symbol_rate as u32,
            modulation_type: "QPSK".to_string(),
            fec_rate: format!("{}/{}", self.ldpc_suite.matrices.message_bits, self.ldpc_suite.matrices.codeword_bits),
            frame_layout: FrameLayoutInfo {
                sync_bytes: (self.protocol.frame_layout.sync_symbols * 2) / 8,
                data_bytes: (self.protocol.frame_layout.data_payload_symbols * 2) / 8,
                parity_bytes: (self.protocol.frame_layout.ecc_symbols * 2) / 8,
                total_bytes: (self.protocol.frame_layout.total_symbols * 2) / 8,
            },
        };
        
        // Calculate EVM and SNR from buffered symbols
        let evm_percent = compute_evm(&tx_symbols, &rx_symbols);
        let snr_estimate_db = estimate_snr(&rx_symbols);
        
        // Compute RX spectrum
        let (rx_spectrum, rx_freq_start, rx_freq_end) = Self::compute_carrier_spectrum(&rx_carrier_samples, self.config.sample_rate, self.protocol.carrier_freq_hz);
        
        // Post-channel diagnostics
        output.post_channel = PostChannelDiagnostics {
            rx_constellation_i: rx_i_norm,
            rx_constellation_q: rx_q_norm,
            rx_spectrum_magnitude: rx_spectrum,
            spectrum_freq_start_hz: rx_freq_start,
            spectrum_freq_end_hz: rx_freq_end,
            timing_error: diagnostics.timing_error.iter().map(|&v| v as f32).collect(),
            frequency_offset_hz: 0.0,
            phase_offset_rad: 0.0,
            evm_percent,
            snr_estimate_db,
            ber_instantaneous: 0.0, // TODO: calculate from decoder
            ber_average: self.ber_accumulator,
            sync_status: decoder.is_synced(),
            lock_status: if !decoder.is_synced() { "SEARCHING".to_string() }
                        else if self.frame_count < 3 { "ACQUIRING".to_string() }
                        else { "LOCKED".to_string() },
        };
        
        // Decoded text
        let all_decoded = decoder.get_decoded_payload();
        output.decoded_text = String::from_utf8_lossy(&all_decoded).to_string();
        
        // Performance metrics
        output.frames_processed = self.frame_count;
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
        
        // Generate audio samples for the symbols
        output.audio_samples = self.symbols_to_audio_incremental(&tx_symbols);
        
        output
    }
    
    fn compute_spectrum_with_scaling(symbols: &[Complex<f64>]) -> Vec<f32> {
        // Use larger FFT for better frequency resolution
        let fft_size = 2048.min(symbols.len().next_power_of_two().max(2048));
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        
        let mut buffer: Vec<Complex32> = symbols.iter()
            .take(fft_size)
            .map(|c| Complex32::new(c.re as f32, c.im as f32))
            .collect();
        
        // Pad if needed
        while buffer.len() < fft_size {
            buffer.push(Complex32::new(0.0, 0.0));
        }
        
        // Apply Hann window to reduce spectral leakage
        for i in 0..buffer.len() {
            let window_value = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 
                / (buffer.len() as f32 - 1.0)).cos());
            buffer[i] = buffer[i] * window_value;
        }
        
        fft.process(&mut buffer);
        
        // Hann window power scaling factor
        let window_power: f32 = 0.375;
        let scale = 1.0 / ((fft_size as f32) * window_power.sqrt());
        
        // Convert to power spectrum in dB with proper scaling
        buffer.iter()
            .map(|c| {
                let power = c.norm_sqr() * scale * scale;
                if power > 1e-10 {
                    10.0 * power.log10()
                } else {
                    -100.0 // Floor value
                }
            })
            .collect()
    }
    
    /// Compute spectrum from real-valued modulated carrier signal (shows 12 kHz peak)
    /// Returns (magnitude_vector, freq_start_hz, freq_end_hz)
    fn compute_carrier_spectrum(carrier_samples: &[f32], sample_rate: usize, carrier_freq: f64) -> (Vec<f32>, f32, f32) {
        if carrier_samples.is_empty() || sample_rate == 0 {
            return (Vec::new(), 0.0, 0.0);
        }
        
        // Use very large FFT for high frequency resolution (needed for narrow bandwidth)
        // With 48 kHz sample rate, 16384-point FFT gives ~2.9 Hz resolution
        let fft_size = 16384.min(carrier_samples.len().next_power_of_two().max(8192));
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        
        // Convert real samples to complex (imaginary = 0)
        let mut buffer: Vec<Complex32> = carrier_samples.iter()
            .take(fft_size)
            .map(|&s| Complex32::new(s, 0.0))
            .collect();
        
        // Pad if needed
        while buffer.len() < fft_size {
            buffer.push(Complex32::new(0.0, 0.0));
        }
        
        // Apply Hann window to reduce spectral leakage
        for i in 0..buffer.len() {
            let window_value = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 
                / (buffer.len() as f32 - 1.0)).cos());
            buffer[i] = buffer[i] * window_value;
        }
        
        fft.process(&mut buffer);
        
        // Hann window power scaling factor
        let window_power: f32 = 0.375;
        let scale = 1.0 / ((fft_size as f32) * window_power.sqrt());
        
        // Calculate frequency resolution
        let freq_resolution = sample_rate as f32 / fft_size as f32;
        let display_span = 400.0; // Show ±200 Hz around carrier (plenty for 32 Hz BW)
        
        // Find bin indices for carrier ± span
        let carrier_bin = (carrier_freq as f32 / freq_resolution) as usize;
        let span_bins = (display_span / freq_resolution) as usize;
        let start_bin = carrier_bin.saturating_sub(span_bins / 2);
        let end_bin = (carrier_bin + span_bins / 2).min(fft_size / 2);
        
        // Calculate actual frequency range being returned
        let freq_start = start_bin as f32 * freq_resolution;
        let freq_end = end_bin as f32 * freq_resolution;
        
        // Return only the narrow frequency span around the carrier
        let magnitude: Vec<f32> = buffer.iter()
            .skip(start_bin)
            .take(end_bin - start_bin)
            .map(|c| {
                let power = c.norm_sqr() * scale * scale;
                if power > 1e-10 {
                    10.0 * power.log10()
                } else {
                    -100.0 // Floor value
                }
            })
            .collect();
            
        (magnitude, freq_start, freq_end)
    }
    
    fn compute_evm(&self, tx_symbols: &[Complex<f64>], rx_i: &[f64], rx_q: &[f64]) -> f32 {
        if rx_i.is_empty() || tx_symbols.is_empty() {
            return 0.0;
        }
        
        let count = rx_i.len().min(tx_symbols.len());
        let mut error_sum = 0.0;
        let mut ref_power = 0.0;
        
        for i in 0..count {
            let tx = &tx_symbols[i];
            let error = Complex::new(rx_i[i] - tx.re, rx_q[i] - tx.im);
            error_sum += error.norm_sqr();
            ref_power += tx.norm_sqr();
        }
        
        if ref_power > 0.0 {
            100.0 * (error_sum / ref_power).sqrt() as f32
        } else {
            0.0
        }
    }
    
    fn estimate_snr(&self, rx_i: &[f64], rx_q: &[f64]) -> f32 {
        if rx_i.is_empty() {
            return 0.0;
        }
        
        let mut signal_power = 0.0;
        let mut noise_power = 0.0;
        
        for i in 0..rx_i.len() {
            let magnitude = (rx_i[i] * rx_i[i] + rx_q[i] * rx_q[i]).sqrt();
            signal_power += magnitude;
            
            // Estimate noise from deviation from ideal radius (1.0 for QPSK)
            let deviation = (magnitude - 1.0).abs();
            noise_power += deviation * deviation;
        }
        
        signal_power /= rx_i.len() as f64;
        noise_power /= rx_i.len() as f64;
        
        if noise_power > 0.0 {
            10.0 * (signal_power.powi(2) / noise_power).log10() as f32
        } else {
            40.0 // Very high SNR
        }
    }
    
    fn compute_ber(&self, tx_bits: &[u8], rx_bits: &[u8]) -> f32 {
        if tx_bits.is_empty() || rx_bits.is_empty() {
            return 0.0;
        }
        
        let count = tx_bits.len().min(rx_bits.len());
        let mut errors = 0;
        
        for i in 0..count {
            if tx_bits[i] != rx_bits[i] {
                errors += 1;
            }
        }
        
        errors as f32 / count as f32
    }
    
    /// Convert I/Q samples to audio
    fn iq_to_audio(&self, iq: &[f64]) -> Vec<f32> {
        if self.config.sample_rate == 0 || iq.len() < 2 {
            return Vec::new();
        }
        
        let dt = 1.0 / self.config.sample_rate as f64;
        let carrier_freq = self.protocol.carrier_freq_hz;
        let mut t = 0.0_f64;
        let mut audio = Vec::with_capacity(iq.len() / 2);
        
        for chunk in iq.chunks_exact(2) {
            let i = chunk[0];
            let q = chunk[1];
            let angle = TAU * carrier_freq * t;
            let sample = i * angle.cos() - q * angle.sin();
            audio.push(sample as f32);
            t += dt;
        }
        
        self.normalize_audio(&mut audio);
        audio
    }
    
    fn normalize_audio(&self, samples: &mut [f32]) {
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
    
    /// Convert symbols to modulated carrier signal (static version for borrow checker)
    /// This generates the actual 12 kHz QPSK-modulated carrier per the spec
    /// 
    /// This IS the transmitted signal - no additional pulse shaping needed.
    /// The demod/decoder works on this signal after channel impairments are applied.
    fn symbols_to_carrier_signal(symbols: &[Complex<f64>], sample_rate: usize, symbol_rate: usize, carrier_freq: f64) -> Vec<f32> {
        if sample_rate == 0 || symbols.is_empty() {
            return Vec::new();
        }
        
        let samples_per_symbol = (sample_rate / symbol_rate).max(1);
        let dt = 1.0 / sample_rate as f64;
        let mut audio = Vec::with_capacity(symbols.len() * samples_per_symbol);
        
        // Generate QPSK-modulated carrier: s(t) = I(t)cos(2πf_c·t) - Q(t)sin(2πf_c·t)
        // where f_c = 12 kHz carrier frequency
        // This rectangular pulse QPSK has bandwidth ≈ 2 * symbol_rate = 32 Hz for 16 sym/s
        for (sym_idx, symbol) in symbols.iter().enumerate() {
            for sample_idx in 0..samples_per_symbol {
                let t = (sym_idx * samples_per_symbol + sample_idx) as f64 * dt;
                let angle = TAU * carrier_freq * t;
                // QPSK modulation onto carrier
                let sample = symbol.re * angle.cos() - symbol.im * angle.sin();
                audio.push(sample as f32);
            }
        }
        
        Self::normalize_audio_static(&mut audio);
        audio
    }
    
    /// Static version of normalize_audio for use in static methods
    fn normalize_audio_static(samples: &mut [f32]) {
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
    
    /// Convert symbols to modulated carrier signal (instance method for backward compatibility)
    /// This generates the actual 12 kHz QPSK-modulated carrier per the spec
    fn symbols_to_audio_incremental(&self, symbols: &[Complex<f64>]) -> Vec<f32> {
        Self::symbols_to_carrier_signal(symbols, self.config.sample_rate, self.protocol.qpsk_symbol_rate, self.protocol.carrier_freq_hz)
    }
    
    /// Get current configuration
    pub fn get_config(&self) -> StreamConfig {
        StreamConfig {
            simulation: self.config.clone(),
            protocol: self.protocol.clone(),
        }
    }
    
    /// Reconfigure the pipeline
    pub fn reconfigure(&mut self, sim: SimulationConfig, protocol: ProtocolConfig, ldpc: LDPCConfig) {
        // Recalculate noise parameters
        let signal_power = 1.0;
        let link_loss_linear = 10f64.powf(sim.link_loss_db / 10.0);
        let attenuated_signal_power = signal_power / link_loss_linear;
        let snr_linear = 10f64.powf(sim.snr_db / 10.0);
        let noise_variance = if snr_linear > 0.0 {
            attenuated_signal_power / snr_linear
        } else {
            0.0
        };
        let noise_std = (noise_variance / 2.0).sqrt();
        
        let rng = match sim.rng_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        
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
}

/// Configuration for streaming pipeline
#[derive(Clone, Debug)]
pub struct StreamConfig {
    pub simulation: SimulationConfig,
    pub protocol: ProtocolConfig,
}

/// Standalone EVM calculation for symbol pairs
fn compute_evm(tx_symbols: &[Complex<f64>], rx_symbols: &[Complex<f64>]) -> f32 {
    if rx_symbols.is_empty() || tx_symbols.is_empty() {
        return 0.0;
    }
    
    let count = rx_symbols.len().min(tx_symbols.len());
    let mut error_sum = 0.0;
    let mut ref_power = 0.0;
    
    for i in 0..count {
        let error = rx_symbols[i] - tx_symbols[i];
        error_sum += error.norm_sqr();
        ref_power += tx_symbols[i].norm_sqr();
    }
    
    if ref_power > 0.0 {
        100.0 * (error_sum / ref_power).sqrt() as f32
    } else {
        0.0
    }
}

/// Standalone SNR estimation from received symbols
fn estimate_snr(rx_symbols: &[Complex<f64>]) -> f32 {
    if rx_symbols.is_empty() {
        return 0.0;
    }
    
    let mut signal_power = 0.0;
    let mut noise_power = 0.0;
    
    for symbol in rx_symbols {
        let magnitude = symbol.norm();
        signal_power += magnitude;
        
        // Estimate noise from deviation from ideal radius (1.0 for QPSK)
        let deviation = (magnitude - 1.0).abs();
        noise_power += deviation * deviation;
    }
    
    signal_power /= rx_symbols.len() as f64;
    noise_power /= rx_symbols.len() as f64;
    
    if noise_power > 0.0 {
        10.0 * (signal_power.powi(2) / noise_power).log10() as f32
    } else {
        40.0 // Very high SNR
    }
}
