//! Configuration types for the Chimera pipeline.
use serde::{Deserialize, Serialize};
use crate::errors::{ConfigError, Result};
use crate::protocol::CommandType;
use std::sync::{Arc, RwLock};

// Default value functions for serde
fn default_true() -> bool { true }

// ============================================================================
// PUBLIC API - User-facing configuration
// ============================================================================

/// Public configuration that users can modify via TOML or API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserConfig {
    /// Protocol selection (defaults to "whisper")
    pub protocol: String,
    
    /// Protocol-specific parameters that users can configure
    #[serde(default)]
    pub protocol_params: UserProtocolParams,
    
    /// Simulation settings for testing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation: Option<UserSimulationConfig>,
    
    /// Channel parameters (runtime adjustable)
    #[serde(default)]
    pub channel: ChannelParams,
    
    /// THz carrier modulation control (runtime adjustable)
    #[serde(default)]
    pub thz_modulation: ThzModulationParams,
    
    /// Signal processing controls (runtime adjustable)
    #[serde(default)]
    pub signal_processing: SignalProcessingParams,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            protocol: "whisper".to_string(),
            protocol_params: UserProtocolParams::default(),
            simulation: None,
            channel: ChannelParams::default(),
            thz_modulation: ThzModulationParams::default(),
            signal_processing: SignalProcessingParams::default(),
        }
    }
}

/// User-configurable protocol parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserProtocolParams {
    /// Command to send (as string, e.g., "send_data", "data_transfer")
    pub command: String,
    
    /// Target device identifier (hex string)
    #[serde(alias = "target_id_hex")]
    pub target_id: String,
}

impl Default for UserProtocolParams {
    fn default() -> Self {
        Self {
            command: "send_data".to_string(),
            target_id: "DEADBEEF".to_string(),
        }
    }
}

impl UserProtocolParams {
    /// Get command opcode from the command string
    pub fn get_command_opcode(&self) -> Result<u32> {
        CommandType::from_str(&self.command)
            .map(|cmd| cmd.to_opcode())
            .ok_or_else(|| ConfigError::InvalidFrameLayout {
                reason: format!("Unknown command: {}", self.command)
            }.into())
    }
}

/// User simulation configuration (static, set at startup)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSimulationConfig {
    /// Plaintext message to encode (runtime adjustable, waits for current transmission)
    #[serde(alias = "plaintext_source")]
    pub message: String,
    
    /// Audio mixing configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_mixing: Option<AudioMixingConfig>,
    
    /// Bypass THz simulation (for validation)
    #[serde(default)]
    pub bypass_thz_simulation: bool,
}

impl Default for UserSimulationConfig {
    fn default() -> Self {
        Self {
            message: "Default test message".to_string(),
            audio_mixing: None,
            bypass_thz_simulation: false,
        }
    }
}

// ============================================================================
// RUNTIME ADJUSTABLE PARAMETERS
// ============================================================================

/// Channel simulation parameters (adjustable in realtime)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ChannelParams {
    /// Signal-to-Noise Ratio in dB
    pub snr_db: f64,
    
    /// Link loss/attenuation in dB
    pub link_loss_db: f64,
}

impl Default for ChannelParams {
    fn default() -> Self {
        Self {
            snr_db: 20.0,
            link_loss_db: 0.0,
        }
    }
}

/// THz carrier modulation parameters (adjustable in realtime)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThzModulationParams {
    /// Enable/disable THz carrier modulation
    pub enabled: bool,
    
    /// Modulation depth (0.0 to 1.0)
    pub modulation_depth: f64,
    
    /// Enable/disable second-order modulation
    pub enable_second_order: bool,
    
    /// Second-order modulation coefficient (0.0 to 1.0)
    pub second_order_coefficient: f64,
    
    /// Enable/disable third-order modulation
    pub enable_third_order: bool,
    
    /// Third-order modulation coefficient (0.0 to 1.0)
    pub third_order_coefficient: f64,
    
    /// Cortical processing coefficient (0.0 to 1.0)
    pub cortical_coefficient: f64,
}

