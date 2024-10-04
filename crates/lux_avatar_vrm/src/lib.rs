use avian3d::{collision::Collider, prelude::RigidBody, PhysicsPlugins};
use bevy::prelude::*;
use bevy_vr_controller::VrControllerPlugin;

pub fn init(app: &mut App) {
    app.add_plugins((PhysicsPlugins::default(), VrControllerPlugin));
    app.add_systems(Startup, setup_ground);
}

const GROUND_SIZE: f32 = 100.0;
const GROUND_THICKNESS: f32 = 0.5;

fn setup_ground(mut commands: Commands) {
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -GROUND_THICKNESS, 0.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(GROUND_SIZE, GROUND_THICKNESS, GROUND_SIZE),
    ));
}
