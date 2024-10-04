#![allow(clippy::type_complexity)]

use bevy::{math::vec3, prelude::*};
use bevy_mod_xr::{
    camera::XrCamera,
    hands::{HandBone, LeftHand, RightHand},
    session::XrTrackingRoot,
};
use lux_avatar_generic::{bones::*, AvatarGeneric};
use lux_components::LocalUser;

pub fn init(app: &mut App) {
    app.add_systems(
        Update,
        (local_user_exits_root, local_user_enters_root).chain(),
    );
    app.add_systems(
        Update,
        (copy_head, copy_transform_hand_l, copy_transform_hand_r).chain(),
    );
}

fn local_user_enters_root(
    mut cmd: Commands,
    xr_root: Query<Entity, With<XrTrackingRoot>>,
    q: Query<Entity, (With<AvatarGeneric>, Added<LocalUser>)>,
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
    q: Query<Entity, With<AvatarGeneric>>,
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

fn copy_head(
    src: Query<&Transform, With<XrCamera>>,
    mut dst: Query<&mut Transform, (Without<XrCamera>, With<Target<Head>>, With<LocalUser>)>,
) {
    for mut tfd in dst.iter_mut() {
        let mut pos = vec3(0.0, 0.0, 0.0);
        let mut ct = vec3(0.0, 0.0, 0.0);
        for tfs in src.iter() {
            let rot = tfs.rotation.to_euler(EulerRot::XYZ);
            let rot =
                Quat::from_euler(EulerRot::XYZ, -rot.0, rot.1 - f32::to_radians(180.0), rot.2);
            tfd.rotation = rot;
            pos += tfs.translation;
            ct += vec3(1.0, 1.0, 1.0);
        }
        tfd.translation = pos / ct;
    }
}
