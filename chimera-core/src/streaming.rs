//! Streaming DSP pipeline for real-time audio processing
//!
//! This module provides a streaming version of the batch-mode simulation,
//! allowing chunk-by-chunk processing suitable for real-time audio applications.

use crate::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use crate::encoder::generate_modulated_signal;
use crate::decoder::demodulate_and_decode;
use crate::ldpc::{LDPCSuite};
use num_complex::Complex;
use rustfft::{FftPlanner, num_complex::Complex32};
use std::collections::VecDeque;
use std::f64::consts::TAU;

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
}

/// Streaming DSP pipeline
pub struct StreamingPipeline {
    config: SimulationConfig,
    protocol: ProtocolConfig,
    ldpc_suite: LDPCSuite,
    
    // Buffers
    input_buffer: VecDeque<u8>,
    frame_count: usize,
    total_frames: usize,
    message_position: usize,
    total_symbols: usize,
    total_errors: usize,
    ber_accumulator: f32,
}

impl StreamingPipeline {
    /// Create a new streaming pipeline with the given configuration
    pub fn new(
        sim: SimulationConfig,
        protocol: ProtocolConfig,
        ldpc: LDPCConfig,
    ) -> Self {
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
        
        // Calculate total frames needed for the message
        let message_bytes = sim.plaintext_source.as_bytes().len();
        // Each QPSK symbol carries 2 bits, so data_payload_symbols * 2 / 8 = bytes per frame
        let data_bytes_per_frame = (protocol.frame_layout.data_payload_symbols * 2) / 8;
        let total_frames = if data_bytes_per_frame > 0 {
            (message_bytes + data_bytes_per_frame - 1) / data_bytes_per_frame
        } else {
            1
        };
        
        Self {
            config: sim,
            protocol,
            ldpc_suite,
            input_buffer: VecDeque::with_capacity(8192),
            frame_count: 0,
            total_frames: total_frames.max(1),
            message_position: 0,
            total_symbols: 0,
            total_errors: 0,
            ber_accumulator: 0.0,
        }
    }
    
