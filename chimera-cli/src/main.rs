mod config;
mod frame_decoder;
mod logging;
mod telemetry;

use clap::Parser;
use color_eyre::eyre::{Context, Result};

use chimera_core::run_simulation;
use config::CliConfig;
use frame_decoder::FrameDecoder;
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
    
    // Calculate frame count for logging
    let payload_bits = chimera_core::utils::string_to_bitstream(&config.simulation.message);
    let bits_per_frame = config.protocol.frame_layout.data_payload_symbols * 2;
    let total_frames = (payload_bits.len() + bits_per_frame - 1) / bits_per_frame;
    
    logger.log(LogEvent::Info {
        message: format!(
            "Encoding {} bytes ({} bits) into {} frames",
            config.simulation.message.len(),
            payload_bits.len(),
            total_frames
        ),
    })?;
    
    // Always run the full simulation (which generates audio internally)
    let result = run_simulation(&config.simulation, &config.protocol, &config.ldpc);
    
    // Initialize telemetry aggregator
    let mut telemetry = TelemetryAggregator::new(config.terminal.telemetry_interval_secs);
    
    // Initialize frame decoder
    let frame_decoder = FrameDecoder::new(config.protocol.clone());
    
    // Update telemetry with actual simulation results
    telemetry.update(
        result.report.pre_fec_ber,
        result.report.post_fec_ber,
        config.protocol.carrier_freq_hz,
        result.diagnostics.tx_symbols_i.len(),
        total_frames,
        result.report.post_fec_errors == 0, // Synced if no errors
    );
    
    // Log telemetry sample
    if let Some(telemetry_event) = telemetry.sample() {
        logger.log(LogEvent::Telemetry(telemetry_event))?;
    }
    
    // Decode and log each frame if frame data is available
    if !result.diagnostics.frames.is_empty() {
        for (frame_idx, frame_desc) in result.diagnostics.frames.iter().enumerate() {
            let frame_event = frame_decoder.decode_frame(frame_idx, &result.diagnostics.tx_bits);
            logger.log(LogEvent::FrameDecode(frame_event))?;
            
            // Log frame metadata
            logger.log(LogEvent::Info {
                message: format!(
                    "Frame {}/{}: {} - {}",
                    frame_desc.frame_index + 1,
                    frame_desc.total_frames,
                    frame_desc.frame_label,
                    frame_desc.payload_preview
                ),
            })?;
        }
    }
    
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
    logger.log(LogEvent::Info {
        message: format!(
            "Pre-FEC BER: {:.6}, Post-FEC BER: {:.6}, Errors corrected: {}",
            result.report.pre_fec_ber,
            result.report.post_fec_ber,
            result.report.pre_fec_errors - result.report.post_fec_errors
        ),
    })?;
    
    logger.log(LogEvent::Info {
        message: format!("Recovered message: {}", result.report.recovered_message),
    })?;
    
    // If WAV output is requested, write the audio from the simulation
    if let Some(wav_path) = &config.terminal.wav_output {
        if let Some(audio_data) = &result.diagnostics.modulation_audio {
            logger.log(LogEvent::Info {
                message: format!("Writing audio to {}", wav_path.display()),
            })?;
            
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: audio_data.sample_rate as u32,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            
            let mut writer = hound::WavWriter::create(&wav_path, spec)?;
            
            // Write the noisy audio if available, otherwise clean
            let audio_samples = if !audio_data.noisy.is_empty() {
                &audio_data.noisy
            } else {
                &audio_data.clean
            };
            
            for &sample in audio_samples {
                writer.write_sample(sample)?;
            }
            writer.finalize()?;
            
            logger.log(LogEvent::Info {
                message: format!(
                    "Wrote {} samples ({:.2}s) to {}",
                    audio_samples.len(),
                    audio_samples.len() as f64 / audio_data.sample_rate as f64,
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
                "Diagnostic summary: {} TX symbols, {} encoding logs, {} decoding logs",
                result.diagnostics.tx_symbols_i.len(),
                result.diagnostics.encoding_logs.len(),
                result.diagnostics.decoding_logs.len()
            ),
        })?;
        
        if let Some(audio) = &result.diagnostics.modulation_audio {
            logger.log(LogEvent::Info {
                message: format!(
                    "Audio: {} samples at {} Hz, carrier at {:.1} Hz",
                    audio.clean.len(),
                    audio.sample_rate,
                    audio.carrier_freq_hz
                ),
            })?;
        }
    }

    Ok(())
}
