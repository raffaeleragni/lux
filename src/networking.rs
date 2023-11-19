use std::net::{IpAddr, Ipv4Addr};

use bevy::{pbr::wireframe::Wireframe, prelude::*, render::primitives::Aabb};
use bevy_sync::{ClientPlugin, ServerPlugin, SyncComponent, SyncPlugin};

use crate::cli::{Args, Command};

static SYNC_PORT: u16 = 4001;

pub fn init(args: &Args, app: &mut App) {
    setup_sync(args, app);
}

fn setup_sync(args: &Args, app: &mut App) {
    if args.command.is_none() {
        return;
    }
    app.add_plugins(SyncPlugin);
    app.sync_component::<Name>();
    app.sync_component::<Aabb>();
    app.sync_component::<Visibility>();
    app.sync_component::<Transform>();
    app.sync_component::<Wireframe>();
    app.sync_component::<PointLight>();
    app.sync_component::<Handle<StandardMaterial>>();
    app.sync_component::<Handle<Mesh>>();
    app.sync_materials(true);
    app.sync_meshes(true);

    let localhost = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    match &args.command {
        Some(Command::Host {
            world_file: _,
            headless: _,
        }) => app.add_plugins(ServerPlugin {
            ip: localhost,
            port: SYNC_PORT,
        }),
        Some(Command::Join { ip }) => app.add_plugins(ClientPlugin {
            ip: ip.clone().to_owned(),
            port: SYNC_PORT,
        }),
        _ => app,
    };
}
