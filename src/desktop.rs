use bevy::prelude::*;
mod layouts;
mod menu;
pub fn init(app: &mut App) {
    app.world.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    app.add_plugins(menu::MenuPlugin);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    layouts::init(app);
}