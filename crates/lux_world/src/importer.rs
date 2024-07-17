use bevy::{prelude::*, scene::SceneInstance};
use bevy_sync::{SyncEntity, SyncMark, Uuid};

pub(crate) fn init(app: &mut App) {
    app.add_systems(Update, (propagate, cleanup).chain());
    app.add_systems(Update, (handle_mesh, cleanup_mesh).chain());
    app.add_systems(Update, (handle_material, cleanup_material).chain());
    app.add_systems(Update, (handle_audio, cleanup_audio).chain());
}

pub fn import_gltf(file_name: &str, commands: &mut Commands, assets: &AssetServer) {
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

pub fn import_audio(file_name: &str, commands: &mut Commands, assets: &AssetServer) {
    commands.spawn((
        AudioBundle {
            source: assets.load(file_name.to_owned()),
            ..default()
        },
        LoadedAudioItem,
    ));
}

#[derive(Component)]
struct LoadedSceneItem;

#[derive(Component)]
struct LoadedSceneItemHandleMesh;

#[derive(Component)]
struct LoadedSceneItemHandleMaterial;

#[derive(Component)]
struct LoadedAudioItem;

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

fn cleanup(
    query: Query<Entity, (With<LoadedSceneItem>, With<SyncEntity>)>,
    mut commands: Commands,
) {
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

fn cleanup_audio(
    query_handle: Query<Entity, (Added<LoadedAudioItem>, Without<Handle<AudioSource>>)>,
    mut commands: Commands,
) {
    for e in query_handle.iter() {
        debug!("Cleaning up LoadedAudio from entity {:?}", e);
        commands.get_entity(e).unwrap().remove::<LoadedAudioItem>();
    }
}

fn handle_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &Handle<Mesh>), Added<LoadedSceneItemHandleMesh>>,
) {
    for (e, h) in query.iter() {
        let id = AssetId::Uuid {
            uuid: Uuid::new_v4(),
        };
        let mut asset = meshes.remove(h.id()).unwrap();
        if let Some(morphs) = extract_morph_targets(&asset) {
            if morphs.is_strong() {
                let morphs = morphs.clone();
                if let Some(morphs) = swap_single_image(&mut images, morphs) {
                    asset.set_morph_targets(morphs);
                    debug!("Reassigned morph targets on {:?}", id);
                }
            }
        }
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

fn extract_morph_targets(mesh: &Mesh) -> &Option<Handle<Image>> {
    let refmesh = mesh as &dyn Struct;
    let morph_targets = refmesh
        .field("morph_targets")
        .unwrap()
        .downcast_ref::<Option<Handle<Image>>>()
        .unwrap();
    morph_targets
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
        let Some(mut asset) = materials.remove(h.id()) else {
            return;
        };
        handle_images(images.as_mut(), &mut asset);
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

fn handle_audio(
    mut commands: Commands,
    mut assets: ResMut<Assets<AudioSource>>,
    query: Query<(Entity, &Handle<AudioSource>), Added<LoadedAudioItem>>,
) {
    for (e, h) in query.iter() {
        let id = AssetId::Uuid {
            uuid: Uuid::new_v4(),
        };
        let Some(asset) = assets.remove(h.id()) else {
            return;
        };
        assets.insert(id, asset);
        debug!("Reassigned audio to uuid {:?}", id);
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedAudioItem>()
            .remove::<Handle<AudioSource>>()
            .insert(Handle::Weak(id));
    }
}

fn handle_images(images: &mut Assets<Image>, material: &mut StandardMaterial) {
    macro_rules! swap_image {
        ($image:expr) => {
            if let Some(h) = $image.clone() {
                $image = swap_single_image(images, h);
            }
        };
    }
    swap_image!(material.base_color_texture);
    swap_image!(material.emissive_texture);
    swap_image!(material.normal_map_texture);
    swap_image!(material.occlusion_texture);
    swap_image!(material.metallic_roughness_texture);
}

fn swap_single_image(images: &mut Assets<Image>, image: Handle<Image>) -> Option<Handle<Image>> {
    let image = images.remove(image.id())?;
    let id = AssetId::Uuid {
        uuid: Uuid::new_v4(),
    };
    images.insert(id, image);
    debug!("Reassigned image to uuid {:?}", id);
    Some(Handle::Weak(id))
}
