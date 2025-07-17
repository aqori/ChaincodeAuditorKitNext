// src/main.rs
/*
 * Main executable for ChaincodeAuditorKitNext
 */

use clap::Parser;
use chaincodeauditorkitnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChaincodeAuditorKitNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
