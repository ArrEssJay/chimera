//! CLI-specific configuration structures for TOML-based configuration.

use chimera_core::config::{
    LDPCConfig, InternalProtocolConfig, UserSimulationConfig,
    ChannelParams, ThzModulationParams, SignalProcessingParams
};
use color_eyre::eyre::{Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};

/// Deep merge JSON values - override takes precedence over base
/// For objects, recursively merge; for arrays and primitives, override replaces base
fn merge_json_values(base: &mut JsonValue, override_val: JsonValue) {
    match (base, override_val) {
        (JsonValue::Object(base_map), JsonValue::Object(override_map)) => {
            // Recursively merge objects
            for (key, value) in override_map {
                if let Some(base_value) = base_map.get_mut(&key) {
                    // Key exists in both - recursively merge
                    merge_json_values(base_value, value);
                } else {
                    // Key only in override - insert it
                    base_map.insert(key, value);
                }
            }
        }
        (base_val, override_val) => {
            // For non-objects, override completely replaces base
            *base_val = override_val;
        }
    }
}

/// Custom deserializer to handle both single string and array of strings for include
fn deserialize_include_optional<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }
    
    let opt: Option<StringOrVec> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(Vec::new()),
        Some(StringOrVec::String(s)) => Ok(vec![s]),
        Some(StringOrVec::Vec(v)) => Ok(v),
    }
}

/// Complete CLI configuration bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Optional path(s) to other config file(s) to include (relative to this config's directory)
    /// Supports single string or array of strings
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_include_optional")]
    pub include: Vec<String>,
    
    /// Protocol configuration (internal, not directly user-configurable)
    #[serde(default, skip_serializing_if = "is_default_protocol")]
    pub protocol: InternalProtocolConfig,
    
    /// Simulation configuration
    #[serde(default, skip_serializing_if = "is_default_simulation")]
    pub simulation: UserSimulationConfig,
    
    /// LDPC configuration
    #[serde(default, skip_serializing_if = "is_default_ldpc")]
    pub ldpc: LDPCConfig,
    
    /// Channel parameters (runtime adjustable)
    #[serde(default)]
    pub channel: ChannelParams,
    
    /// THz modulation parameters (runtime adjustable)
    #[serde(default)]
    pub thz_modulation: ThzModulationParams,
    
    /// Signal processing parameters (runtime adjustable)
    #[serde(default)]
    pub signal_processing: SignalProcessingParams,
    
    /// Terminal interface configuration
    #[serde(default)]
    pub terminal: TerminalConfig,
}

fn is_default_protocol(_p: &InternalProtocolConfig) -> bool {
    false // Always serialize for now
}

fn is_default_simulation(_s: &UserSimulationConfig) -> bool {
    false // Always serialize for now
}

fn is_default_ldpc(_l: &LDPCConfig) -> bool {
    false // Always serialize for now
}

/// Terminal interface configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Output WAV filename (optional)
    #[serde(default)]
    pub wav_output: Option<PathBuf>,
    
    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
    
    /// Telemetry sampling interval in seconds
    #[serde(default = "default_telemetry_interval")]
    pub telemetry_interval_secs: f64,
}

fn default_telemetry_interval() -> f64 {
    1.0
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            wav_output: None,
            logging: LoggingConfig::default(),
            telemetry_interval_secs: default_telemetry_interval(),
        }
    }
}

/// Logging output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log output target
    #[serde(default = "default_log_target")]
    pub target: LogTarget,
    
    /// Log file path (if target is File)
    #[serde(default)]
    pub file_path: Option<PathBuf>,
    
    /// Log format
    #[serde(default = "default_log_format")]
    pub format: LogFormat,
    
    /// Log level filter
    #[serde(default = "default_log_level")]
    pub level: LogLevel,
}

fn default_log_target() -> LogTarget {
    LogTarget::Stdout
}

fn default_log_format() -> LogFormat {
    LogFormat::Json
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            target: default_log_target(),
            file_path: None,
            format: default_log_format(),
            level: default_log_level(),
        }
    }
}

impl LoggingConfig {
    /// Convert CLI logging config to core logging config
    pub fn to_core_log_config(&self) -> chimera_core::logging::LogConfig {
        use chimera_core::logging::{LogConfig as CoreLogConfig, LogLevel as CoreLogLevel};
        
        let level = match self.level {
            LogLevel::Debug => CoreLogLevel::Debug,
            LogLevel::Info => CoreLogLevel::Info,
            LogLevel::Warn => CoreLogLevel::Warn,
            LogLevel::Error => CoreLogLevel::Error,
        };
        
        CoreLogConfig {
            level,
            enable_timing: true,
            enable_carrier: true,
            enable_framing: true,
            enable_fec: true,
            max_entries: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogTarget {
    Stdout,
    Stderr,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Logfmt,
    Pretty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl CliConfig {
    /// Load configuration from TOML file with include support
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .wrap_err_with(|| format!("Failed to read config file: {}", path.display()))?;
        
        // Parse as raw TOML value first to allow partial configs
        let config_toml: toml::Value = toml::from_str(&content)
            .wrap_err("Failed to parse TOML configuration")?;
        
        // Convert to JSON for easier manipulation
        let mut config_json = serde_json::to_value(&config_toml)
            .wrap_err("Failed to convert TOML to JSON")?;
        
        // Extract and process include directives
        let includes = match config_json.get("include") {
            Some(JsonValue::String(s)) => vec![s.clone()],
            Some(JsonValue::Array(arr)) => {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            }
            _ => Vec::new(),
        };
        
        // Remove include from the config JSON
        if let JsonValue::Object(ref mut map) = config_json {
            map.remove("include");
        }
        
        // Always start with defaults as base to support partial configs
        let mut base_json = serde_json::to_value(&Self::default())
            .wrap_err("Failed to create default config")?;
        
        // Process includes in order if present
        if !includes.is_empty() {
            // Load and merge each included file in order
            for include_path in &includes {
                let include_full_path = if Path::new(include_path).is_absolute() {
                    PathBuf::from(include_path)
                } else {
                    // Resolve relative to config file's directory
                    path.parent()
                        .unwrap_or_else(|| Path::new("."))
                        .join(include_path)
                };
                
                let included_config = Self::from_file(&include_full_path)
                    .wrap_err_with(|| format!("Failed to load included config: {}", include_full_path.display()))?;
                
                let included_json = serde_json::to_value(&included_config)
                    .wrap_err("Failed to convert included config to JSON")?;
                
                // Deep merge included config into accumulated config
                merge_json_values(&mut base_json, included_json);
            }
        }
        
        // Finally merge current config on top (it has highest priority)
        merge_json_values(&mut base_json, config_json);
        let config_json = base_json;
        
        // Convert final merged JSON to typed CliConfig
        let config: CliConfig = serde_json::from_value(config_json)
            .wrap_err("Failed to parse final configuration")?;
        
        Ok(config)
    }
    
    /// Create default configuration
    pub fn default() -> Self {
        Self {
            include: Vec::new(),
            protocol: InternalProtocolConfig::default(),
            simulation: UserSimulationConfig::default(),
            ldpc: LDPCConfig::default(),
            channel: ChannelParams::default(),
            thz_modulation: ThzModulationParams::default(),
            signal_processing: SignalProcessingParams::default(),
            terminal: TerminalConfig::default(),
        }
    }
}
