use bevy::prelude::*;
use lux_desktop_camera::NoClip;

mod layouts;
mod menu;

pub fn init(app: &mut App) {
    app.world_mut().spawn((
        NoClip::default(),
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    app.add_plugins(menu::MenuPlugin);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.add_plugins(lux_desktop_camera::DesktopCameraPlugin);
    layouts::init(app);
}
