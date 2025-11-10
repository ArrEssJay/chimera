//! Chimera error types with granular categories

use thiserror::Error;

/// Top-level error type for all Chimera operations
#[derive(Debug, Error)]
pub enum ChimeraError {
    #[error("Encoding error: {0}")]
    Encoding(#[from] EncodingError),
    
    #[error("Decoding error: {0}")]
    Decoding(#[from] DecodingError),
    
    #[error("LDPC error: {0}")]
    Ldpc(#[from] LdpcError),
    
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("DSP processing error: {0}")]
    Dsp(#[from] DspError),
}

/// Encoding-specific errors
#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Invalid block size: expected {expected}, got {actual}")]
    InvalidBlockSize { expected: usize, actual: usize },
    
    #[error("Message length {message_len} exceeds maximum {max_len}")]
    MessageTooLong { message_len: usize, max_len: usize },
    
    #[error("Generator matrix dimension mismatch: message={message_bits}, matrix_rows={matrix_rows}")]
    GeneratorDimensionMismatch {
        message_bits: usize,
        matrix_rows: usize,
    },
    
    #[error("Frame encoding failed: {reason}")]
    FrameEncodingFailed { reason: String },
    
    #[error("Invalid modulation parameters: {details}")]
    InvalidModulation { details: String },
}

/// Decoding-specific errors
#[derive(Debug, Error)]
pub enum DecodingError {
    #[error("Sync not found after {symbols_searched} symbols")]
    SyncNotFound { symbols_searched: usize },
    
    #[error("Invalid codeword: syndrome check failed")]
    InvalidCodeword,
    
    #[error("Demodulation failed: {reason}")]
    DemodulationFailed { reason: String },
    
    #[error("Frame checksum mismatch: expected {expected:#x}, got {actual:#x}")]
    ChecksumMismatch { expected: u32, actual: u32 },
    
    #[error("Insufficient symbols: need {required}, have {available}")]
    InsufficientSymbols { required: usize, available: usize },
    
    #[error("Belief propagation did not converge after {iterations} iterations")]
    ConvergenceFailed { iterations: usize },
}

/// LDPC-specific errors
#[derive(Debug, Error)]
pub enum LdpcError {
    #[error("Matrix construction failed: {reason}")]
    MatrixConstructionFailed { reason: String },
    
    #[error("Invalid code rate: k={k}, n={n}")]
    InvalidCodeRate { k: usize, n: usize },
    
    #[error("Parity check matrix has incompatible dimensions: {rows}×{cols}")]
    IncompatibleMatrixDimensions { rows: usize, cols: usize },
    
    #[error("Maximum iterations {max_iter} exceeded without convergence")]
    MaxIterationsExceeded { max_iter: usize },
    
    #[error("Singular matrix: cannot perform Gaussian elimination")]
    SingularMatrix,
}

/// Configuration errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid sample rate: {rate} Hz (must be > 0)")]
    InvalidSampleRate { rate: f64 },
    
    #[error("Nyquist violation: carrier {carrier_hz} Hz requires sample rate > {min_required_hz} Hz, got {actual_hz} Hz")]
    NyquistViolation {
        carrier_hz: f64,
        min_required_hz: f64,
        actual_hz: f64,
    },
    
    #[error("Invalid symbol rate: {rate} (must be > 0)")]
    InvalidSymbolRate { rate: usize },
    
    #[error("Invalid SNR: {snr_db} dB (must be finite)")]
    InvalidSnr { snr_db: f64 },
    
    #[error("Invalid frame layout: {reason}")]
    InvalidFrameLayout { reason: String },
    
    #[error("FSK frequencies out of range: f0={f0} Hz, f1={f1} Hz")]
    InvalidFskFrequencies { f0: f64, f1: f64 },
}

/// DSP processing errors
#[derive(Debug, Error)]
pub enum DspError {
    #[error("FFT size {size} is not a power of 2")]
    InvalidFftSize { size: usize },
    
    #[error("Filter design failed: {reason}")]
    FilterDesignFailed { reason: String },
    
    #[error("Phase-locked loop failed to lock")]
    PllLockFailed,
    
    #[error("Timing recovery failed: excessive timing error")]
    TimingRecoveryFailed,
    
    #[error("Buffer underrun: requested {requested}, available {available}")]
    BufferUnderrun { requested: usize, available: usize },
    
    #[error("Buffer overflow: capacity {capacity}, attempted write of {size}")]
    BufferOverflow { capacity: usize, size: usize },
}

/// Result type alias for Chimera operations
pub type Result<T> = std::result::Result<T, ChimeraError>;
