mod cli;
mod initial;
mod layouts;
mod menu;
mod xr;

use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use bevy::{
    app::ScheduleRunnerPlugin, pbr::wireframe::Wireframe, prelude::*, render::primitives::Aabb,
};
use bevy_sync::prelude::*;
use clap::Parser;
use cli::{Args, Command};

fn main() {
    let mut app = App::new();
    let args = Args::parse();
    let headless = match args.command {
        Some(Command::Host {
            world_file: _,
            headless,
        }) => headless,
        _ => false,
    };

    app.insert_resource(args.clone());

    if headless {
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )));
    } else {
        if args.xr_enabled {
            xr::init(&mut app);
       } else {
            app.add_plugins(DefaultPlugins);
        }
        app.add_plugins(bevy_editor_pls::EditorPlugin::default());
        app.add_plugins(menu::MenuPlugin);

        layouts::setup(&mut app);
    }

    setup_sync(args.clone(), &mut app);

    components::register(&mut app);
    initial::init(&mut app);
    app.run();
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

