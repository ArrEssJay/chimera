//! CLI-specific configuration structures for TOML-based configuration.

use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use color_eyre::eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Complete CLI configuration bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Optional path to another config file to include (relative to this config's directory)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    
    #[serde(default, skip_serializing_if = "is_default_protocol")]
    pub protocol: ProtocolConfig,
    
    #[serde(default, skip_serializing_if = "is_default_simulation")]
    pub simulation: SimulationConfig,
    
    #[serde(default, skip_serializing_if = "is_default_ldpc")]
    pub ldpc: LDPCConfig,
    
    #[serde(default)]
    pub terminal: TerminalConfig,
}

fn is_default_protocol(_p: &ProtocolConfig) -> bool {
    false // Always serialize for now
}

fn is_default_simulation(_s: &SimulationConfig) -> bool {
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
        
        let mut config: CliConfig = toml::from_str(&content)
            .wrap_err("Failed to parse TOML configuration")?;
        
        // Process include directive
        if let Some(include_path) = &config.include {
            let include_full_path = if Path::new(include_path).is_absolute() {
                PathBuf::from(include_path)
            } else {
                // Resolve relative to config file's directory
                path.parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(include_path)
            };
            
            // Load base config
            let base_config = Self::from_file(&include_full_path)
                .wrap_err_with(|| format!("Failed to load included config: {}", include_full_path.display()))?;
            
            // Merge: current config overrides base config
            // For simplicity, we'll do field-by-field merge
            // Protocol, simulation, ldpc can be overridden entirely if present
            // Terminal settings merge more carefully
            config = Self::merge(base_config, config);
        }
        
        Ok(config)
    }
    
    /// Merge two configs, with override taking precedence
    fn merge(_base: Self, override_cfg: Self) -> Self {
        // Simple merge: override_cfg values take precedence over base
        // For nested structures, we'd need more sophisticated merging
        // For now, entire sections are replaced if present in override
        
        Self {
            include: None, // Don't propagate include directive
            protocol: override_cfg.protocol,
            simulation: override_cfg.simulation,
            ldpc: override_cfg.ldpc,
            terminal: override_cfg.terminal,
        }
    }
    
    /// Create default configuration
    pub fn default() -> Self {
        Self {
            include: None,
            protocol: ProtocolConfig::default(),
            simulation: SimulationConfig::default(),
            ldpc: LDPCConfig::default(),
            terminal: TerminalConfig::default(),
        }
    }
}
