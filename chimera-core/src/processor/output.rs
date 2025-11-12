//! Output types for the Chimera processor

use num_complex::Complex64;

/// Output from processor operations
#[derive(Clone, Debug)]
pub struct ProcessorOutput {
    /// Decoded bytes (may be empty if not enough data yet)
    pub decoded_bytes: Vec<u8>,
    
    /// Whether output is ready/available
    pub ready: bool,
    
    /// Transmitted symbols (for diagnostics)
    pub tx_symbols: Vec<Complex64>,
    
    /// Received symbols after demodulation (for diagnostics)
    pub rx_symbols: Vec<Complex64>,
    
    /// Audio signal (modulated carrier)
    pub audio: Vec<f32>,
    
    /// Estimated SNR from demodulator
    pub snr_db: f32,
    
    /// Success flag
    pub success: bool,
    
    /// Error message if any
    pub error: Option<String>,
}

impl ProcessorOutput {
    pub fn empty() -> Self {
        Self {
            decoded_bytes: Vec::new(),
            ready: false,
            tx_symbols: Vec::new(),
            rx_symbols: Vec::new(),
            audio: Vec::new(),
            snr_db: 0.0,
            success: false,
            error: None,
        }
    }
    
    pub fn has_data(&self) -> bool {
        !self.decoded_bytes.is_empty()
    }
}

/// Batch processing output with additional metadata
#[derive(Clone, Debug)]
pub struct BatchOutput {
    pub input_message: String,
    pub output_message: String,
    pub ber: f32,
    pub audio: Vec<f32>,
    pub tx_symbols: Vec<Complex64>,
    pub rx_symbols: Vec<Complex64>,
    pub snr_db: f32,
    pub success: bool,
}
