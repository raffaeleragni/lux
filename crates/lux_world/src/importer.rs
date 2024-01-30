use bevy::{prelude::*, scene::SceneInstance, utils::Uuid};
use bevy_sync::{SyncDown, SyncMark};

pub(crate) fn init(app: &mut App) {
    app.add_systems(Update, (propagate, cleanup).chain());
    app.add_systems(Update, (handle_mesh, cleanup_mesh).chain());
    app.add_systems(Update, (handle_material, cleanup_material).chain());
}

pub(crate) fn import(file_name: &str, commands: &mut Commands, assets: &AssetServer) {
    let scene = assets.load(file_name.to_owned() + "#Scene0");
    debug!("Loading SceneBundle: {:?}", scene);
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
        debug!("Propagating entity {:?}", e);
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItem>()
            .remove::<SceneInstance>()
            .insert(SyncMark);
        for c in childs {
            debug!("Propagating entity {:?} children", e);
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
        debug!("Cleanup of entity {:?}", e);
        commands.get_entity(e).unwrap().remove::<LoadedSceneItem>();
    }
}

fn cleanup_mesh(
    query_handle_mesh: Query<Entity, (Added<LoadedSceneItemHandleMesh>, Without<Handle<Mesh>>)>,
    mut commands: Commands,
) {
    for e in query_handle_mesh.iter() {
        debug!("Cleaning up LoadedSceneItemHandleMesh from entity {:?}", e);
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
            Added<LoadedSceneItemHandleMaterial>,
            Without<Handle<StandardMaterial>>,
        ),
    >,
    mut commands: Commands,
) {
    for e in query_handle_material.iter() {
        debug!(
            "Cleaning up LoadedSceneItemHandleMaterial from entity {:?}",
            e
        );
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItemHandleMaterial>();
    }
}

fn handle_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Handle<Mesh>), Added<LoadedSceneItemHandleMesh>>,
) {
    for (e, h) in query.iter() {
        let id = AssetId::Uuid {
            uuid: Uuid::new_v4(),
        };
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
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &Handle<StandardMaterial>), Added<LoadedSceneItemHandleMaterial>>,
) {
    for (e, h) in query.iter() {
        let id = AssetId::Uuid {
            uuid: Uuid::new_v4(),
        };
        let asset = materials.get_mut(h.id()).unwrap();
        handle_images(images.as_mut(), asset);
        let asset = (*asset).clone();
        materials.insert(id, asset);
        debug!("Reassigned material to uuid {:?}", id);
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItemHandleMaterial>()
            .remove::<Handle<StandardMaterial>>()
            .insert(Handle::Weak(id));
    }
}

fn handle_images(images: &mut Assets<Image>, material: &mut StandardMaterial) {
    macro_rules! swap_image {
        ($image:expr) => {
            if let Some(h) = $image.clone() {
                let image = images.get(h.id()).unwrap();
                let image = (*image).clone();
                let id = AssetId::Uuid {
                    uuid: Uuid::new_v4(),
                };
                images.insert(id, image);
                $image = Some(Handle::Weak(id));
                debug!("Reassigned image to uuid {:?}", id);
            }
        };
    }
    swap_image!(material.base_color_texture);
    swap_image!(material.emissive_texture);
    swap_image!(material.normal_map_texture);
    swap_image!(material.occlusion_texture);
    swap_image!(material.metallic_roughness_texture);
}
