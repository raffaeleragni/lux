mod empty_world;
mod importer;

use bevy::prelude::*;
use empty_world::spawn_empty_world;
use lux_cli::{Args, Command};

pub fn init(app: &mut App) {
    app.add_systems(
        Startup,
        load_world_from_args.run_if(resource_exists::<Args>()),
    );

    importer::init(app);
}

fn load_world_from_args(
    args: Res<Args>,
    assets: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    match &args.command {
        Some(Command::Host {
            world_file,
            headless: _,
        }) => importer::import_file(world_file, &mut commands, &assets),
        Some(Command::Join { ip: _ }) => (),
        _ => spawn_empty_world(meshes, materials, commands),
    }
}
