use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct User;

#[derive(Default)]
pub(crate) struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, _app: &mut App) {
        // app.sync_component::<User>();
    }
}
