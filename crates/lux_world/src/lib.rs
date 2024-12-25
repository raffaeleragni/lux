mod empty_world;
mod importer;

use bevy::prelude::*;
use empty_world::spawn_empty_world;
use importer::AvatarLaterImport;
use lux_cli::{Args, Command};

pub use importer::import_audio;
pub use importer::import_gltf;

pub fn init(app: &mut App) {
    app.add_systems(
        Startup,
        load_world_from_args.run_if(resource_exists::<Args>),
    );
    app.add_systems(
        Startup,
        load_avatar_from_args.run_if(resource_exists::<Args>),
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
            ip: _,
            avatar_file: _,
        }) => importer::import_gltf(world_file, &mut commands, &assets),
        Some(Command::Join {
            ip: _,
            avatar_file: _,
        }) => (),
        _ => spawn_empty_world(meshes, materials, commands),
    }
}

fn load_avatar_from_args(args: Res<Args>, mut commands: Commands) {
    let avatar_file = match &args.command {
        Some(Command::Host {
            world_file: _,
            headless,
            ip: _,
            avatar_file,
        }) => {
            if *headless {
                return;
            }
            avatar_file
        }
        Some(Command::Join { ip: _, avatar_file }) => avatar_file,
        _ => &None,
    }
    .to_owned();
    if let Some(avatar_file) = avatar_file {
        commands.spawn(AvatarLaterImport {
            file_name: avatar_file.to_string(),
        });
    }
}
