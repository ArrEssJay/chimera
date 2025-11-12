//! Configuration types for the Chimera processor

/// Configuration for the Chimera processor
#[derive(Clone, Debug)]
pub struct ProcessorConfig {
    pub sample_rate: usize,
    pub symbol_rate: usize,
    pub carrier_freq: f64,
    
    /// Channel configuration
    pub channel: ChannelConfig,
    
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
            channel: ChannelConfig::default(),
            optimize_for_latency: true,
            min_chunk_size: None,
        }
    }
}

/// Channel effects configuration
#[derive(Clone, Debug)]
pub struct ChannelConfig {
    pub snr_db: f32,
    pub enable_noise: bool,
    pub enable_fading: bool,
    pub attenuation: f64,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            snr_db: 100.0,  // Clean channel by default for testing
            enable_noise: false,
            enable_fading: false,
            attenuation: 1.0,
        }
    }
}

impl ChannelConfig {
    pub fn disable_noise(&mut self) {
        self.enable_noise = false;
        self.snr_db = 100.0;
    }
}
