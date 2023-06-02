use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(bevy_editor_pls::EditorPlugin::default());
    app.run();
}
