use bevy::prelude::*;
use clap::Parser;
use lux_cli::{Args, Command};

pub fn app() -> App {
    let mut app = App::new();
    let args = Args::parse();
    app.insert_resource(args.clone());

    base_init(&args, &mut app);
    lux_networking::init(&args, &mut app);
    lux_components::init(&mut app);
    lux_world::init(&mut app);

    app
}

fn base_init(args: &Args, app: &mut App) {
    let headless = match args.command {
        Some(Command::Host {
            world_file: _,
            headless,
            ip: _,
        }) => headless,
        _ => false,
    };
    if headless {
        lux_headless::init(app);
    } else if args.xr_enabled {
        cfg_if::cfg_if! {
            if #[cfg(feature="xr")] {
                lux_xr::init(app);
                lux_desktop::init(app);
            } else {
                eprintln!("XR feature is not compiled in.");
                std::process::exit(1);
            }
        }
    } else {
        app.add_plugins(DefaultPlugins);
        lux_desktop::init(app);
    }
}
