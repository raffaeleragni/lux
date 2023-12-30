use std::time::Duration;

use bevy::{app::AppExit, prelude::*, time::common_conditions::on_timer, utils::Uuid};
use lux_cli::{Args, Command};
use lux_world::init;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.insert_resource(Args {
        xr_enabled: false,
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
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    mut quit_events: EventWriter<AppExit>,
) {
    let mut mesh_uuid: Option<Uuid> = None;
    let mut material_uuid: Option<Uuid> = None;
    for handle in mesh_query.iter() {
        match handle {
            Handle::Strong(_) => {
                println!("MESH IS STRONG HANDLE");
                return;
            }
            Handle::Weak(id) => {
                let AssetId::Uuid { uuid: id }: &AssetId<Mesh> = id else {
                    break;
                };
                mesh_uuid = Some(*id);
            }
        }
    }
    for handle in material_query.iter() {
        match handle {
            Handle::Strong(_) => {
                println!("MATERIAL IS STRONG HANDLE");
                return;
            }
            Handle::Weak(id) => {
                let AssetId::Uuid { uuid: id }: &AssetId<StandardMaterial> = id else {
                    break;
                };
                material_uuid = Some(*id);
            }
        }
    }
    if mesh_uuid.is_some() && material_uuid.is_some() {
        println!("OK: {:?}:{:?}", mesh_uuid, material_uuid);
        quit_events.send(AppExit);
    } else {
        println!("HANDLES MISSING");
    }
    println!("Mesh count: {}", meshes.len());
    println!("Material count: {}", materials.len());
}
