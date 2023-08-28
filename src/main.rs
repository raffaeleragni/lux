mod cli;
mod menu;

use bevy::prelude::*;
use clap::Parser;
use cli::{Args, Command};

fn main() {
    let mut app = App::new();
    components::register(&mut app);
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.insert_resource(Args::parse());
    app.add_plugins(menu::MenuPlugin);
    app.add_systems(Startup, load_world.run_if(resource_exists::<Args>()));
    app.run();
}

fn load_world(args: Res<Args>, ass: Res<AssetServer>, mut commands: Commands) {
    match &args.command {
        Command::Join { ip: _ } => todo!(),
        Command::Host { world_file } => {
            let loaded = ass.load(world_file.to_owned() + "#Scene0");
            commands.spawn(SceneBundle {
                scene: loaded,
                ..Default::default()
            });
        }
    }
}
