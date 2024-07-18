use bevy::prelude::*;

#[derive(Component)]
pub struct Avatar;

#[derive(Default)]
pub(crate) struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, _: &mut App) {}
}
