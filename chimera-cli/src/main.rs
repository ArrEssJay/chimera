mod config;
mod frame_decoder;
mod logging;
mod telemetry;

use clap::Parser;
use color_eyre::eyre::{Context, Result};

use chimera_core::config::SimulationConfig;
use chimera_core::{generate_audio_batch, run_simulation};
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
        config.simulation.plaintext_source = message;
    }
    
    // Initialize structured logger
    let mut logger = StructuredLogger::new(config.terminal.logging.clone())?;
    
    logger.log(LogEvent::Info {
        message: format!("Chimera CLI starting with message: \"{}\"", config.simulation.plaintext_source),
    })?;
    
    // If WAV output is requested, use batch audio generation
    if let Some(wav_path) = &config.terminal.wav_output {
        logger.log(LogEvent::Info {
            message: format!("Generating audio to {}", wav_path.display()),
        })?;
        
        let payload_bits = chimera_core::utils::string_to_bitstream(&config.simulation.plaintext_source);
        let bits_per_frame = config.protocol.frame_layout.data_payload_symbols * 2;
        let total_frames = (payload_bits.len() + bits_per_frame - 1) / bits_per_frame;
        
        logger.log(LogEvent::Info {
            message: format!(
                "Encoding {} bytes ({} bits) into {} frames",
                config.simulation.plaintext_source.len(),
                payload_bits.len(),
                total_frames
            ),
        })?;
        
        // Generate all audio at once using batch mode
        let audio = generate_audio_batch(
            &config.simulation.plaintext_source,
            &config.protocol,
            &config.ldpc,
        );
        
        // Write to WAV file
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: SimulationConfig::SAMPLE_RATE as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        
        let mut writer = hound::WavWriter::create(&wav_path, spec)?;
        for &sample in &audio {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
        
        logger.log(LogEvent::Info {
            message: format!(
                "Wrote {} samples ({:.2}s) to {}",
                audio.len(),
                audio.len() as f64 / SimulationConfig::SAMPLE_RATE as f64,
                wav_path.display()
            ),
        })?;
        
        return Ok(());
    }

    // Otherwise, run standard batch simulation with telemetry
    logger.log(LogEvent::Info {
        message: "Starting simulation with telemetry logging".to_string(),
    })?;
    
    let result = run_simulation(&config.simulation, &config.protocol, &config.ldpc);
    
    // Initialize telemetry aggregator
    let mut telemetry = TelemetryAggregator::new(config.terminal.telemetry_interval_secs);
    
    // Initialize frame decoder
    let frame_decoder = FrameDecoder::new(config.protocol.clone());
    
    // Simulate telemetry sampling (in real streaming scenario, this would be periodic)
    telemetry.update(
        result.report.pre_fec_ber,
        result.report.post_fec_ber,
        12000.0, // FSK frequency placeholder
        result.diagnostics.demodulation.symbol_decisions.len(),
        result.diagnostics.frames.len(),
        true, // Assume synced after successful decode
    );
    
    // Log telemetry sample
    if let Some(telemetry_event) = telemetry.sample() {
        logger.log(LogEvent::Telemetry(telemetry_event))?;
    }
    
    // Decode and log each frame
    for (frame_idx, _frame_desc) in result.diagnostics.frames.iter().enumerate() {
        // Reconstruct frame bits for hex dump
        // In a real scenario, we'd have the actual frame bits from the decoder
        // For now, we create a minimal frame decode event
        let frame_event = frame_decoder.decode_frame(frame_idx, &result.diagnostics.tx_bits);
        logger.log(LogEvent::FrameDecode(frame_event))?;
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
    
    if args.verbose {
        logger.log(LogEvent::Info {
            message: format!("Diagnostics: {:?}", result.diagnostics),
        })?;
    }
    
    logger.log(LogEvent::Info {
        message: format!("Recovered message: {}", result.report.recovered_message),
    })?;

    Ok(())
}
