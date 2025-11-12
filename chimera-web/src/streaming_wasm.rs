//! WASM bindings for streaming DSP pipeline
//!
//! Exposes the streaming pipeline to JavaScript for use in web applications.

use wasm_bindgen::prelude::*;
use chimera_core::pipeline::RealtimePipeline;
use chimera_core::config::{UserSimulationConfig, InternalProtocolConfig, LDPCConfig};
use js_sys::Float32Array;
use serde::{Deserialize, Serialize};

/// WASM wrapper for streaming DSP engine
#[wasm_bindgen]
pub struct WASMStreamingDSP {
    pipeline: RealtimePipeline,
}

#[wasm_bindgen]
impl WASMStreamingDSP {
    /// Create a new streaming DSP engine with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WASMStreamingDSP, JsValue> {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        
        // Use default configurations
        let sim = UserSimulationConfig::default();
        let protocol = InternalProtocolConfig::default();
        let ldpc = LDPCConfig::default();
        
        Ok(WASMStreamingDSP {
            pipeline: RealtimePipeline::new(sim, protocol, ldpc),
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
    fn create_output(&mut self, output: chimera_core::pipeline::RealtimeOutput) -> Result<WASMStreamOutput, JsValue> {
        
        // Package for JavaScript
        Ok(WASMStreamOutput {
            audio: Float32Array::from(&output.audio_samples[..]),
            
            // Pre-channel (TX)
            tx_constellation_i: Float32Array::from(&output.pre_channel.tx_constellation_i[..]),
            tx_constellation_q: Float32Array::from(&output.pre_channel.tx_constellation_q[..]),
            tx_spectrum_magnitude: Float32Array::from(&output.pre_channel.tx_spectrum_magnitude[..]),
            tx_spectrum_freq_start_hz: output.pre_channel.spectrum_freq_start_hz,
            tx_spectrum_freq_end_hz: output.pre_channel.spectrum_freq_end_hz,
            frame_count: output.pre_channel.frame_count as u32,
            total_frames: output.pre_channel.total_frames as u32,
            symbol_count: output.pre_channel.symbol_count as u32,
            carrier_freq_hz: output.pre_channel.carrier_freq_hz,
            symbol_rate_hz: output.pre_channel.symbol_rate_hz,
            modulation_type: output.pre_channel.modulation_type,
            fec_rate: output.pre_channel.fec_rate,
            sync_bytes: output.pre_channel.frame_layout.sync_bytes as u32,
            data_bytes: output.pre_channel.frame_layout.data_bytes as u32,
            parity_bytes: output.pre_channel.frame_layout.parity_bytes as u32,
            total_frame_bytes: output.pre_channel.frame_layout.total_bytes as u32,
            
            // Post-channel (RX)
            rx_constellation_i: Float32Array::from(&output.post_channel.rx_constellation_i[..]),
            rx_constellation_q: Float32Array::from(&output.post_channel.rx_constellation_q[..]),
            rx_spectrum_magnitude: Float32Array::from(&output.post_channel.rx_spectrum_magnitude[..]),
            rx_spectrum_freq_start_hz: output.post_channel.spectrum_freq_start_hz,
            rx_spectrum_freq_end_hz: output.post_channel.spectrum_freq_end_hz,
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
            
            // Current frame data
            frame_number: output.current_frame_data.frame_number as u32,
            frame_sync_data: output.current_frame_data.sync_data,
            frame_payload_data: output.current_frame_data.payload_data,
            frame_parity_data: output.current_frame_data.parity_data,
            frame_decoded_text: output.current_frame_data.decoded_text,
            frame_symbol_progress: output.current_frame_data.symbol_progress as u32,
            
            // FSK layer state
            fsk_state: output.fsk_state.map(|fsk| WASMFSKState {
                current_frequency_hz: fsk.current_frequency_hz,
                frequency_deviation_hz: fsk.frequency_deviation_hz,
                current_bit: fsk.current_bit,
                bit_index: fsk.bit_index,
                bit_history: fsk.bit_history,
                symbols_per_bit: fsk.symbols_per_bit,
                bit_rate_hz: fsk.bit_rate_hz,
            }),
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
    
    /// Update channel parameters (SNR and link loss) without resetting the pipeline
    #[wasm_bindgen]
    pub fn update_channel(&mut self, snr_db: f64, link_loss_db: f64) {
        self.pipeline.update_channel_params(snr_db, link_loss_db);
    }
    
    /// Update SNR only
    #[wasm_bindgen]
    pub fn update_snr(&mut self, snr_db: f64) {
        self.pipeline.update_channel_params(snr_db, self.pipeline.get_link_loss());
    }
    
    /// Update link loss only
    #[wasm_bindgen]
    pub fn update_link_loss(&mut self, link_loss_db: f64) {
        self.pipeline.update_channel_params(self.pipeline.get_snr(), link_loss_db);
    }
    
    /// Update message (waits for current transmission to complete before applying)
    #[wasm_bindgen]
    pub fn update_message(&mut self, message: String) -> Result<(), JsValue> {
        self.pipeline.update_message(message)
            .map_err(|e| JsValue::from_str(&format!("Failed to update message: {}", e)))
    }
    
    /// Update command by string name (e.g., "send_data", "data_transfer")
    #[wasm_bindgen]
    pub fn update_command(&mut self, command: String) -> Result<(), JsValue> {
        self.pipeline.update_command(command)
            .map_err(|e| JsValue::from_str(&format!("Failed to update command: {}", e)))
    }
    
    /// Update target ID
    #[wasm_bindgen]
    pub fn update_target_id(&mut self, target_id: String) -> Result<(), JsValue> {
        self.pipeline.update_target_id(target_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to update target ID: {}", e)))
    }
    
    /// Set THz modulation mode (false = idle <5%, true = active 70-80%)
    #[wasm_bindgen]
    pub fn set_modulation_mode(&mut self, active: bool) {
        self.pipeline.set_modulation_mode(active);
    }
    
    /// Set custom modulation depth (0.0 to 1.0)
    /// Typical values: idle = 0.01-0.05, active = 0.70-0.80
    #[wasm_bindgen]
    pub fn set_modulation_depth(&mut self, depth: f32) {
        self.pipeline.set_modulation_depth(depth);
    }
    
    /// Set mixing coefficient for third-order intermodulation
    #[wasm_bindgen]
    pub fn set_mixing_coefficient(&mut self, coefficient: f32) {
        self.pipeline.set_mixing_coefficient(coefficient);
    }
    
    /// Enable/disable QPSK modulation (debug control)
    #[wasm_bindgen]
    pub fn set_qpsk_enabled(&mut self, enabled: bool) {
        self.pipeline.set_qpsk_enabled(enabled);
    }
    
    /// Enable/disable FSK frequency dithering (debug control)
    #[wasm_bindgen]
    pub fn set_fsk_enabled(&mut self, enabled: bool) {
        self.pipeline.set_fsk_enabled(enabled);
    }
    
    /// Set TX and RX gains
    #[wasm_bindgen]
    pub fn set_gains(&mut self, tx_gain: f64, rx_gain: f64) {
        self.pipeline.set_tx_gain(tx_gain as f32);
        self.pipeline.set_rx_gain(rx_gain as f32);
    }
    
    /// Set TX gain only
    #[wasm_bindgen]
    pub fn set_tx_gain(&mut self, gain: f64) {
        self.pipeline.set_tx_gain(gain as f32);
    }
    
    /// Set RX gain only
    #[wasm_bindgen]
    pub fn set_rx_gain(&mut self, gain: f64) {
        self.pipeline.set_rx_gain(gain as f32);
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
    tx_spectrum_freq_start_hz: f32,
    tx_spectrum_freq_end_hz: f32,
    frame_count: u32,
    total_frames: u32,
    symbol_count: u32,
    carrier_freq_hz: f64,
    symbol_rate_hz: u32,
    modulation_type: String,
    fec_rate: String,
    sync_bytes: u32,
    data_bytes: u32,
    parity_bytes: u32,
    total_frame_bytes: u32,
    
    // Post-channel (RX)
    rx_constellation_i: Float32Array,
    rx_constellation_q: Float32Array,
    rx_spectrum_magnitude: Float32Array,
    rx_spectrum_freq_start_hz: f32,
    rx_spectrum_freq_end_hz: f32,
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
    
    // Current frame data
    frame_number: u32,
    frame_sync_data: Vec<u8>,
    frame_payload_data: Vec<u8>,
    frame_parity_data: Vec<u8>,
    frame_decoded_text: String,
    frame_symbol_progress: u32,
    
    // FSK layer state (optional, decoded from received signal)
    fsk_state: Option<WASMFSKState>,
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
    pub fn tx_spectrum_freq_start_hz(&self) -> f32 {
        self.tx_spectrum_freq_start_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn tx_spectrum_freq_end_hz(&self) -> f32 {
        self.tx_spectrum_freq_end_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
    
    #[wasm_bindgen(getter)]
    pub fn total_frames(&self) -> u32 {
        self.total_frames
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
    
    #[wasm_bindgen(getter)]
    pub fn sync_bytes(&self) -> u32 {
        self.sync_bytes
    }
    
    #[wasm_bindgen(getter)]
    pub fn data_bytes(&self) -> u32 {
        self.data_bytes
    }
    
    #[wasm_bindgen(getter)]
    pub fn parity_bytes(&self) -> u32 {
        self.parity_bytes
    }
    
    #[wasm_bindgen(getter)]
    pub fn total_frame_bytes(&self) -> u32 {
        self.total_frame_bytes
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
    pub fn rx_spectrum_freq_start_hz(&self) -> f32 {
        self.rx_spectrum_freq_start_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn rx_spectrum_freq_end_hz(&self) -> f32 {
        self.rx_spectrum_freq_end_hz
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
    
    // Current frame data getters
    #[wasm_bindgen(getter)]
    pub fn frame_number(&self) -> u32 {
        self.frame_number
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_sync_data(&self) -> Vec<u8> {
        self.frame_sync_data.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_payload_data(&self) -> Vec<u8> {
        self.frame_payload_data.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_parity_data(&self) -> Vec<u8> {
        self.frame_parity_data.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_decoded_text(&self) -> String {
        self.frame_decoded_text.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn frame_symbol_progress(&self) -> u32 {
        self.frame_symbol_progress
    }
    
    #[wasm_bindgen(getter)]
    pub fn fsk_state(&self) -> Option<WASMFSKState> {
        self.fsk_state.clone()
    }
}

/// Configuration structure for WASM
#[derive(Serialize, Deserialize)]
struct StreamConfigWASM {
    simulation: UserSimulationConfig,
    protocol: InternalProtocolConfig,
    ldpc: LDPCConfig,
}

/// FSK layer state for WASM
#[wasm_bindgen]
#[derive(Clone)]
pub struct WASMFSKState {
    current_frequency_hz: f64,
    frequency_deviation_hz: f64,
    current_bit: u8,
    bit_index: usize,
    bit_history: Vec<u8>,
    symbols_per_bit: usize,
    bit_rate_hz: f64,
}

#[wasm_bindgen]
impl WASMFSKState {
    #[wasm_bindgen(getter)]
    pub fn current_frequency_hz(&self) -> f64 {
        self.current_frequency_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn frequency_deviation_hz(&self) -> f64 {
        self.frequency_deviation_hz
    }
    
    #[wasm_bindgen(getter)]
    pub fn current_bit(&self) -> u8 {
        self.current_bit
    }
    
    #[wasm_bindgen(getter)]
    pub fn bit_index(&self) -> usize {
        self.bit_index
    }
    
    #[wasm_bindgen(getter)]
    pub fn bit_history(&self) -> Vec<u8> {
        self.bit_history.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn symbols_per_bit(&self) -> usize {
        self.symbols_per_bit
    }
    
    #[wasm_bindgen(getter)]
    pub fn bit_rate_hz(&self) -> f64 {
        self.bit_rate_hz
    }
}
