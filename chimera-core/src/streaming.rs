//! Streaming DSP pipeline for real-time audio processing
//!
//! This module provides a streaming version of the batch-mode simulation,
//! allowing chunk-by-chunk processing suitable for real-time audio applications.

use crate::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use crate::encoder::generate_modulated_signal;
use crate::decoder::demodulate_and_decode;
use crate::ldpc::{LDPCSuite};
use std::collections::VecDeque;
use std::f64::consts::TAU;

/// Output from a single processing chunk
#[derive(Clone, Debug, Default)]
pub struct StreamingOutput {
    /// Audio samples (modulated carrier)
    pub audio_samples: Vec<f32>,
    /// Baseband I component
    pub baseband_i: Vec<f32>,
    /// Baseband Q component
    pub baseband_q: Vec<f32>,
    /// Constellation points (I component) after demodulation
    pub constellation_i: Vec<f32>,
    /// Constellation points (Q component) after demodulation
    pub constellation_q: Vec<f32>,
    /// Decoded data bytes
    pub decoded_data: Vec<u8>,
    /// BER samples
    pub ber_samples: Vec<f32>,
    /// Timing error
    pub timing_error: f64,
}

/// Streaming DSP pipeline
pub struct StreamingPipeline {
    config: SimulationConfig,
    protocol: ProtocolConfig,
    ldpc_suite: LDPCSuite,
    
    // Buffers
    input_buffer: VecDeque<u8>,
    frame_counter: u32,
}

impl StreamingPipeline {
    /// Create a new streaming pipeline with the given configuration
    pub fn new(
        sim: SimulationConfig,
        protocol: ProtocolConfig,
        ldpc: LDPCConfig,
    ) -> Self {
        let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
        
        Self {
            config: sim,
            protocol,
            ldpc_suite,
            input_buffer: VecDeque::with_capacity(8192),
            frame_counter: 0,
        }
    }
    
    /// Process a chunk of input bytes through the pipeline
    /// 
    /// This reuses the existing encoder/decoder but operates on smaller chunks
    pub fn process_chunk(&mut self, input: &[u8]) -> StreamingOutput {
        // Add input to buffer
        self.input_buffer.extend(input);
        
        // For now, use the existing batch processing
        // In production, this would process frame-by-frame
        let mut output = StreamingOutput::default();
        
        // Process if we have at least some data
        if self.input_buffer.len() >= 16 {
            // Take available data (will be used for real data encoding in production)
            let _data: Vec<u8> = self.input_buffer.drain(..).collect();
            
            // Use existing batch processing as a starting point
            let encoding = generate_modulated_signal(
                &self.config,
                &self.protocol,
                &self.ldpc_suite.matrices,
            );
            
            let demodulation = demodulate_and_decode(
                &encoding,
                &self.ldpc_suite.matrices,
                &self.config,
                &self.protocol,
            );
            
            // Extract outputs
            output.baseband_i = encoding.qpsk_symbols.iter().map(|c| c.re as f32).collect();
            output.baseband_q = encoding.qpsk_symbols.iter().map(|c| c.im as f32).collect();
            
            // Generate audio from baseband
            if let Some(clean_signal) = encoding.clean_signal.as_slice() {
                output.audio_samples = self.iq_to_audio(clean_signal);
            }
            
            // Constellation points from demodulation
            output.constellation_i = demodulation.diagnostics.received_symbols_i.iter()
                .map(|&v| v as f32).collect();
            output.constellation_q = demodulation.diagnostics.received_symbols_q.iter()
                .map(|&v| v as f32).collect();
            
            // Decoded data
            output.decoded_data = demodulation.decoded_bitstream.clone();
            
            // BER
            output.ber_samples.push(demodulation.report.post_fec_ber as f32);
            
            self.frame_counter += 1;
        }
        
        output
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
        self.config = sim;
        self.protocol = protocol.clone();
        self.ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc);
        self.input_buffer.clear();
        self.frame_counter = 0;
    }
}

/// Configuration for streaming pipeline
#[derive(Clone, Debug)]
pub struct StreamConfig {
    pub simulation: SimulationConfig,
    pub protocol: ProtocolConfig,
}

/// FFT processor for spectrum analysis
pub struct FFTProcessor {
    size: usize,
    window: Vec<f32>,
}

impl FFTProcessor {
    pub fn new(size: usize) -> Self {
        // Hanning window
        let window: Vec<f32> = (0..size)
            .map(|i| {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (size - 1) as f32).cos())
            })
            .collect();
        
        Self { size, window }
    }
    
    pub fn process(&self, i_samples: &[f32], q_samples: &[f32]) -> FFTResult {
        use rustfft::num_complex::Complex32;
        use rustfft::FftPlanner;
        
        let len = i_samples.len().min(q_samples.len()).min(self.size);
        
        // Create complex samples and apply window
        let mut buffer: Vec<Complex32> = i_samples[..len]
            .iter()
            .zip(&q_samples[..len])
            .zip(&self.window[..len])
            .map(|((&i, &q), &w)| Complex32::new(i * w, q * w))
            .collect();
        
        // Pad to FFT size
        buffer.resize(self.size, Complex32::new(0.0, 0.0));
        
        // Perform FFT
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.size);
        fft.process(&mut buffer);
        
        // Convert to magnitude and phase
        let magnitude: Vec<f32> = buffer.iter()
            .map(|c: &rustfft::num_complex::Complex32| 20.0 * (c.norm() + 1e-10).log10())
            .collect();
        
        let phase: Vec<f32> = buffer.iter()
            .map(|c: &rustfft::num_complex::Complex32| c.arg())
            .collect();
        
        FFTResult { magnitude, phase }
    }
}

/// Result from FFT processing
#[derive(Clone, Debug)]
pub struct FFTResult {
    pub magnitude: Vec<f32>,
    pub phase: Vec<f32>,
}
