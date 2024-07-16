use std::{marker::PhantomData, net::{IpAddr, Ipv6Addr}};

use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    render::{
        mesh::{morph::MeshMorphWeights, skinning::SkinnedMesh},
        primitives::Aabb,
    },
};
use bevy_sync::{ClientPlugin, ServerPlugin, SyncComponent, SyncPlugin};

use lux_cli::{Args, Command};

static SYNC_PORT: u16 = 4001;
static WEB_PORT: u16 = 4002;

pub fn init(args: &Args, app: &mut App) {
    setup_sync(args, app);
}

/// This component marks that a component is controlled by another component
/// This will allow to span or control how many controlling components there
/// are and if they are zero or 1+, to delete or sidespawn a SyncExclude<C>.
/// This will not be synched as it is a local only blocker for sending data.
#[derive(Component, Default)]
pub struct ControlledBy<C: Component, F: Component> {
    c: PhantomData<C>,
    f: PhantomData<F>
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
        }) => app.add_plugins(ServerPlugin {
            ip: ip.unwrap_or(localhost),
            port: SYNC_PORT,
            web_port: WEB_PORT,
            max_transfer: 1_000_000_000,
        }),
        Some(Command::Join { ip }) => app.add_plugins(ClientPlugin {
            ip: ip.clone().to_owned(),
            port: SYNC_PORT,
            web_port: WEB_PORT + 1,
            max_transfer: 1_000_000_000,
        }),
        _ => app,
    };
}
