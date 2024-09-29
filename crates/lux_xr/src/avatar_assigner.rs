use bevy::prelude::*;
use bevy_mod_xr::session::XrTrackingRoot;
use lux_components::{Avatar, LocalUser};

pub fn init(app: &mut App) {
    app.add_systems(
        Update,
        (local_user_exits_root, local_user_enters_root).chain(),
    );
    // app.add_systems(Update, (copy_xr_view_transform).chain());
}

fn local_user_enters_root(
    mut cmd: Commands,
    xr_root: Query<Entity, With<XrTrackingRoot>>,
    q: Query<Entity, (With<Avatar>, Added<LocalUser>)>,
) {
    for avatar_root_id in q.iter() {
        let avatar_root = cmd.entity(avatar_root_id).id();
        if let Ok(xr_root_id) = xr_root.get_single() {
            cmd.entity(xr_root_id).add_child(avatar_root);
        }
    }
}

fn local_user_exits_root(
    mut cmd: Commands,
    mut removed: RemovedComponents<LocalUser>,
    q: Query<Entity, With<Avatar>>,
) {
    for entity in removed.read() {
        if let Ok(e) = q.get(entity) {
            cmd.entity(e).remove_parent();
        }
    }
}
