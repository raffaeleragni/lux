use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use clap::Parser;
use lux_cli::{Args, Command};
use std::time::Duration;

pub fn app() -> App {
    let mut app = App::new();
    let args = Args::parse();
    app.insert_resource(args.clone());

    base_init(&args, &mut app);
    lux_components::init(&mut app);
    lux_networking::init(&args, &mut app);
    lux_world::init(&mut app);

    app
}

fn base_init(args: &Args, app: &mut App) {
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
        let mut plugins_added = false;
        #[cfg(feature = "xr")]
        if args.xr_enabled {
            lux_xr::init(app);
            plugins_added = true;
        }
        if !plugins_added {
            app.add_plugins(DefaultPlugins);
        }
        lux_desktop::init(app);
    }
}