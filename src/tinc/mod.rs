mod setup;
mod update;

use crate::tinc::update::update_nodes;
use clap::Subcommand;
use setup::{setup_tinc, validate_ip_addr, validate_name};
use std::net::Ipv4Addr;

#[derive(Subcommand, Debug)]
pub enum TincCommand {
    /// Setup Tinc Node
    Setup {
        /// Tinc node name what you want. e.g. syamimomo
        #[clap(value_parser = validate_name)]
        name: String,
        #[clap(value_parser = validate_ip_addr)]
        ip_addr: Ipv4Addr,
    },
    Update {
        /// If the value is set to 1 or more, it runs as a daemon.
        #[clap(short = 'd', default_value_t = 0)]
        loop_sec: u64,
        /// Do not restart when updated nodes
        #[clap(long, action)]
        no_restart: bool,
    },
}

pub fn run_tinc_command(command: TincCommand) {
    match command {
        TincCommand::Setup { name, ip_addr } => setup_tinc(name, ip_addr),
        TincCommand::Update {
            loop_sec,
            no_restart,
        } => update_nodes(loop_sec, no_restart),
    }
}
