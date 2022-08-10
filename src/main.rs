mod aptos;
mod tinc;
mod utils;

use crate::tinc::{run_tinc_command, TincCommand};
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Machikado Network Tinc Commands
    Tinc {
        #[clap(subcommand)]
        subcommand: TincCommand,
    },
}

/// A CLI for Machikado Network
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommand::Tinc { subcommand } => run_tinc_command(subcommand),
    }
}
