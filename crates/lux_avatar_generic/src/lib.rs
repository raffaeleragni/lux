use avatars::AvatarPlugin;

mod avatars;

pub use avatars::Avatar as AvatarGeneric;
pub use avatars::bones;

pub fn init(app: &mut bevy::prelude::App) {
    app.add_plugins(AvatarPlugin);
}
