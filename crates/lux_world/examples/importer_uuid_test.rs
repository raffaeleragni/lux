use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, app::AppExit};
use lux_cli::{Args, Command};
use lux_world::init;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.insert_resource(Args {
        command: Some(Command::Host {
            world_file: "cube.glb".to_string(),
            headless: false,
        }),
    });
    init(&mut app);
    app.add_systems(
        Update,
        check_handles.run_if(on_timer(Duration::from_secs(1))),
    );
    app.run();
}

fn check_handles(
    mesh_query: Query<&Handle<Mesh>>,
    material_query: Query<&Handle<StandardMaterial>>,
    mut quit_events: EventWriter<AppExit>
) {
    for handle in mesh_query.iter() {
        if let Handle::Strong(_) = handle {
            println!("MESH IS STRONG HANDLE");
            return;
        }
    }
    for handle in material_query.iter() {
        if let Handle::Strong(_) = handle {
            println!("MATERIAL IS STRONG HANDLE");
            return;
        }
    }
    println!("OK");
    quit_events.send(AppExit);    
}
