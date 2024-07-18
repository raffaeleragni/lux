use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LocalUser;

#[derive(Default)]
pub(crate) struct LocalUserPlugin;

impl Plugin for LocalUserPlugin {
    fn build(&self, _: &mut App) {}
}
