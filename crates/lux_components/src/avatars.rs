use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_sync::SyncComponent;

#[derive(Component, Reflect)]
pub struct Avatar;

#[derive(Default)]
pub(crate) struct AvatarPlugin;

enum Bones {
    Root,
    Hips,
    Spine,
    Neck,
    Head,
}

#[derive(Default)]
struct Bone<Bones> {
    b: PhantomData<Bones>
}

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.sync_component::<Avatar>();
    }
}
