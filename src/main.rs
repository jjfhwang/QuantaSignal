// src/main.rs
/*
 * Main executable for QuantaSignal
 */

use clap::Parser;
use quantasignal::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantaSignal - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
