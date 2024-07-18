use bevy::prelude::*;

#[derive(Component, Default)]
pub struct User;

#[derive(Default)]
pub(crate) struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, _: &mut App) {}
}
