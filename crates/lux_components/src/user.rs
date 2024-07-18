use bevy::prelude::*;
use bevy_sync::SyncComponent;

#[derive(Component, Default, Reflect)]
pub struct User;

#[derive(Default)]
pub(crate) struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.sync_component::<User>();
    }
}