impl Default for ThzModulationParams {
    fn default() -> Self {
        Self {
            enabled: true,
            modulation_depth: 0.5,
            enable_second_order: true,
            second_order_coefficient: 0.3,
            enable_third_order: true,
            third_order_coefficient: 0.2,
            cortical_coefficient: 0.4,
        }
    }
}

/// Signal processing controls (adjustable in realtime)
/// Note: FEC is always enabled internally and not exposed in user API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SignalProcessingParams {
    /// Enable/disable QPSK modulation
    pub enable_qpsk: bool,
    
    /// Enable/disable FSK modulation
    pub enable_fsk: bool,
    
    /// Transmit gain control
    pub tx_gain: f64,
    
    /// Receive gain control
    pub rx_gain: f64,
}

impl Default for SignalProcessingParams {
    fn default() -> Self {
        Self {
            enable_qpsk: true,
            enable_fsk: true,
            tx_gain: 1.0,
            rx_gain: 1.0,
        }
    }
}

// ============================================================================
// REALTIME CONTROL API
// ============================================================================

/// Runtime parameters that can be adjusted
#[derive(Debug, Clone)]
pub struct RuntimeParams {
    pub channel: ChannelParams,
    pub thz_modulation: ThzModulationParams,
    pub signal_processing: SignalProcessingParams,
    pub message: Option<String>, // Current message (updates wait for transmission complete)
}

impl RuntimeParams {
    pub fn new(config: &UserConfig, simulation: Option<&UserSimulationConfig>) -> Self {
        Self {
            channel: config.channel.clone(),
            thz_modulation: config.thz_modulation.clone(),
            signal_processing: config.signal_processing.clone(),
            message: simulation.map(|s| s.message.clone()),
        }
    }
}

/// Handle for adjusting parameters during runtime
#[derive(Clone)]
pub struct RealtimeController {
    params: Arc<RwLock<RuntimeParams>>,
}

impl RealtimeController {
    pub fn new(initial: RuntimeParams) -> Self {
        Self {
            params: Arc::new(RwLock::new(initial)),
        }
    }
    
    /// Get a thread-safe handle to the parameters
    pub fn get_handle(&self) -> Arc<RwLock<RuntimeParams>> {
        Arc::clone(&self.params)
    }
    
    /// Update SNR in realtime
    pub fn set_snr(&self, snr_db: f64) -> Result<()> {
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        params.channel.snr_db = snr_db;
        Ok(())
    }
    
    /// Update link loss in realtime
    pub fn set_link_loss(&self, link_loss_db: f64) -> Result<()> {
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        params.channel.link_loss_db = link_loss_db;
        Ok(())
    }
    
    /// Update THz modulation depth
    pub fn set_modulation_depth(&self, depth: f64) -> Result<()> {
        if !(0.0..=1.0).contains(&depth) {
            return Err(ConfigError::InvalidFrameLayout {
                reason: "Modulation depth must be between 0.0 and 1.0".into()
            }.into());
        }
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        params.thz_modulation.modulation_depth = depth;
        Ok(())
    }
    
    /// Enable/disable QPSK modulation
    pub fn set_qpsk_enabled(&self, enabled: bool) -> Result<()> {
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        params.signal_processing.enable_qpsk = enabled;
        Ok(())
    }
    
    /// Enable/disable FSK modulation
    pub fn set_fsk_enabled(&self, enabled: bool) -> Result<()> {
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        params.signal_processing.enable_fsk = enabled;
        Ok(())
    }
    
    /// Update message (this should wait for current transmission to complete)
    pub fn set_message(&self, message: String) -> Result<()> {
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        params.message = Some(message);
        Ok(())
    }
    
    /// Batch update multiple parameters
    pub fn update<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut RuntimeParams),
    {
        let mut params = self.params.write()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        updater(&mut params);
        Ok(())
    }
    
    /// Get current snapshot of parameters
    pub fn get_snapshot(&self) -> Result<RuntimeParams> {
        let params = self.params.read()
            .map_err(|_| ConfigError::InvalidSnr { snr_db: 0.0 })?;
        Ok(params.clone())
    }
}

