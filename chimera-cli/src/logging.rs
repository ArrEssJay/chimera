//! Structured logging for telemetry and frame analysis.

use crate::config::{LogFormat, LogTarget, LoggingConfig};
use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufWriter, Write};

/// Structured log event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LogEvent {
    /// Telemetry sample event
    Telemetry(TelemetryEvent),
    
    /// Frame decode event
    FrameDecode(FrameDecodeEvent),
    
    /// Statistical summary event
    Statistics(StatisticsEvent),
    
    /// General info message
    Info { message: String },
    
    /// Warning message
    Warn { message: String },
    
    /// Error message
    Error { message: String, details: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub timestamp: DateTime<Utc>,
    pub pre_fec_ber: f64,
    pub post_fec_ber: f64,
    pub fsk_frequency_hz: f64,
    pub symbols_decoded: usize,
    pub frames_decoded: usize,
    pub sync_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameDecodeEvent {
    pub timestamp: DateTime<Utc>,
    pub frame_index: usize,
    pub hex_dump: FrameHexDump,
    pub decoded: DecodedFrame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameHexDump {
    pub sync_sequence: String,
    pub target_id: String,
    pub command_type: String,
    pub current_frame: u32,
    pub total_frames: u32,
    pub payload: String,
    pub ecc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedFrame {
    pub target_name: String,
    pub command_opcode: String,
    pub command_description: String,
    pub frame_position: String,
    pub payload_preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsEvent {
    pub timestamp: DateTime<Utc>,
    pub duration_secs: f64,
    pub pre_fec_ber: StatSummary,
    pub post_fec_ber: StatSummary,
    pub fsk_frequency_hz: StatSummary,
    pub total_frames: usize,
    pub total_symbols: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatSummary {
    pub mean: f64,
    pub stddev: f64,
    pub min: f64,
    pub max: f64,
    pub samples: usize,
}

/// Logger that outputs structured logs
pub struct StructuredLogger {
    config: LoggingConfig,
    writer: Box<dyn Write + Send>,
    start_time: DateTime<Utc>,
}

impl StructuredLogger {
    pub fn new(config: LoggingConfig) -> Result<Self> {
        let writer: Box<dyn Write + Send> = match &config.target {
            LogTarget::Stdout => Box::new(io::stdout()),
            LogTarget::Stderr => Box::new(io::stderr()),
            LogTarget::File => {
                let path = config.file_path.as_ref()
                    .ok_or_else(|| color_eyre::eyre::eyre!("File path required for file logging"))?;
                Box::new(BufWriter::new(File::create(path)?))
            }
        };
        
        Ok(Self {
            config,
            writer,
            start_time: Utc::now(),
        })
    }
    
    pub fn log(&mut self, event: LogEvent) -> Result<()> {
        let output = match self.config.format {
            LogFormat::Json => self.format_json(&event)?,
            LogFormat::Logfmt => self.format_logfmt(&event)?,
            LogFormat::Pretty => self.format_pretty(&event)?,
        };
        
        writeln!(self.writer, "{}", output)?;
        self.writer.flush()?;
        
        Ok(())
    }
    
    fn format_json(&self, event: &LogEvent) -> Result<String> {
        Ok(serde_json::to_string(event)?)
    }
    
    fn format_logfmt(&self, event: &LogEvent) -> Result<String> {
        let ts = Utc::now().to_rfc3339();
        
        let msg = match event {
            LogEvent::Telemetry(t) => {
                format!(
                    "ts=\"{}\" type=telemetry pre_fec_ber={:.6} post_fec_ber={:.6} fsk_freq_hz={:.2} symbols={} frames={} sync_locked={}",
                    ts, t.pre_fec_ber, t.post_fec_ber, t.fsk_frequency_hz, t.symbols_decoded, t.frames_decoded, t.sync_locked
                )
            }
            LogEvent::FrameDecode(f) => {
                format!(
                    "ts=\"{}\" type=frame_decode frame={} target=\"{}\" opcode=\"{}\" position=\"{}\"",
                    ts, f.frame_index, f.decoded.target_name, f.decoded.command_opcode, f.decoded.frame_position
                )
            }
            LogEvent::Statistics(s) => {
                format!(
                    "ts=\"{}\" type=statistics duration_secs={:.2} total_frames={} total_symbols={} pre_fec_ber_mean={:.6} post_fec_ber_mean={:.6}",
                    ts, s.duration_secs, s.total_frames, s.total_symbols, s.pre_fec_ber.mean, s.post_fec_ber.mean
                )
            }
            LogEvent::Info { message } => format!("ts=\"{}\" level=info msg=\"{}\"", ts, message),
            LogEvent::Warn { message } => format!("ts=\"{}\" level=warn msg=\"{}\"", ts, message),
            LogEvent::Error { message, details } => {
                if let Some(d) = details {
                    format!("ts=\"{}\" level=error msg=\"{}\" details=\"{}\"", ts, message, d)
                } else {
                    format!("ts=\"{}\" level=error msg=\"{}\"", ts, message)
                }
            }
        };
        
        Ok(msg)
    }
    
    fn format_pretty(&self, event: &LogEvent) -> Result<String> {
        let ts = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
        
        let msg = match event {
            LogEvent::Telemetry(t) => {
                format!(
                    "[{}] TELEMETRY: BER pre={:.6} post={:.6} | FSK={:.2}Hz | Symbols={} Frames={} | Sync={}",
                    ts, t.pre_fec_ber, t.post_fec_ber, t.fsk_frequency_hz, t.symbols_decoded, t.frames_decoded, 
                    if t.sync_locked { "LOCKED" } else { "SEARCHING" }
                )
            }
            LogEvent::FrameDecode(f) => {
                format!(
                    "[{}] FRAME[{}]: {} | {} ({}) | {}",
                    ts, f.frame_index, f.decoded.target_name, f.decoded.command_opcode, 
                    f.decoded.command_description, f.decoded.frame_position
                )
            }
            LogEvent::Statistics(s) => {
                format!(
                    "[{}] STATISTICS ({:.2}s): Frames={} Symbols={} | Pre-FEC BER: μ={:.6} σ={:.6} [{:.6},{:.6}] | Post-FEC BER: μ={:.6} σ={:.6} [{:.6},{:.6}]",
                    ts, s.duration_secs, s.total_frames, s.total_symbols,
                    s.pre_fec_ber.mean, s.pre_fec_ber.stddev, s.pre_fec_ber.min, s.pre_fec_ber.max,
                    s.post_fec_ber.mean, s.post_fec_ber.stddev, s.post_fec_ber.min, s.post_fec_ber.max
                )
            }
            LogEvent::Info { message } => format!("[{}] INFO: {}", ts, message),
            LogEvent::Warn { message } => format!("[{}] WARN: {}", ts, message),
            LogEvent::Error { message, details } => {
                if let Some(d) = details {
                    format!("[{}] ERROR: {} | {}", ts, message, d)
                } else {
                    format!("[{}] ERROR: {}", ts, message)
                }
            }
        };
        
        Ok(msg)
    }
    
    pub fn elapsed(&self) -> f64 {
        (Utc::now() - self.start_time).num_milliseconds() as f64 / 1000.0
    }
}
