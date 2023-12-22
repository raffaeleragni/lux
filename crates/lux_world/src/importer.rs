use bevy::{prelude::*, utils::Uuid};
use bevy_sync::{SyncDown, SyncMark};

pub(crate) fn init(app: &mut App) {
    app.add_systems(Update, (propagate, handle_mesh, handle_material));
    app.add_systems(Update, cleanup);
    app.add_systems(Update, cleanup_mesh);
    app.add_systems(Update, cleanup_material);
}

pub(crate) fn import(file_name: &str, commands: &mut Commands, assets: &AssetServer) {
    let scene = assets.load(file_name.to_owned() + "#Scene0");
    println!("{:?}", scene);
    commands.spawn((
        SceneBundle {
            scene,
            ..Default::default()
        },
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

fn propagate(query: Query<(Entity, &Children), With<LoadedSceneItem>>, mut commands: Commands) {
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

fn cleanup(query: Query<Entity, (With<LoadedSceneItem>, With<SyncDown>)>, mut commands: Commands) {
    for e in query.iter() {
        commands.get_entity(e).unwrap().remove::<LoadedSceneItem>();
    }
}

fn cleanup_mesh(
    query_handle_mesh: Query<Entity, (With<LoadedSceneItemHandleMesh>, Without<Handle<Mesh>>)>,
    mut commands: Commands,
) {
    for e in query_handle_mesh.iter() {
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItemHandleMesh>();
    }
}

fn cleanup_material(
    query_handle_material: Query<
        Entity,
        (
            With<LoadedSceneItemHandleMaterial>,
            Without<Handle<StandardMaterial>>,
        ),
    >,
    mut commands: Commands,
) {
    for e in query_handle_material.iter() {
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItemHandleMaterial>();
    }
}

fn handle_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Handle<Mesh>), With<LoadedSceneItemHandleMesh>>,
) {
    for (e, h) in query.iter() {
        let id = AssetId::Uuid { uuid: Uuid::new_v4() };
        let asset = meshes.get(h.id()).unwrap();
        let asset = (*asset).clone();
        meshes.insert(id, asset);
        debug!("Reassigned mesh to uuid {:?}", id);
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItemHandleMesh>()
            .remove::<Handle<Mesh>>()
            .insert(Handle::Weak(id));
    }
}

fn handle_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Handle<StandardMaterial>), With<LoadedSceneItemHandleMaterial>>,
) {
    for (e, h) in query.iter() {
        let id = AssetId::Uuid { uuid: Uuid::new_v4() };
        let asset = materials.get(h.id()).unwrap();
        let asset = (*asset).clone();
        materials.insert(id, asset);
        debug!("Reassigned material to uuid {:?}", id);
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItemHandleMaterial>()
            .remove::<Handle<Mesh>>()
            .insert(Handle::Weak(id));
    }
}
