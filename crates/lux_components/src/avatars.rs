use bevy::prelude::*;
use bevy_sync::SyncComponent;

#[derive(Component, Reflect)]
pub struct Avatar;

#[derive(Default)]
pub(crate) struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.sync_component::<Avatar>();
    }
}
