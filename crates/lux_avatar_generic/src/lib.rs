use avatars::AvatarPlugin;

mod avatars;

pub use avatars::bones;
pub use avatars::Avatar as AvatarGeneric;

pub fn init(app: &mut bevy::prelude::App) {
    app.add_plugins(AvatarPlugin);
}