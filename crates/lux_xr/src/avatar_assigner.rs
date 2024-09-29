use bevy::prelude::*;
use bevy_mod_xr::session::XrTrackingRoot;
use lux_components::{Avatar, LocalUser};

pub fn init(app: &mut App) {
    app.add_systems(Update, local_user_exits_root);
    app.add_systems(Update, local_user_enters_root);
}

fn local_user_enters_root(
    mut cmd: Commands,
    prev_roots: Query<Entity, With<XrTrackingRoot>>,
    q: Query<Entity, (With<Avatar>, Added<LocalUser>)>,
) {
    for e in q.iter() {
        let mut root = cmd.entity(e);
        // Move all the slots XrTrackingRoot under the avatar slot
        for e in prev_roots.iter() {
            root.add_child(e);
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
            cmd.entity(e).remove::<XrTrackingRoot>();
        }
    }
}