    /// Process a chunk of input bytes through the pipeline
    pub fn process_chunk(&mut self, _input: &[u8]) -> StreamingOutput {
        let mut output = StreamingOutput::default();
        
        // Use the configured plaintext_source as the message to transmit
        // This is a transmitter simulation, not a receiver processing actual input
        if !self.config.plaintext_source.is_empty() {
            
            // Generate modulated signal
            let encoding = generate_modulated_signal(
                &self.config,
                &self.protocol,
                &self.ldpc_suite.matrices,
            );
            
            // Demodulate and decode
            let demodulation = demodulate_and_decode(
                &encoding,
                &self.ldpc_suite.matrices,
                &self.config,
                &self.protocol,
            );
            
            self.frame_count += 1;
            
            // Normalize constellation points to [-1, 1] range
            let normalize_constellation = |symbols: &[Complex<f64>]| -> (Vec<f32>, Vec<f32>) {
                if symbols.is_empty() {
                    return (Vec::new(), Vec::new());
                }
                
                // Find max magnitude for normalization
                let max_mag = symbols.iter()
                    .map(|s| s.norm())
                    .fold(0.0_f64, f64::max)
                    .max(1.0);
                
                let scale = 1.0 / max_mag;
                
                let i_vals: Vec<f32> = symbols.iter().map(|c| (c.re * scale) as f32).collect();
                let q_vals: Vec<f32> = symbols.iter().map(|c| (c.im * scale) as f32).collect();
                
                (i_vals, q_vals)
            };
            
            let (tx_i_norm, tx_q_norm) = normalize_constellation(&encoding.qpsk_symbols);
            
            // Pre-channel diagnostics (transmitter side)
            output.pre_channel = PreChannelDiagnostics {
                frame_count: self.frame_count,
                total_frames: self.total_frames,
                symbol_count: encoding.qpsk_symbols.len(),
                tx_constellation_i: tx_i_norm,
                tx_constellation_q: tx_q_norm,
                tx_spectrum_magnitude: Self::compute_spectrum_with_scaling(&encoding.qpsk_symbols),
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
            
            // Post-channel diagnostics (receiver side)
            let rx_symbols_i = &demodulation.diagnostics.received_symbols_i;
            let rx_symbols_q = &demodulation.diagnostics.received_symbols_q;
            
            // Normalize RX constellation
            let normalize_rx = |i_vals: &[f64], q_vals: &[f64]| -> (Vec<f32>, Vec<f32>) {
                if i_vals.is_empty() {
                    return (Vec::new(), Vec::new());
                }
                
                let max_val = i_vals.iter().chain(q_vals.iter())
                    .map(|v| v.abs())
                    .fold(0.0_f64, f64::max)
                    .max(1.0);
                
                let scale = 1.0 / max_val;
                
                (
                    i_vals.iter().map(|&v| (v * scale) as f32).collect(),
                    q_vals.iter().map(|&v| (v * scale) as f32).collect(),
                )
            };
            
            let (rx_i_norm, rx_q_norm) = normalize_rx(rx_symbols_i, rx_symbols_q);
            
            let evm_percent = self.compute_evm(&encoding.qpsk_symbols, rx_symbols_i, rx_symbols_q);
            let snr_estimate_db = self.estimate_snr(rx_symbols_i, rx_symbols_q);
            let ber_instantaneous = self.compute_ber(&encoding.qpsk_bitstream, &demodulation.decoded_bitstream);
            
            output.post_channel = PostChannelDiagnostics {
                rx_constellation_i: rx_i_norm,
                rx_constellation_q: rx_q_norm,
                rx_spectrum_magnitude: Self::compute_spectrum_from_real_with_scaling(rx_symbols_i, rx_symbols_q),
                timing_error: demodulation.diagnostics.timing_error.iter().map(|&v| v as f32).collect(),
                frequency_offset_hz: if !demodulation.diagnostics.nco_freq_offset.is_empty() {
                    demodulation.diagnostics.nco_freq_offset.last().copied().unwrap_or(0.0) as f32
                } else { 0.0 },
                phase_offset_rad: 0.0,
                evm_percent,
                snr_estimate_db,
                ber_instantaneous,
                ber_average: self.ber_accumulator,
                sync_status: !rx_symbols_i.is_empty(),
                lock_status: if rx_symbols_i.is_empty() { "UNLOCKED".to_string() } 
                            else if self.frame_count < 3 { "ACQUIRING".to_string() }
                            else { "LOCKED".to_string() },
            };
            
            // Update running BER average
            if ber_instantaneous > 0.0 {
                self.ber_accumulator = (self.ber_accumulator * (self.frame_count - 1) as f32 
                    + ber_instantaneous) / self.frame_count as f32;
            }
            
            // Audio output - use the noisy_signal if available, otherwise clean_signal
            let audio_source = if !encoding.noisy_signal.is_empty() {
                encoding.noisy_signal.as_slice().unwrap_or(&[])
            } else if let Some(clean) = encoding.clean_signal.as_slice() {
                clean
            } else {
                &[]
            };
            
            if !audio_source.is_empty() {
                output.audio_samples = self.iq_to_audio(audio_source);
            }
            
            // Decoded data
            output.decoded_data = demodulation.decoded_bitstream.clone();
            output.decoded_text = String::from_utf8_lossy(&demodulation.decoded_bitstream).to_string();
            
            // Performance metrics
            output.frames_processed = self.frame_count;
            output.symbols_decoded = self.total_symbols + rx_symbols_i.len();
            output.fec_corrections = 0; // Would need to track this from LDPC decoder
            
            self.total_symbols += rx_symbols_i.len();
        }
        
        output
    }
    
    fn compute_spectrum_with_scaling(symbols: &[Complex<f64>]) -> Vec<f32> {
        let fft_size = 512.min(symbols.len().next_power_of_two());
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
        
        fft.process(&mut buffer);
        
        // Apply window function and scaling
        let window_scale = 1.0 / (fft_size as f32).sqrt();
        
        // Convert to magnitude in dB with proper scaling
        buffer.iter()
            .map(|c| {
                let mag = c.norm() * window_scale;
                if mag > 1e-10 {
                    20.0 * mag.log10()
                } else {
                    -100.0 // Floor value
                }
            })
            .collect()
    }
    
    fn compute_spectrum_from_real_with_scaling(i_samples: &[f64], q_samples: &[f64]) -> Vec<f32> {
        let fft_size = 512.min(i_samples.len().next_power_of_two());
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        
        let mut buffer: Vec<Complex32> = i_samples.iter()
            .zip(q_samples.iter())
            .take(fft_size)
            .map(|(&i, &q)| Complex32::new(i as f32, q as f32))
            .collect();
        
        while buffer.len() < fft_size {
            buffer.push(Complex32::new(0.0, 0.0));
        }
        
        fft.process(&mut buffer);
        
        let window_scale = 1.0 / (fft_size as f32).sqrt();
        
        buffer.iter()
            .map(|c| {
                let mag = c.norm() * window_scale;
                if mag > 1e-10 {
                    20.0 * mag.log10()
                } else {
                    -100.0
                }
            })
            .collect()
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
    
    /// Get current configuration
    pub fn get_config(&self) -> StreamConfig {
        StreamConfig {
            simulation: self.config.clone(),
            protocol: self.protocol.clone(),
        }
    }
    
    /// Reconfigure the pipeline
    pub fn reconfigure(&mut self, sim: SimulationConfig, protocol: ProtocolConfig, ldpc: LDPCConfig) {
        // Recalculate total frames
        let message_bytes = sim.plaintext_source.as_bytes().len();
        let data_bytes_per_frame = (protocol.frame_layout.data_payload_symbols * 2) / 8;
        let total_frames = if data_bytes_per_frame > 0 {
            (message_bytes + data_bytes_per_frame - 1) / data_bytes_per_frame
        } else {
            1
        };
        
        self.config = sim;
        self.protocol = protocol.clone();
        self.ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
        self.input_buffer.clear();
        self.frame_count = 0;
        self.total_frames = total_frames.max(1);
        self.message_position = 0;
        self.total_symbols = 0;
        self.total_errors = 0;
        self.ber_accumulator = 0.0;
    }
}

/// Configuration for streaming pipeline
#[derive(Clone, Debug)]
pub struct StreamConfig {
    pub simulation: SimulationConfig,
    pub protocol: ProtocolConfig,
}
