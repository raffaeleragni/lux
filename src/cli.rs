use std::net::Ipv4Addr;

use bevy::prelude::Resource;
use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug, Resource)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(name = "host")]
    Host {
        world_file: String,
        #[clap(long, short, action, default_value_t = false)]
        headless: bool,
    },
    #[clap(name = "join")]
    Join { ip: Ipv4Addr },
}
