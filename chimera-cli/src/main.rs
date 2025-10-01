use clap::Parser;
use color_eyre::eyre::Result;

use chimera_core::config::{LDPCConfig, ProtocolConfig, SimulationConfig};
use chimera_core::run_simulation;

#[derive(Parser, Debug)]
#[command(author, version, about = "Chimera modulation pipeline simulator", long_about = None)]
struct Cli {
    /// Path to a JSON or TOML configuration file overriding defaults
    #[arg(short, long)]
    config: Option<String>,

    /// Enable verbose diagnostic output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    let sim = SimulationConfig::default();
    let protocol = ProtocolConfig::default();
    let ldpc = LDPCConfig::default();

    let result = run_simulation(&sim, &protocol, &ldpc);

    if args.verbose {
        println!("Diagnostics: {:?}", result.diagnostics);
    }

    println!("Recovered message: {}", result.report.recovered_message);
    println!("Pre-FEC BER : {:.6}", result.report.pre_fec_ber);
    println!("Post-FEC BER: {:.6}", result.report.post_fec_ber);

    Ok(())
}
