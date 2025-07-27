// src/main.rs
/*
 * Main executable for NexusAI
 */

use clap::Parser;
use nexusai::{Result, run};

#[derive(Parser)]
#[command(version, about = "NexusAI - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
