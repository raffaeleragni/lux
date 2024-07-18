pub use avatars::Avatar;
pub use controlled_by::ControlledBy;
pub use local_user::LocalUser;
pub use user::User;

mod avatars;
mod controlled_by;
mod local_user;
mod user;

use avatars::AvatarPlugin;
use local_user::LocalUserPlugin;
use user::UserPlugin;

pub fn init(app: &mut bevy::prelude::App) {
    app.add_plugins(AvatarPlugin);
    app.add_plugins(LocalUserPlugin);
    app.add_plugins(UserPlugin);
}
