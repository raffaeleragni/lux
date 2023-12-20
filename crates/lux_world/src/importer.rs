use bevy::prelude::*;
use bevy_sync::{SyncDown, SyncMark};

pub(crate) fn init(app: &mut App) {
    app.add_systems(
        Update,
        (
            propagate,
            cleanup,
            handle_mesh,
            handle_material,
        ),
    );
}

pub(crate) fn import_file(world_file: &String, commands: &mut Commands, assets: &AssetServer) {
    let scene = assets.load(world_file.to_owned() + "#Scene0");
    commands
        .spawn((SceneBundle {
            scene,
            ..Default::default()
        },))
        .insert((
            LoadedSceneItem,
            LoadedSceneItemHandleMesh,
            LoadedSceneItemHandleMaterial,
        ));
}

#[derive(Component)]
struct LoadedSceneItem;

#[derive(Component)]
struct LoadedSceneItemHandleMesh;

#[derive(Component)]
struct LoadedSceneItemHandleMaterial;

fn propagate(
    query: Query<(Entity, &Children), With<LoadedSceneItem>>,
    mut commands: Commands,
) {
    for (e, childs) in query.iter() {
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItem>()
            .insert(SyncMark);
        for c in childs {
            commands
                .get_entity(*c)
                .unwrap()
                .insert(LoadedSceneItem)
                .insert(LoadedSceneItemHandleMesh)
                .insert(LoadedSceneItemHandleMaterial)
                .insert(SyncMark);
        }
    }
}

fn cleanup(
    query: Query<Entity, (With<LoadedSceneItem>, With<SyncDown>)>,
    mut commands: Commands,
) {
    for e in query.iter() {
        commands.get_entity(e).unwrap().remove::<LoadedSceneItem>();
    }
}

fn handle_mesh() {}

fn handle_material() {}
