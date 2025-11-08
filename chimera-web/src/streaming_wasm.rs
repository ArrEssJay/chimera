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
        
        // Convert decoded bytes to string
        let decoded_text = String::from_utf8_lossy(&output.decoded_data).to_string();
        
        // Package for JavaScript
        Ok(WASMStreamOutput {
            audio: Float32Array::from(&output.audio_samples[..]),
            constellation_i: Float32Array::from(&output.constellation_i[..]),
            constellation_q: Float32Array::from(&output.constellation_q[..]),
            fft_magnitude: Float32Array::from(&output.fft_magnitude[..]),
            ber: output.ber,
            decoded_text,
            timing_error: output.timing_error,
            mean_evm: output.mean_evm,
            peak_evm: output.peak_evm,
            sync_found: output.sync_found,
            symbol_count: output.symbol_count,
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
    audio: Float32Array,
    constellation_i: Float32Array,
    constellation_q: Float32Array,
    fft_magnitude: Float32Array,
    ber: f32,
    decoded_text: String,
    timing_error: f64,
    mean_evm: f32,
    peak_evm: f32,
    sync_found: bool,
    symbol_count: usize,
}

#[wasm_bindgen]
impl WASMStreamOutput {
    #[wasm_bindgen(getter)]
    pub fn audio(&self) -> Float32Array {
        self.audio.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn constellation_i(&self) -> Float32Array {
        self.constellation_i.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn constellation_q(&self) -> Float32Array {
        self.constellation_q.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn fft_magnitude(&self) -> Float32Array {
        self.fft_magnitude.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn ber(&self) -> f32 {
        self.ber
    }
    
    #[wasm_bindgen(getter)]
    pub fn decoded_text(&self) -> String {
        self.decoded_text.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn timing_error(&self) -> f64 {
        self.timing_error
    }
    
    #[wasm_bindgen(getter)]
    pub fn mean_evm(&self) -> f32 {
        self.mean_evm
    }
    
    #[wasm_bindgen(getter)]
    pub fn peak_evm(&self) -> f32 {
        self.peak_evm
    }
    
    #[wasm_bindgen(getter)]
    pub fn sync_found(&self) -> bool {
        self.sync_found
    }
    
    #[wasm_bindgen(getter)]
    pub fn symbol_count(&self) -> usize {
        self.symbol_count
    }
}

/// Configuration structure for WASM
#[derive(Serialize, Deserialize)]
struct StreamConfigWASM {
    simulation: SimulationConfig,
    protocol: ProtocolConfig,
    ldpc: LDPCConfig,
}
