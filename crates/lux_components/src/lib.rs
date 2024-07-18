pub use avatars::Avatar;
use avatars::AvatarPlugin;
pub use controlled_by::ControlledBy;

mod avatars;
mod controlled_by;

pub fn init(app: &mut bevy::prelude::App) {
    app.add_plugins(AvatarPlugin);
}
