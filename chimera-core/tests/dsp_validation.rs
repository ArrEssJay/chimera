//! DSP Validation Test Suite
//!
//! Comprehensive testing of the signal processing pipeline.
//! Tests are organized into submodules by category.
//!
//! This file serves as the test entry point. The actual test
//! modules are in the dsp_validation/ subdirectory.

#[path = "dsp_validation/signal_analysis.rs"]
mod signal_analysis;

#[path = "dsp_validation/fixtures.rs"]
mod fixtures;

#[path = "dsp_validation/carrier.rs"]
mod carrier;

#[path = "dsp_validation/modulation.rs"]
mod modulation;

#[path = "dsp_validation/channel.rs"]
mod channel;

#[path = "dsp_validation/demodulation.rs"]
mod demodulation;

#[path = "dsp_validation/thz_processing.rs"]
mod thz_processing;

#[path = "dsp_validation/realtime.rs"]
mod realtime;

#[path = "dsp_validation/frame_sync.rs"]
mod frame_sync;

#[path = "dsp_validation/ldpc_fec.rs"]
mod ldpc_fec;

#[path = "dsp_validation/edge_cases.rs"]
mod edge_cases;

#[path = "dsp_validation/integration.rs"]
mod integration;

#[path = "dsp_validation/diagnostics.rs"]
mod diagnostics;
