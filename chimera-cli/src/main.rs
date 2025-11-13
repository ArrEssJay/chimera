mod config;
mod frame_decoder;
mod logging;
mod telemetry;

use clap::Parser;
use color_eyre::eyre::{Context, Result};

use chimera_core::processor::{ChimeraProcessor, ProcessorConfig};
use config::CliConfig;
use logging::{LogEvent, StatisticsEvent, StructuredLogger};
use std::path::PathBuf;
use telemetry::TelemetryAggregator;

#[derive(Parser, Debug)]
#[command(author, version, about = "Chimera modulation pipeline simulator", long_about = None)]
struct Cli {
    /// Path to TOML configuration file (or preset name: raman-whisper, burst-telemetry, deep-space-probe)
    #[arg(short, long)]
    config: Option<String>,

    /// Enable verbose diagnostic output
    #[arg(short, long)]
    verbose: bool,
    
    /// Output audio to WAV file (overrides config)
    #[arg(short = 'w', long)]
    wav_output: Option<PathBuf>,
    
    /// Message to encode (overrides config message)
    #[arg(short = 'm', long)]
    message: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    // Load configuration from file
    let mut config = if let Some(config_path) = &args.config {
        CliConfig::from_file(config_path)
            .wrap_err_with(|| format!("Failed to load config from {}", config_path))?
    } else {
        eprintln!("Error: --config parameter is required. Example: --config configs/long_message.toml");
        std::process::exit(1);
    };
    
    // Override with CLI arguments
    if let Some(wav_path) = args.wav_output {
        config.terminal.wav_output = Some(wav_path);
    }
    
    if let Some(message) = args.message {
        config.simulation.message = message;
    }
    
    // Initialize structured logger
    let mut logger = StructuredLogger::new(config.terminal.logging.clone())?;
    
    logger.log(LogEvent::Info {
        message: format!("Chimera CLI starting with message: \"{}\"", config.simulation.message),
    })?;
    
    // SINGLE FRAME MODE: Calculate max message size
    let ldpc = chimera_core::ldpc::LDPCSuite::new(&config.protocol.frame_layout, &config.ldpc);
    let max_bytes = ldpc.matrices.message_bits / 8;
    let message_bytes = config.simulation.message.len().min(max_bytes);
    let payload_bits = chimera_core::utils::string_to_bitstream(&config.simulation.message[..message_bytes]);
    
    logger.log(LogEvent::Info {
        message: format!(
            "Encoding {} bytes ({} bits) into 1 frame (max {} bytes)",
            message_bytes,
            payload_bits.len(),
            max_bytes
        ),
    })?;
    
    // Create processor with configuration from config file
    let processor_config = ProcessorConfig {
        sample_rate: chimera_core::config::SystemConfig::SAMPLE_RATE,
        symbol_rate: config.protocol.qpsk_symbol_rate,
        carrier_freq: config.protocol.carrier_freq_hz,
        logging: config.terminal.logging.to_core_log_config(),
        optimize_for_latency: false, // Batch mode
        min_chunk_size: None,
    };
    
    let mut processor = ChimeraProcessor::new(processor_config);
    if args.verbose {
        processor.enable_diagnostics();
    }
    
    // Run the batch processing
    let result = processor.process_batch(&config.simulation.message);
    
    // Initialize telemetry aggregator
    let mut telemetry = TelemetryAggregator::new(config.terminal.telemetry_interval_secs);
    
    // Update telemetry with actual simulation results
    telemetry.update(
        result.pre_fec_ber,
        result.post_fec_ber,
        config.protocol.carrier_freq_hz,
        result.tx_symbols.len(),
        1, // Single frame mode
        result.post_fec_errors == 0, // Synced if no errors
    );
    
    // Log telemetry sample
    if let Some(telemetry_event) = telemetry.sample() {
        logger.log(LogEvent::Telemetry(telemetry_event))?;
    }
    
    // Decode and log frames (processor doesn't expose frame-by-frame data yet)
    // TODO: Add frame diagnostics to processor if needed
    // For now, we'll skip frame-by-frame logging
    
    // Compute and log final statistics
    let (pre_fec_stats, post_fec_stats, fsk_stats) = telemetry.compute_statistics();
    
    let stats_event = StatisticsEvent {
        timestamp: chrono::Utc::now(),
        duration_secs: logger.elapsed(),
        pre_fec_ber: pre_fec_stats,
        post_fec_ber: post_fec_stats,
        fsk_frequency_hz: fsk_stats,
        total_frames: telemetry.total_frames(),
        total_symbols: telemetry.total_symbols(),
    };
    
    logger.log(LogEvent::Statistics(stats_event))?;
    
    // Log summary metrics
    let errors_corrected = if result.pre_fec_errors > result.post_fec_errors {
        result.pre_fec_errors - result.post_fec_errors
    } else {
        0
    };
    
    logger.log(LogEvent::Info {
        message: format!(
            "Pre-FEC BER: {:.6}, Post-FEC BER: {:.6}, Errors corrected: {}",
            result.pre_fec_ber,
            result.post_fec_ber,
            errors_corrected
        ),
    })?;
    
    logger.log(LogEvent::Info {
        message: format!("Recovered message: {}", result.recovered_message),
    })?;
    
    // If WAV output is requested, write the audio from the simulation
    if let Some(wav_path) = &config.terminal.wav_output {
        if !result.audio.is_empty() {
            logger.log(LogEvent::Info {
                message: format!("Writing audio to {}", wav_path.display()),
            })?;
            
            let sample_rate = chimera_core::config::SystemConfig::SAMPLE_RATE;
            
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: sample_rate as u32,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            
            let mut writer = hound::WavWriter::create(&wav_path, spec)?;
            
            for &sample in &result.audio {
                writer.write_sample(sample)?;
            }
            writer.finalize()?;
            
            logger.log(LogEvent::Info {
                message: format!(
                    "Wrote {} samples ({:.2}s) to {}",
                    result.audio.len(),
                    result.audio.len() as f64 / sample_rate as f64,
                    wav_path.display()
                ),
            })?;
        } else {
            logger.log(LogEvent::Info {
                message: "Warning: No audio data available to write".to_string(),
            })?;
        }
    }
    
    // Only log detailed diagnostics if verbose flag is set
    if args.verbose {
        logger.log(LogEvent::Info {
            message: format!(
                "Diagnostic summary: {} TX symbols, {} RX symbols, {} audio samples",
                result.tx_symbols.len(),
                result.rx_symbols.len(),
                result.audio.len()
            ),
        })?;
        
        logger.log(LogEvent::Info {
            message: format!(
                "Audio: {} samples at {} Hz, carrier at {:.1} Hz, SNR: {:.1} dB",
                result.audio.len(),
                chimera_core::config::SystemConfig::SAMPLE_RATE,
                config.protocol.carrier_freq_hz,
                result.snr_db
            ),
        })?;
    }

    Ok(())
}
