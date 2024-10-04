mod flightcam;
mod naming;

use bevy::prelude::*;
use bevy_mod_openxr::add_xr_plugins;
use bevy_mod_xr::session::{XrEndSessionEvent, XrRequestExitEvent};
use bevy_xr_utils::{hand_gizmos::HandGizmosPlugin, xr_utils_actions::XRUtilsActionsPlugin};

pub fn init<G: PluginGroup>(app: &mut App, g: G) {
    info!("Initializing XR");
    app.add_plugins(add_xr_plugins(g))
        .add_plugins(HandGizmosPlugin)
        .add_plugins(XRUtilsActionsPlugin)
        .add_systems(PostUpdate, quit_xr_on_app_exit);
    naming::init(app);
    flightcam::init(app);
    lux_xr_avatar_generic::init(app);
}

fn quit_xr_on_app_exit(
    mut events: EventReader<AppExit>,
    mut writer_r: EventWriter<XrRequestExitEvent>,
    mut writer_e: EventWriter<XrEndSessionEvent>,
) {
    for _ in events.read() {
        info!("Quitting XR");
        writer_r.send(XrRequestExitEvent {});
        writer_e.send(XrEndSessionEvent {});
    }
}
