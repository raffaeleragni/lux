pub use controlled_by::ControlledBy;
pub use local_user::LocalUser;
pub use reference::ComponentEntityRef;
pub use user::User;

mod controlled_by;
mod local_user;
mod reference;
mod user;

use local_user::LocalUserPlugin;
use user::UserPlugin;

pub fn init(app: &mut bevy::prelude::App) {
    app.add_plugins(LocalUserPlugin);
    app.add_plugins(UserPlugin);
}
