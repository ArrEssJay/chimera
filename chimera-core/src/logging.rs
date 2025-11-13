//! Unified logging infrastructure for Chimera signal processing
//!
//! This module provides structured logging that can be configured and
//! accessed from both CLI and WASM interfaces.

use std::fmt;

/// Log level for filtering messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Minimum log level to record
    pub level: LogLevel,
    
    /// Enable logging for specific subsystems
    pub enable_timing: bool,
    pub enable_carrier: bool,
    pub enable_framing: bool,
    pub enable_fec: bool,
    
    /// Maximum number of log entries to keep (for memory management)
    pub max_entries: usize,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enable_timing: true,
            enable_carrier: true,
            enable_framing: true,
            enable_fec: true,
            max_entries: 1000,
        }
    }
}

impl LogConfig {
    /// Verbose logging for debugging
    pub fn verbose() -> Self {
        Self {
            level: LogLevel::Debug,
            enable_timing: true,
            enable_carrier: true,
            enable_framing: true,
            enable_fec: true,
            max_entries: 5000,
        }
    }
    
    /// Quiet logging for production
    pub fn quiet() -> Self {
        Self {
            level: LogLevel::Warn,
            enable_timing: false,
            enable_carrier: false,
            enable_framing: false,
            enable_fec: false,
            max_entries: 100,
        }
    }
    
    /// Disable all logging
    pub fn disabled() -> Self {
        Self {
            level: LogLevel::Error,
            enable_timing: false,
            enable_carrier: false,
            enable_framing: false,
            enable_fec: false,
            max_entries: 0,
        }
    }
}

/// A single log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub subsystem: &'static str,
    pub message: String,
}

/// Logger that collects structured log entries
#[derive(Debug, Clone)]
pub struct SignalLogger {
    config: LogConfig,
    entries: Vec<LogEntry>,
}

impl SignalLogger {
    pub fn new(config: LogConfig) -> Self {
        let capacity = config.max_entries.min(1000);
        Self {
            config,
            entries: Vec::with_capacity(capacity),
        }
    }
    
    /// Log a message at the specified level
    pub fn log(&mut self, level: LogLevel, subsystem: &'static str, message: impl fmt::Display) {
        if level < self.config.level {
            return;
        }
        
        // Check subsystem filters
        let enabled = match subsystem {
            "TIMING" | "GARDNER" | "DECIMATE" => self.config.enable_timing,
            "CARRIER" | "COSTAS" | "SYNC" | "FSK" | "BOOTSTRAP" => self.config.enable_carrier,
            "FRAME" | "TRACK" => self.config.enable_framing,
            "FEC" | "LDPC" => self.config.enable_fec,
            _ => true, // Unknown subsystems always log
        };
        
        if !enabled {
            return;
        }
        
        let entry = LogEntry {
            level,
            subsystem,
            message: message.to_string(),
        };
        
        // Respect max_entries limit
        if self.config.max_entries > 0 {
            if self.entries.len() >= self.config.max_entries {
                // Remove oldest entry
                self.entries.remove(0);
            }
            self.entries.push(entry);
        }
        
        // Also print to stderr in test/debug mode for immediate visibility
        #[cfg(any(test, debug_assertions))]
        {
            eprintln!("[{}] {}: {}", level, subsystem, message);
        }
    }
    
    /// Convenience methods for each log level
    pub fn trace(&mut self, subsystem: &'static str, message: impl fmt::Display) {
        self.log(LogLevel::Trace, subsystem, message);
    }
    
    pub fn debug(&mut self, subsystem: &'static str, message: impl fmt::Display) {
        self.log(LogLevel::Debug, subsystem, message);
    }
    
    pub fn info(&mut self, subsystem: &'static str, message: impl fmt::Display) {
        self.log(LogLevel::Info, subsystem, message);
    }
    
    pub fn warn(&mut self, subsystem: &'static str, message: impl fmt::Display) {
        self.log(LogLevel::Warn, subsystem, message);
    }
    
    pub fn error(&mut self, subsystem: &'static str, message: impl fmt::Display) {
        self.log(LogLevel::Error, subsystem, message);
    }
    
    /// Get all log entries
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }
    
    /// Get formatted log output
    pub fn to_string(&self) -> String {
        self.entries
            .iter()
            .map(|e| format!("[{}] {}: {}", e.level, e.subsystem, e.message))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Get entries for a specific subsystem
    pub fn entries_for_subsystem(&self, subsystem: &str) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.subsystem == subsystem)
            .collect()
    }
    
    /// Get entries at or above a specific level
    pub fn entries_at_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level >= level)
            .collect()
    }
    
    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    /// Get count of entries at each level
    pub fn level_counts(&self) -> (usize, usize, usize, usize, usize) {
        let mut trace = 0;
        let mut debug = 0;
        let mut info = 0;
        let mut warn = 0;
        let mut error = 0;
        
        for entry in &self.entries {
            match entry.level {
                LogLevel::Trace => trace += 1,
                LogLevel::Debug => debug += 1,
                LogLevel::Info => info += 1,
                LogLevel::Warn => warn += 1,
                LogLevel::Error => error += 1,
            }
        }
        
        (trace, debug, info, warn, error)
    }
}

impl Default for SignalLogger {
    fn default() -> Self {
        Self::new(LogConfig::default())
    }
}

impl fmt::Display for SignalLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_filtering_by_level() {
        let config = LogConfig {
            level: LogLevel::Info,
            ..Default::default()
        };
        let mut logger = SignalLogger::new(config);
        
        logger.trace("TEST", "trace message");
        logger.debug("TEST", "debug message");
        logger.info("TEST", "info message");
        logger.warn("TEST", "warn message");
        
        assert_eq!(logger.entries().len(), 2); // info and warn only
    }
    
    #[test]
    fn test_subsystem_filtering() {
        let config = LogConfig {
            level: LogLevel::Debug,
            enable_timing: false,
            enable_carrier: true,
            ..Default::default()
        };
        let mut logger = SignalLogger::new(config);
        
        logger.info("TIMING", "timing message");
        logger.info("CARRIER", "carrier message");
        
        assert_eq!(logger.entries().len(), 1); // carrier only
        assert_eq!(logger.entries()[0].subsystem, "CARRIER");
    }
    
    #[test]
    fn test_max_entries_limit() {
        let config = LogConfig {
            level: LogLevel::Debug,
            max_entries: 3,
            ..Default::default()
        };
        let mut logger = SignalLogger::new(config);
        
        logger.info("TEST", "message 1");
        logger.info("TEST", "message 2");
        logger.info("TEST", "message 3");
        logger.info("TEST", "message 4");
        
        assert_eq!(logger.entries().len(), 3); // limited to 3
        assert!(logger.entries()[0].message.contains("message 2")); // oldest removed
    }
}