// ============================================================================
// INTERNAL PROTOCOL CONFIGURATION
// ============================================================================

/// Internal protocol configuration - NOT exposed to users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalProtocolConfig {
    pub carrier_freq_hz: f64,
    pub qpsk_symbol_rate: usize,
    pub qpsk_bandwidth_hz: f64,
    pub fsk_bit_rate: f64,
    pub fsk_freq_zero_hz: f64,
    pub fsk_freq_one_hz: f64,
    pub sync_sequence_hex: String,
    pub frame_layout: FrameLayout,
    pub current_frame_shift: usize,
    pub total_frames_shift: usize,
    // User-configurable fields (use CommandType, not raw opcode)
    pub command: String,  // Command name (e.g., "send_data")
    pub target_id_hex: String,
    pub max_frames: usize,
    // Runtime control
    pub enable_qpsk: bool,
    pub enable_fsk: bool,
}

impl InternalProtocolConfig {
    pub fn fsk_freq_deviation_hz(&self) -> f64 {
        self.fsk_freq_one_hz - self.carrier_freq_hz
    }
    
    /// Get the command opcode from the command string
    pub fn get_command_opcode(&self) -> u32 {
        CommandType::from_str(&self.command)
            .map(|cmd| cmd.to_opcode())
            .unwrap_or(0x0001) // Default to send_data
    }
    
    /// Validate protocol configuration
    pub fn validate(&self) -> Result<()> {
        // Check sample rate validity
        if self.qpsk_symbol_rate == 0 {
            return Err(ConfigError::InvalidSymbolRate { 
                rate: self.qpsk_symbol_rate 
            }.into());
        }
        
        // Check Nyquist criterion for carrier frequency
        let min_sample_rate = self.carrier_freq_hz * 2.0;
        let actual_sample_rate = SystemConfig::SAMPLE_RATE as f64;
        if actual_sample_rate < min_sample_rate {
            return Err(ConfigError::NyquistViolation {
                carrier_hz: self.carrier_freq_hz,
                min_required_hz: min_sample_rate,
                actual_hz: actual_sample_rate,
            }.into());
        }
        
        // Validate FSK frequencies
        if !self.fsk_freq_zero_hz.is_finite() || !self.fsk_freq_one_hz.is_finite() {
            return Err(ConfigError::InvalidFskFrequencies {
                f0: self.fsk_freq_zero_hz,
                f1: self.fsk_freq_one_hz,
            }.into());
        }
        
        // Validate frame layout
        self.frame_layout.validate()?;
        
        Ok(())
    }
}

/// Get hardcoded protocol preset by name
pub fn get_protocol_preset(name: &str) -> Result<InternalProtocolConfig> {
    match name {
        "whisper" => Ok(InternalProtocolConfig {
            // Hardcoded whisper protocol parameters
            carrier_freq_hz: 12000.0,
            qpsk_symbol_rate: 16,
            qpsk_bandwidth_hz: 20.0,
            fsk_bit_rate: 1.0,
            fsk_freq_zero_hz: 11999.0,
            fsk_freq_one_hz: 12001.0,
            sync_sequence_hex: "A5A5A5A5".to_string(),
            frame_layout: FrameLayout::default(),
            current_frame_shift: 16,
            total_frames_shift: 24,
            // Default user-configurable values
            command: "send_data".to_string(),
            target_id_hex: "DEADBEEF".to_string(),
            max_frames: 256,
            // Runtime defaults
            enable_qpsk: true,
            enable_fsk: true,
        }),
        _ => Err(ConfigError::InvalidFrameLayout {
            reason: format!("Unknown protocol: {}", name)
        }.into()),
    }
}

impl Default for InternalProtocolConfig {
    fn default() -> Self {
        get_protocol_preset("whisper").unwrap()
    }
}

