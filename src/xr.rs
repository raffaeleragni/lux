use bevy::prelude::*;
use bevy_oxr::{
    xr_input::{
        hand::HandInputDebugRenderer,
        interactions::{XRDirectInteractor, XRInteractorState, XRRayInteractor},
        trackers::{
            AimPose, OpenXRController, OpenXRLeftController, OpenXRRightController, OpenXRTracker,
        },
    },
    DefaultXrPlugins,
};

pub fn init(app: &mut App) {
    app.add_plugins(DefaultXrPlugins);
    app.add_plugins(HandInputDebugRenderer);
    app.add_systems(Startup, spawn_xr_entities);
}

fn spawn_xr_entities(mut commands: Commands) {
    commands.spawn((
        OpenXRLeftController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
        XRRayInteractor,
        AimPose(Transform::default()),
        XRInteractorState::default(),
    ));
    commands.spawn((
        OpenXRRightController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
        XRDirectInteractor,
        XRInteractorState::default(),
    ));
}

