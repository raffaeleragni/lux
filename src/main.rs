mod cli;
mod layouts;
mod menu;

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use clap::Parser;
use cli::{Args, Command};

fn main() {
    let mut app = App::new();
    let args = Args::parse();
    let headless = match args.command {
        Command::Host {
            world_file: _,
            headless,
        } => headless,
        _ => false,
    };
    app.insert_resource(args);

    if headless {
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )));
    } else {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(bevy_editor_pls::EditorPlugin::default());
        app.add_plugins(menu::MenuPlugin);
        layouts::setup(&mut app);
    }

    components::register(&mut app);
    app.add_systems(
        Startup,
        load_world_from_args.run_if(resource_exists::<Args>()),
    );
    app.run();
}

fn load_world_from_args(args: Res<Args>, ass: Res<AssetServer>, mut commands: Commands) {
    match &args.command {
        Command::Join { ip: _ } => todo!(),
        Command::Host {
            world_file,
            headless: _,
        } => {
            let loaded = ass.load(world_file.to_owned() + "#Scene0");
            commands.spawn(SceneBundle {
                scene: loaded,
                ..Default::default()
            });
        }
    }
}
