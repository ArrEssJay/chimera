//! WASM bindings for streaming DSP pipeline
//!
//! Exposes the streaming pipeline to JavaScript for use in web applications.

use wasm_bindgen::prelude::*;
use chimera_core::streaming::StreamingPipeline;
use chimera_core::config::{SimulationConfig, ProtocolConfig, LDPCConfig};
use js_sys::Float32Array;
use serde::{Deserialize, Serialize};

/// WASM wrapper for streaming DSP engine
#[wasm_bindgen]
pub struct WASMStreamingDSP {
    pipeline: StreamingPipeline,
}

#[wasm_bindgen]
impl WASMStreamingDSP {
    /// Create a new streaming DSP engine with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WASMStreamingDSP, JsValue> {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        
        // Use default configurations
        let sim = SimulationConfig::default();
        let protocol = ProtocolConfig::default();
        let ldpc = LDPCConfig::default();
        
        Ok(WASMStreamingDSP {
            pipeline: StreamingPipeline::new(sim, protocol, ldpc),
        })
    }
    
    /// Process audio input and return multiple output streams
    #[wasm_bindgen]
    pub fn process_audio(&mut self, input_audio: &Float32Array) -> Result<WASMStreamOutput, JsValue> {
        // Convert Float32Array to Vec
        let mut input_vec: Vec<f32> = vec![0.0; input_audio.length() as usize];
        input_audio.copy_to(&mut input_vec[..]);
        
        // Convert audio samples to bytes for processing
        // In a real implementation, this would extract data from the audio
        let input_bytes: Vec<u8> = input_vec.iter()
            .map(|&s| ((s * 127.0).clamp(-128.0, 127.0) as i8) as u8)
            .collect();
        
        // Process through pipeline
        let output = self.pipeline.process_chunk(&input_bytes);
        
        self.create_output(output)
    }
    
    /// Process raw bytes and return output
    #[wasm_bindgen]
    pub fn process(&mut self, input_data: &[u8]) -> Result<WASMStreamOutput, JsValue> {
        let output = self.pipeline.process_chunk(input_data);
        self.create_output(output)
    }
    
    /// Helper to create WASM output from streaming output
    fn create_output(&mut self, output: chimera_core::streaming::StreamingOutput) -> Result<WASMStreamOutput, JsValue> {
        
        // Package for JavaScript
        Ok(WASMStreamOutput {
            audio: Float32Array::from(&output.audio_samples[..]),
            
            // Pre-channel (TX)
            tx_constellation_i: Float32Array::from(&output.pre_channel.tx_constellation_i[..]),
            tx_constellation_q: Float32Array::from(&output.pre_channel.tx_constellation_q[..]),
            tx_spectrum_magnitude: Float32Array::from(&output.pre_channel.tx_spectrum_magnitude[..]),
            frame_count: output.pre_channel.frame_count as u32,
            symbol_count: output.pre_channel.symbol_count as u32,
            carrier_freq_hz: output.pre_channel.carrier_freq_hz,
            symbol_rate_hz: output.pre_channel.symbol_rate_hz,
            modulation_type: output.pre_channel.modulation_type,
            fec_rate: output.pre_channel.fec_rate,
            
            // Post-channel (RX)
            rx_constellation_i: Float32Array::from(&output.post_channel.rx_constellation_i[..]),
            rx_constellation_q: Float32Array::from(&output.post_channel.rx_constellation_q[..]),
            rx_spectrum_magnitude: Float32Array::from(&output.post_channel.rx_spectrum_magnitude[..]),
            timing_error: Float32Array::from(&output.post_channel.timing_error[..]),
            frequency_offset_hz: output.post_channel.frequency_offset_hz,
            phase_offset_rad: output.post_channel.phase_offset_rad,
            evm_percent: output.post_channel.evm_percent,
            snr_estimate_db: output.post_channel.snr_estimate_db,
            ber_instantaneous: output.post_channel.ber_instantaneous,
            ber_average: output.post_channel.ber_average,
            sync_status: output.post_channel.sync_status,
            lock_status: output.post_channel.lock_status,
            
            // Decoded
            decoded_text: output.decoded_text,
            
            // Performance
            frames_processed: output.frames_processed as u32,
            symbols_decoded: output.symbols_decoded as u32,
            fec_corrections: output.fec_corrections as u32,
        })
    }
    
    /// Configure the pipeline with JSON configuration
    #[wasm_bindgen]
    pub fn configure(&mut self, config_json: &str) -> Result<(), JsValue> {
        let config: StreamConfigWASM = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse config: {}", e)))?;
        
        self.pipeline.reconfigure(
            config.simulation,
            config.protocol,
            config.ldpc,
        );
        
        Ok(())
    }
    
    /// Get current configuration as JSON
    #[wasm_bindgen]
    pub fn get_config(&self) -> String {
        let config = self.pipeline.get_config();
        serde_json::to_string(&StreamConfigWASM {
            simulation: config.simulation,
            protocol: config.protocol,
            ldpc: LDPCConfig::default(), // TODO: store LDPC config in pipeline
        }).unwrap_or_default()
    }
}

