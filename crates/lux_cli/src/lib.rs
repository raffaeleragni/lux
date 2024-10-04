use std::net::IpAddr;

use bevy::prelude::Resource;
use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug, Resource)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable XR/VR
    #[clap(name = "xr", long, default_value_t = false)]
    pub xr_enabled: bool,
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(name = "host")]
    Host {
        world_file: String,
        #[clap(long, default_value_t = false)]
        headless: bool,
        ip: Option<IpAddr>,
        /// Path to the avatar file.
        /// Supports: .vrm or .glb/.gltf avatars.
        #[clap(name = "avatar", long)]
        avatar_file: Option<String>,
    },
    #[clap(name = "join")]
    Join {
        ip: IpAddr,
        /// Path to the avatar file.
        /// Supports: .vrm or .glb/.gltf avatars.
        #[clap(name = "avatar", long)]
        avatar_file: Option<String>,
    },
}
