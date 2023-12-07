use crate::cli::{Args, Command};
use bevy::{prelude::*, utils::Uuid};
use bevy_sync::{SyncDown, SyncMark};

pub fn init(app: &mut App) {
    app.add_systems(
        Startup,
        load_world_from_args.run_if(resource_exists::<Args>()),
    );

    app.add_systems(
        Update,
        (loaded_scene_item_propagate, loaded_scene_item_cleanup),
    );
}

trait AddByUuid<A: Asset> {
    fn addu(&mut self, asset: A) -> Handle<A>;
}
impl<A: Asset> AddByUuid<A> for Assets<A> {
    fn addu(&mut self, asset: A) -> Handle<A> {
        let id = AssetId::Uuid {
            uuid: Uuid::new_v4(),
        };
        self.insert(id, asset);
        Handle::<A>::Weak(id)
    }
}

#[derive(Component)]
struct LoadedSceneItem;

fn loaded_scene_item_propagate(
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
                .insert(SyncMark);
        }
    }
}

fn loaded_scene_item_cleanup(
    query: Query<Entity, (With<LoadedSceneItem>, With<SyncDown>)>,
    mut commands: Commands,
) {
    for e in query.iter() {
        commands.get_entity(e).unwrap().remove::<LoadedSceneItem>();
    }
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
        }) => {
            let scene = assets.load(world_file.to_owned() + "#Scene0");
            commands.spawn((
                SceneBundle {
                    scene,
                    ..Default::default()
                },
                LoadedSceneItem,
            ));
        }
        Some(Command::Join { ip: _ }) => (),
        _ => spawn_empty_world(meshes, materials, commands),
    }
}

fn spawn_empty_world(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.addu(shape::Plane::from_size(50.0).into()),
            material: materials.addu(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        SyncMark,
        Name::new("Ground"),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.addu(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.addu(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 1.0),
            ..default()
        },
        SyncMark,
        Name::new("Cube"),
    ));
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        SyncMark,
        Name::new("Light"),
    ));
}
