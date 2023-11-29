mod cli;
mod desktop;
mod networking;
mod world;
mod xr;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use clap::Parser;
use cli::{Args, Command};
use std::time::Duration;

fn main() {
    let mut app = App::new();
    let args = Args::parse();
    app.insert_resource(args.clone());

    main_init(&args, &mut app);
    components::init(&mut app);
    networking::init(&args, &mut app);
    world::init(&mut app);

    app.run();
}

fn main_init(args: &Args, app: &mut App) {
    let headless = match args.command {
        Some(Command::Host {
            world_file: _,
            headless,
        }) => headless,
        _ => false,
    };
    if headless {
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )));
    } else {
        if args.xr_enabled {
            xr::init(app);
        } else {
            app.add_plugins(DefaultPlugins);
        }
        desktop::init(app);
    }
}
