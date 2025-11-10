use clap::Parser;
use color_eyre::eyre::Result;

use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::{generate_audio_batch, run_simulation};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Chimera modulation pipeline simulator", long_about = None)]
struct Cli {
    /// Path to a JSON or TOML configuration file overriding defaults
    #[arg(short, long)]
    config: Option<String>,

    /// Enable verbose diagnostic output
    #[arg(short, long)]
    verbose: bool,
    
    /// Output audio to WAV file (for debugging)
    #[arg(short = 'w', long)]
    wav_output: Option<PathBuf>,
    
    /// Message to encode (overrides default config message)
    #[arg(short = 'm', long)]
    message: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();

    // If WAV output is requested, use batch audio generation
    if let Some(wav_path) = args.wav_output {
        let mut sim = sim;
        
        // Override message if provided
        if let Some(message) = args.message {
            sim.plaintext_source = message;
        }
        
        println!("Message: \"{}\"", sim.plaintext_source);
        println!("Generating audio...");
        
        let payload_bits = chimera_core::utils::string_to_bitstream(&sim.plaintext_source);
        let bits_per_frame = protocol.frame_layout.data_payload_symbols * 2; // 2 bits per QPSK symbol
        let total_frames = (payload_bits.len() + bits_per_frame - 1) / bits_per_frame;
        
        println!("Message: {} bytes = {} bits", 
            sim.plaintext_source.len(), 
            payload_bits.len()
        );
        println!("Frames: {} ({} symbols each @ {} sym/s)",
            total_frames,
            protocol.frame_layout.total_symbols,
            protocol.qpsk_symbol_rate
        );
        
        // Generate all audio at once using batch mode
        let audio = generate_audio_batch(&sim.plaintext_source, &protocol, &ldpc);
        
        println!("Generated {} audio samples", audio.len());
        
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
        
        println!("Wrote {} samples ({:.2}s) to {}", 
            audio.len(),
            audio.len() as f64 / SimulationConfig::SAMPLE_RATE as f64,
            wav_path.display()
        );
        
        return Ok(());
    }

    // Otherwise, run standard batch simulation
    let result = run_simulation(&sim, &protocol, &ldpc);

    if args.verbose {
        println!("Diagnostics: {:?}", result.diagnostics);
    }

    println!("Recovered message: {}", result.report.recovered_message);
    println!("Pre-FEC BER : {:.6}", result.report.pre_fec_ber);
    println!("Post-FEC BER: {:.6}", result.report.post_fec_ber);

    Ok(())
}