/// Output structure exposed to JavaScript
#[wasm_bindgen]
pub struct WASMStreamOutput {
    // Audio
    audio: Float32Array,
    
    // Pre-channel (TX)
    tx_constellation_i: Float32Array,
    tx_constellation_q: Float32Array,
    tx_spectrum_magnitude: Float32Array,
    frame_count: u32,
    symbol_count: u32,
    carrier_freq_hz: f64,
    symbol_rate_hz: u32,
    modulation_type: String,
    fec_rate: String,
    
    // Post-channel (RX)
    rx_constellation_i: Float32Array,
    rx_constellation_q: Float32Array,
    rx_spectrum_magnitude: Float32Array,
    timing_error: Float32Array,
    frequency_offset_hz: f32,
    phase_offset_rad: f32,
    evm_percent: f32,
    snr_estimate_db: f32,
    ber_instantaneous: f32,
    ber_average: f32,
    sync_status: bool,
    lock_status: String,
    
    // Decoded
    decoded_text: String,
    
    // Performance
    frames_processed: u32,
    symbols_decoded: u32,
    fec_corrections: u32,
}

#[wasm_bindgen]
impl WASMStreamOutput {
    #[wasm_bindgen(getter)]
    pub fn audio(&self) -> Float32Array {
        self.audio.clone()
    }
    
    // Pre-channel getters
    #[wasm_bindgen(getter)]
    pub fn tx_constellation_i(&self) -> Float32Array {
        self.tx_constellation_i.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn tx_constellation_q(&self) -> Float32Array {
        self.tx_constellation_q.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn tx_spectrum_magnitude(&self) -> Float32Array {
        self.tx_spectrum_magnitude.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
    
    #[wasm_bindgen(getter)]
    pub fn symbol_count(&self) -> u32 {
        self.symbol_count
    }
    
    #[wasm_bindgen(getter)]
    pub fn carrier_freq_hz(&self) -> f64 {
        self.carrier_freq_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn symbol_rate_hz(&self) -> u32 {
        self.symbol_rate_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn modulation_type(&self) -> String {
        self.modulation_type.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn fec_rate(&self) -> String {
        self.fec_rate.clone()
    }
    
    // Post-channel getters
    #[wasm_bindgen(getter)]
    pub fn rx_constellation_i(&self) -> Float32Array {
        self.rx_constellation_i.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn rx_constellation_q(&self) -> Float32Array {
        self.rx_constellation_q.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn rx_spectrum_magnitude(&self) -> Float32Array {
        self.rx_spectrum_magnitude.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn timing_error(&self) -> Float32Array {
        self.timing_error.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn frequency_offset_hz(&self) -> f32 {
        self.frequency_offset_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn phase_offset_rad(&self) -> f32 {
        self.phase_offset_rad
    }
    
    #[wasm_bindgen(getter)]
    pub fn evm_percent(&self) -> f32 {
        self.evm_percent
    }
    
    #[wasm_bindgen(getter)]
    pub fn snr_estimate_db(&self) -> f32 {
        self.snr_estimate_db
    }
    
    #[wasm_bindgen(getter)]
    pub fn ber_instantaneous(&self) -> f32 {
        self.ber_instantaneous
    }
    
    #[wasm_bindgen(getter)]
    pub fn ber_average(&self) -> f32 {
        self.ber_average
    }
    
    #[wasm_bindgen(getter)]
    pub fn sync_status(&self) -> bool {
        self.sync_status
    }
    
    #[wasm_bindgen(getter)]
    pub fn lock_status(&self) -> String {
        self.lock_status.clone()
    }
    
    // Decoded getters
    #[wasm_bindgen(getter)]
    pub fn decoded_text(&self) -> String {
        self.decoded_text.clone()
    }
    
    // Performance getters
    #[wasm_bindgen(getter)]
    pub fn frames_processed(&self) -> u32 {
        self.frames_processed
    }
    
    #[wasm_bindgen(getter)]
    pub fn symbols_decoded(&self) -> u32 {
        self.symbols_decoded
    }
    
    #[wasm_bindgen(getter)]
    pub fn fec_corrections(&self) -> u32 {
        self.fec_corrections
    }
}

/// Configuration structure for WASM
#[derive(Serialize, Deserialize)]
struct StreamConfigWASM {
    simulation: SimulationConfig,
    protocol: ProtocolConfig,
    ldpc: LDPCConfig,
}
