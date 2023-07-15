use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    components::register(&mut app);
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.run();
}