// ============================================================================
// CONFIG BUILDER API
// ============================================================================

/// Builder for creating a complete system configuration
pub struct ConfigBuilder {
    user_config: UserConfig,
}

impl ConfigBuilder {
    /// Create a new configuration builder with defaults
    pub fn new() -> Self {
        Self {
            user_config: UserConfig::default(),
        }
    }
    
    /// Load from a UserConfig (e.g., from TOML file)
    pub fn from_user_config(config: UserConfig) -> Self {
        Self {
            user_config: config,
        }
    }
    
    /// Set the protocol to use
    pub fn protocol(mut self, protocol: &str) -> Self {
        self.user_config.protocol = protocol.to_string();
        self
    }
    
    /// Set the command by name (e.g., "send_data", "data_transfer")
    pub fn command(mut self, command: &str) -> Self {
        self.user_config.protocol_params.command = command.to_string();
        self
    }
    
    /// Set the target ID
    pub fn target(mut self, target_id: &str) -> Self {
        self.user_config.protocol_params.target_id = target_id.to_string();
        self
    }
    
    /// Enable simulation with parameters
    pub fn with_simulation(mut self, sim: UserSimulationConfig) -> Self {
        self.user_config.simulation = Some(sim);
        self
    }
    
    /// Set channel parameters
    pub fn with_channel_params(mut self, params: ChannelParams) -> Self {
        self.user_config.channel = params;
        self
    }
    
