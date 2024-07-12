use bevy::prelude::*;

pub fn init(app: &mut App) {
    info!("Initializing XR");
    app.add_plugins(bevy_mod_openxr::add_xr_plugins(DefaultPlugins))
        .add_plugins(bevy_xr_utils::hand_gizmos::HandGizmosPlugin);
}
