mod cli;
mod initial;
mod layouts;
mod menu;
mod networking;
mod xr;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use clap::Parser;
use cli::{Args, Command};
use std::time::Duration;

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

    networking::init(args, &mut app);

    components::register(&mut app);
    initial::init(&mut app);
    app.run();
}
