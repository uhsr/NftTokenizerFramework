// src/main.rs
/*
 * Main executable for NftTokenizerFramework
 */

use clap::Parser;
use nfttokenizerframework::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftTokenizerFramework - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
