use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    render::{
        mesh::{morph::MeshMorphWeights, skinning::SkinnedMesh},
        primitives::Aabb,
    },
};
use bevy_sync::prelude::*;
use lux_cli::{Args, Command};
use std::net::{IpAddr, Ipv6Addr};

static SYNC_PORT: u16 = 4001;
static WEB_PORT: u16 = 4002;

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
    app.sync_component::<SpotLight>();
    app.sync_component::<DirectionalLight>();
    app.sync_component::<MeshMorphWeights>();
    app.sync_component::<SkinnedMesh>();
    app.sync_component::<Handle<StandardMaterial>>();
    app.sync_component::<Handle<Mesh>>();
    app.sync_component::<Handle<Image>>();
    app.sync_materials(true);
    app.sync_meshes(true);

    let localhost = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    match &args.command {
        Some(Command::Host {
            world_file: _,
            headless: _,
            ip,
            avatar_file: _,
        }) => app.add_plugins(ServerPlugin {
            parameters: SyncConnectionParameters::Socket {
                ip: ip.unwrap_or(localhost),
                port: SYNC_PORT,
                web_port: WEB_PORT,
                max_transfer: 1_000_000_000,
            },
        }),
        Some(Command::Join { ip, avatar_file: _ }) => app.add_plugins(ClientPlugin {
            parameters: SyncConnectionParameters::Socket {
                ip: ip.clone().to_owned(),
                port: SYNC_PORT,
                web_port: WEB_PORT + 1,
                max_transfer: 1_000_000_000,
            },
        }),
        _ => app,
    };
}
