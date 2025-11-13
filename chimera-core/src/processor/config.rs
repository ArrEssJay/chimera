//! Configuration types for the Chimera processor

use crate::logging::LogConfig;

/// Configuration for the Chimera processor
#[derive(Clone, Debug)]
pub struct ProcessorConfig {
    pub sample_rate: usize,
    pub symbol_rate: usize,
    pub carrier_freq: f64,
    
    /// Logging configuration
    pub logging: LogConfig,
    
    /// Processing mode hints
    pub optimize_for_latency: bool,  // true = realtime, false = batch
    pub min_chunk_size: Option<usize>,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            symbol_rate: 16,
            carrier_freq: 12000.0,
            logging: LogConfig::default(),
            optimize_for_latency: true,
            min_chunk_size: None,
        }
    }
}
