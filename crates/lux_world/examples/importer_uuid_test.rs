use std::time::Duration;

use bevy::{app::AppExit, prelude::*, time::common_conditions::on_timer};
use bevy_sync::Uuid;
use lux_cli::{Args, Command};
use lux_world::{import_audio, init};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.insert_resource(Args {
        xr_enabled: false,
        command: Some(Command::Host {
            world_file: "cube.glb".to_string(),
            headless: false,
            ip: None,
            avatar_file: None,
        }),
    });
    init(&mut app);
    app.add_systems(Startup, create);
    app.add_systems(
        Update,
        check_handles.run_if(on_timer(Duration::from_secs(1))),
    );
    app.run();
}

fn create(assets: Res<AssetServer>, mut commands: Commands) {
    import_audio("empty.ogg", &mut commands, &assets);
}

#[allow(clippy::too_many_arguments)]
fn check_handles(
    mesh_query: Query<&Handle<Mesh>>,
    material_query: Query<&Handle<StandardMaterial>>,
    audio_query: Query<&Handle<AudioSource>>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    images: Res<Assets<Image>>,
    audios: Res<Assets<AudioSource>>,
    mut quit_events: EventWriter<AppExit>,
) {
    let mut mesh_uuid: Option<Uuid> = None;
    let mut material_uuid: Option<Uuid> = None;
    let mut image_uuid: Option<Uuid> = None;
    let mut audio_uuid: Option<Uuid> = None;
    for handle in audio_query.iter() {
        match handle {
            Handle::Strong(_) => println!("AUDIO IS STRONG HANDLE"),
            Handle::Weak(id) => {
                let AssetId::Uuid { uuid: id }: &AssetId<AudioSource> = id else {
                    break;
                };
                audio_uuid = Some(*id);
            }
        }
    }
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
                let material = materials.get(handle).unwrap();
                if let Some(image_id) = material.base_color_texture.clone() {
                    match image_id {
                        Handle::Strong(_) => {
                            println!("IMAGE IS STRONG HANDLE");
                            return;
                        }
                        Handle::Weak(id) => {
                            let AssetId::Uuid { uuid: id }: AssetId<Image> = id else {
                                break;
                            };
                            image_uuid = Some(id);
                        }
                    }
                }
            }
        }
    }
    if mesh_uuid.is_some()
        && material_uuid.is_some()
        && image_uuid.is_some()
        && audio_uuid.is_some()
    {
        println!(
            "OK: {:?}:{:?}:{:?}:{:?}",
            mesh_uuid, material_uuid, image_uuid, audio_uuid
        );
        quit_events.send(AppExit::Success);
    } else {
        println!("HANDLES MISSING");
    }
    println!("Mesh count: {}", meshes.len());
    println!("Material count: {}", materials.len());
    println!("Image count: {}", images.len());
    println!("Audio count: {}", audios.len());
}
