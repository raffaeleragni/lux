use bevy::{prelude::*, utils::Uuid};
use bevy_sync::SyncMark;

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

pub fn spawn_empty_world(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.addu(Plane3d::default().mesh().size(50.0, 50.0).into()),
            material: materials.addu(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        SyncMark,
        Name::new("Ground"),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.addu(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
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
