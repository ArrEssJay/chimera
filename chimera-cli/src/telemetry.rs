//! Telemetry sampling and statistical aggregation.

use crate::logging::{StatSummary, TelemetryEvent};
use chrono::Utc;
use std::time::Instant;

/// Accumulates telemetry samples and computes statistics
pub struct TelemetryAggregator {
    // Accumulation vectors
    pre_fec_ber_samples: Vec<f64>,
    post_fec_ber_samples: Vec<f64>,
    fsk_frequency_samples: Vec<f64>,
    
    // Current state
    last_sample_time: Instant,
    total_symbols: usize,
    total_frames: usize,
    current_sync_locked: bool,
    
    // Config
    sample_interval_secs: f64,
}

impl TelemetryAggregator {
    pub fn new(sample_interval_secs: f64) -> Self {
        Self {
            pre_fec_ber_samples: Vec::new(),
            post_fec_ber_samples: Vec::new(),
            fsk_frequency_samples: Vec::new(),
            last_sample_time: Instant::now(),
            total_symbols: 0,
            total_frames: 0,
            current_sync_locked: false,
            sample_interval_secs,
        }
    }
    
    /// Update telemetry state with new decoder output
    pub fn update(&mut self, 
                  pre_fec_ber: f64, 
                  post_fec_ber: f64, 
                  fsk_frequency: f64,
                  symbols_decoded: usize,
                  frames_decoded: usize,
                  sync_locked: bool) {
        self.total_symbols += symbols_decoded;
        self.total_frames = frames_decoded;
        self.current_sync_locked = sync_locked;
        
        // Record samples for statistics
        self.pre_fec_ber_samples.push(pre_fec_ber);
        self.post_fec_ber_samples.push(post_fec_ber);
        self.fsk_frequency_samples.push(fsk_frequency);
    }
    
    /// Check if it's time to emit a telemetry sample
    pub fn should_sample(&self) -> bool {
        self.last_sample_time.elapsed().as_secs_f64() >= self.sample_interval_secs
    }
    
    /// Generate telemetry event and reset sampling timer
    pub fn sample(&mut self) -> Option<TelemetryEvent> {
        if !self.should_sample() {
            return None;
        }
        
        // Calculate current averages from recent samples
        let pre_fec_ber = self.mean(&self.pre_fec_ber_samples).unwrap_or(0.0);
        let post_fec_ber = self.mean(&self.post_fec_ber_samples).unwrap_or(0.0);
        let fsk_frequency = self.mean(&self.fsk_frequency_samples).unwrap_or(12000.0);
        
        let event = TelemetryEvent {
            timestamp: Utc::now(),
            pre_fec_ber,
            post_fec_ber,
            fsk_frequency_hz: fsk_frequency,
            symbols_decoded: self.total_symbols,
            frames_decoded: self.total_frames,
            sync_locked: self.current_sync_locked,
        };
        
        self.last_sample_time = Instant::now();
        
        Some(event)
    }
    
    /// Compute statistical summary for all accumulated samples
    pub fn compute_statistics(&self) -> (StatSummary, StatSummary, StatSummary) {
        (
            self.compute_stat_summary(&self.pre_fec_ber_samples),
            self.compute_stat_summary(&self.post_fec_ber_samples),
            self.compute_stat_summary(&self.fsk_frequency_samples),
        )
    }
    
    fn compute_stat_summary(&self, samples: &[f64]) -> StatSummary {
        if samples.is_empty() {
            return StatSummary {
                mean: 0.0,
                stddev: 0.0,
                min: 0.0,
                max: 0.0,
                samples: 0,
            };
        }
        
        let mean = self.mean(samples).unwrap_or(0.0);
        let variance = samples.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / samples.len() as f64;
        let stddev = variance.sqrt();
        let min = samples.iter().copied().fold(f64::INFINITY, f64::min);
        let max = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        
        StatSummary {
            mean,
            stddev,
            min,
            max,
            samples: samples.len(),
        }
    }
    
    fn mean(&self, samples: &[f64]) -> Option<f64> {
        if samples.is_empty() {
            return None;
        }
        Some(samples.iter().sum::<f64>() / samples.len() as f64)
    }
    
    pub fn total_frames(&self) -> usize {
        self.total_frames
    }
    
    pub fn total_symbols(&self) -> usize {
        self.total_symbols
    }
}
