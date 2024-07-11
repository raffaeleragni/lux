use bevy::prelude::*;
use bevy_mod_xr::session::XrSessionPlugin;

pub fn init(app: &mut App) {
    info!("Initializing XR");
    app.add_plugins(XrSessionPlugin { auto_handle: true });
}