    /// Build the final system configuration
    pub fn build(self) -> Result<SystemConfig> {
        // Get the internal protocol preset
        let mut protocol_config = get_protocol_preset(&self.user_config.protocol)?;
        
        // Apply user-configurable protocol parameters
        protocol_config.command = self.user_config.protocol_params.command.clone();
        protocol_config.target_id_hex = self.user_config.protocol_params.target_id.clone();
        
        // Create runtime parameters
        let runtime_params = RuntimeParams::new(
            &self.user_config,
            self.user_config.simulation.as_ref()
        );
        
        // Create realtime controller
        let realtime_controller = RealtimeController::new(runtime_params);
        
        Ok(SystemConfig {
            protocol: protocol_config,
            simulation: self.user_config.simulation,
            realtime: realtime_controller,
        })
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete system configuration (internal use)
pub struct SystemConfig {
    pub protocol: InternalProtocolConfig,
    pub simulation: Option<UserSimulationConfig>,
    pub realtime: RealtimeController,
}

impl SystemConfig {
    /// Sample rate constant (48 kHz)
    pub const SAMPLE_RATE: usize = 48_000;
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameLayout {
    pub total_symbols: usize,
    pub sync_symbols: usize,
    pub target_id_symbols: usize,
    pub command_type_symbols: usize,
    pub data_payload_symbols: usize,
    pub ecc_symbols: usize,
}

impl Default for FrameLayout {
    fn default() -> Self {
        Self {
            total_symbols: 128,
            sync_symbols: 16,
            target_id_symbols: 16,
            command_type_symbols: 16,
            data_payload_symbols: 64,
            ecc_symbols: 16,
        }
    }
}

impl FrameLayout {
    pub fn message_bits(&self) -> usize {
        self.data_payload_symbols * 2
    }

    pub fn ecc_bits(&self) -> usize {
        self.ecc_symbols * 2
    }

    pub fn codeword_bits(&self) -> usize {
        self.message_bits() + self.ecc_bits()
    }

    pub fn frame_bits(&self) -> usize {
        self.total_symbols * 2
    }
    
    /// Validate frame layout consistency
    pub fn validate(&self) -> Result<()> {
        let computed_total = self.sync_symbols 
            + self.target_id_symbols 
            + self.command_type_symbols 
            + self.data_payload_symbols 
            + self.ecc_symbols;
        
        if computed_total != self.total_symbols {
            return Err(ConfigError::InvalidFrameLayout {
                reason: format!(
                    "Symbol sum mismatch: {} + {} + {} + {} + {} = {}, expected {}",
                    self.sync_symbols,
                    self.target_id_symbols,
                    self.command_type_symbols,
                    self.data_payload_symbols,
                    self.ecc_symbols,
                    computed_total,
                    self.total_symbols
                )
            }.into());
        }
        
        if self.total_symbols == 0 {
            return Err(ConfigError::InvalidFrameLayout {
                reason: "total_symbols cannot be zero".to_string()
            }.into());
        }
        
        Ok(())
    }
}

/// Audio source type for intermodulation mixing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AudioSource {
    /// No external audio
    None,
    /// Load from audio file (mp3, m4a, wav, flac)
    File { 
        path: String,
        /// Loop audio if shorter than AID signal (default: true)
        #[serde(default = "default_true")]
        #[serde(rename = "loop")]
        loop_audio: bool,
    },
    /// Generate test signal
    Generator { 
        preset: GeneratorPreset,
        #[serde(default = "default_generator_duration")]
        duration_secs: f64,
    },
}

fn default_generator_duration() -> f64 { 5.0 }

impl Default for AudioSource {
    fn default() -> Self {
        Self::None
    }
}

/// Generator presets for audio testing
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorPreset {
    /// Pink noise (1/f spectrum) - good for general frequency response testing
    PinkNoise,
    /// Constant sine tone at 1kHz - standard reference tone
    Tone1kHz,
    /// Constant sine tone at specified frequency in Hz
    Tone(f64),
    /// Linear frequency sweep 100Hz to 20kHz
    SweepLinear,
    /// Logarithmic frequency sweep 100Hz to 20kHz (equal time per octave)
    SweepLog,
}

/// Configuration for external audio intermodulation mixing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioMixingConfig {
    /// Audio source configuration
    #[serde(default)]
    pub audio_source: AudioSource,
    
    /// Gain applied to external audio before mixing (0.0 to 1.0)
    #[serde(default = "default_external_audio_gain")]
    pub external_audio_gain: f32,
    
    /// Gain applied to AID signal before mixing (0.0 to 1.0)
    #[serde(default = "default_aid_signal_gain")]
    pub aid_signal_gain: f32,
    
    /// Enable second-order intermodulation (sum and difference products)
    #[serde(default = "default_true")]
    pub enable_second_order: bool,
    
    /// Enable third-order intermodulation (more complex products)
    #[serde(default = "default_true")]
    pub enable_third_order: bool,
    
    /// Second-order intermodulation coefficient (0.0 to 1.0)
    #[serde(default = "default_second_order_coeff")]
    pub second_order_coefficient: f32,
    
    /// Third-order intermodulation coefficient (0.0 to 1.0)
    #[serde(default = "default_third_order_coeff")]
    pub third_order_coefficient: f32,
    
    /// Cortical integration coefficient - simulates perceptual blending (0.0 to 1.0)
    #[serde(default = "default_cortical_coeff")]
    pub cortical_coefficient: f32,
}

fn default_external_audio_gain() -> f32 { 0.5 }
fn default_aid_signal_gain() -> f32 { 0.5 }
fn default_second_order_coeff() -> f32 { 0.3 }
fn default_third_order_coeff() -> f32 { 0.2 }
fn default_cortical_coeff() -> f32 { 0.4 }

impl Default for AudioMixingConfig {
    fn default() -> Self {
        Self {
            audio_source: AudioSource::None,
            external_audio_gain: default_external_audio_gain(),
            aid_signal_gain: default_aid_signal_gain(),
            enable_second_order: true,
            enable_third_order: true,
            second_order_coefficient: default_second_order_coeff(),
            third_order_coefficient: default_third_order_coeff(),
            cortical_coefficient: default_cortical_coeff(),
        }
    }
}

// LDPC configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LDPCConfig {
    pub dv: usize,
    pub dc: usize,
    pub seed: Option<u64>,
}

impl Default for LDPCConfig {
    fn default() -> Self {
        Self {
            dv: 2,
            dc: 10,
            seed: Some(42),
        }
    }
}
