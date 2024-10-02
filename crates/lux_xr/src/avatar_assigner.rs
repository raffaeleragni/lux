#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_mod_xr::{
    hands::{HandBone, LeftHand, RightHand},
    session::XrTrackingRoot,
};
use lux_components::{
    avatar_bones::{HandL, HandR, Target},
    Avatar, LocalUser,
};

pub fn init(app: &mut App) {
    app.add_systems(
        Update,
        (local_user_exits_root, local_user_enters_root).chain(),
    );
    app.add_systems(
        Update,
        (copy_transform_hand_l, copy_transform_hand_r).chain(),
    );
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

fn copy_transform_hand_l(
    src: Query<(&Transform, &HandBone), With<LeftHand>>,
    mut dst: Query<&mut Transform, (Without<LeftHand>, With<Target<HandL>>, With<LocalUser>)>,
) {
    for mut tfd in dst.iter_mut() {
        for (tfs, b) in src.iter() {
            if let HandBone::Palm = b {
                tfd.translation = tfs.translation;
                tfd.rotation = tfs.rotation;
            }
        }
    }
}

fn copy_transform_hand_r(
    src: Query<(&Transform, &HandBone), With<RightHand>>,
    mut dst: Query<&mut Transform, (Without<RightHand>, With<Target<HandR>>, With<LocalUser>)>,
) {
    for mut tfd in dst.iter_mut() {
        for (tfs, b) in src.iter() {
            if let HandBone::Palm = b {
                tfd.translation = tfs.translation;
                tfd.rotation = tfs.rotation;
            }
        }
    }
}
