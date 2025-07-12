// src/main.rs
/*
 * Main executable for MarketSentinel
 */

use clap::Parser;
use marketsentinel::{Result, run};

#[derive(Parser)]
#[command(version, about = "MarketSentinel - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
