pub use avatars::Avatar;
pub use controlled_by::ControlledBy;
pub use local_user::LocalUser;

mod avatars;
mod controlled_by;
mod local_user;

use avatars::AvatarPlugin;
use local_user::LocalUserPlugin;

pub fn init(app: &mut bevy::prelude::App) {
    app.add_plugins(AvatarPlugin);
    app.add_plugins(LocalUserPlugin);
}
