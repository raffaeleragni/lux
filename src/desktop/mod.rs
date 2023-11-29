use bevy::prelude::*;
use desktop_camera::NoClip;

mod layouts;
mod menu;

pub fn init(app: &mut App) {
    app.world.spawn((
        NoClip::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
    app.add_plugins(menu::MenuPlugin);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.add_plugins(desktop_camera::DesktopCameraPlugin);
    layouts::init(app);
}
