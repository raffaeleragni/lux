use bevy::prelude::*;
use bevy_console::ConsolePlugin;
use lux_desktop_camera::NoClip;

mod console;
mod layouts;
mod menu;

pub fn init(app: &mut App) {
    app.world_mut().spawn((
        NoClip::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
    app.add_plugins(menu::MenuPlugin);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.add_plugins(lux_desktop_camera::DesktopCameraPlugin);
    app.add_plugins(ConsolePlugin);
    layouts::init(app);
    console::init(app);
}
