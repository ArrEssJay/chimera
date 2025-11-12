//! Signal processing functions for modulation, demodulation, and spectrum analysis
//!
//! This module provides core DSP functions for:
//! - Converting symbols to modulated audio carriers (QPSK + FSK)
//! - Demodulating audio back to IQ symbols
//! - Computing frequency spectrums via FFT
//!
//! These functions are designed to be reusable, testable, and suitable for
//! real-time operation.

pub mod modulation;
pub mod demodulation;
pub mod spectrum;
pub mod filters;

// Re-export commonly used items
pub use modulation::{ModulationConfig, symbols_to_carrier_signal, normalize_audio};
pub use demodulation::{DemodulationConfig, audio_to_symbols};
pub use spectrum::{compute_baseband_spectrum};
pub use filters::apply_rrc_filter;
