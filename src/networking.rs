use std::net::{IpAddr, Ipv4Addr};

use bevy::{prelude::*, render::primitives::Aabb, pbr::wireframe::Wireframe};
use bevy_sync::{SyncComponent, SyncPlugin, ClientPlugin, ServerPlugin};

use crate::cli::{Args, Command};

pub fn init(args: Args, app: &mut App) {
    setup_sync(args, app);
}

fn setup_sync(args: Args, app: &mut App) {
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

    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 4000;
    match &args.command {
        Some(Command::Host {
            world_file: _,
            headless: _,
        }) => app.add_plugins(ServerPlugin {
            ip: localhost,
            port,
        }),
        Some(Command::Join { ip }) => app.add_plugins(ClientPlugin {
            ip: ip.clone().to_owned(),
            port,
        }),
        _ => app,
    };
}
