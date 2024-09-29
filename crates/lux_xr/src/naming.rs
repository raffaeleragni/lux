use bevy::prelude::*;
use bevy_mod_xr::session::XrTrackingRoot;

pub fn init(app: &mut App) {
    app.add_systems(Update, name_xr_root);
}

fn name_xr_root(mut cmd: Commands, roots: Query<Entity, (With<XrTrackingRoot>, Without<Name>)>) {
    for e in roots.iter() {
        cmd.entity(e).insert(Name::new("XrRoot"));
    }
}
