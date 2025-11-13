//! Chimera core library
//!
//! This crate provides a Rust-native implementation of the Raman Whisper
//! modulation and decoding pipeline. The modules are organized to mirror
//! the former Python reference implementation, but expose a fully typed
//! and testable API.

pub mod audio_generator;
pub mod config;
pub mod decoder;
pub mod diagnostics;
pub mod encoder;
pub mod errors;
pub mod external_audio;
pub mod ldpc;
pub mod logging;
pub mod pipeline;
pub mod processor;
pub mod protocol;
pub mod signal_processing;
pub mod thz_carriers;
pub mod utils;

// Re-export the canonical processor interface
pub use processor::{ChimeraProcessor, ProcessorConfig, BatchResult};

use config::{LDPCConfig, InternalProtocolConfig, UserSimulationConfig};
use diagnostics::{DiagnosticsBundle, ModulationAudio, SimulationReport};
use ldpc::LDPCSuite;
