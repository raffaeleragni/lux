mod cli;
mod layouts;
mod menu;

use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use bevy::{
    app::ScheduleRunnerPlugin, pbr::wireframe::Wireframe, prelude::*, render::primitives::Aabb,
    utils::Uuid,
};
use bevy_oxr::{
    xr_input::{
        interactions::{XRDirectInteractor, XRInteractorState, XRRayInteractor},
        trackers::{
            AimPose, OpenXRController, OpenXRLeftController, OpenXRRightController, OpenXRTracker,
        },
    },
    DefaultXrPlugins,
};
use bevy_sync::prelude::*;
use clap::Parser;
use cli::{Args, Command};

fn main() {
    let mut app = App::new();
    let args = Args::parse();
    let headless = match args.command {
        Some(Command::Host {
            world_file: _,
            headless,
        }) => headless,
        _ => false,
    };

    app.insert_resource(args.clone());

    if headless {
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )));
    } else {
        if args.xr_enabled {
            app.add_plugins(DefaultXrPlugins);
            app.add_systems(Startup, spawn_controllers);
        } else {
            app.add_plugins(DefaultPlugins);
        }
        app.add_plugins(bevy_editor_pls::EditorPlugin::default());
        app.add_plugins(menu::MenuPlugin);

        layouts::setup(&mut app);
    }

    setup_sync(args.clone(), &mut app);

    components::register(&mut app);
    app.add_systems(
        Startup,
        load_world_from_args.run_if(resource_exists::<Args>()),
    );

    app.add_systems(
        Update,
        (loaded_scene_item_propagate, loaded_scene_item_cleanup),
    );
    app.run();
}

fn spawn_controllers(mut commands: Commands) {
    commands.spawn((
        OpenXRLeftController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
        XRRayInteractor,
        AimPose(Transform::default()),
        XRInteractorState::default(),
    ));
    commands.spawn((
        OpenXRRightController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
        XRDirectInteractor,
        XRInteractorState::default(),
    ));
}

trait AddByUuid<A: Asset> {
    fn addu(&mut self, asset: A) -> Handle<A>;
}
impl<A: Asset> AddByUuid<A> for Assets<A> {
    fn addu(&mut self, asset: A) -> Handle<A> {
        let id = AssetId::Uuid {
            uuid: Uuid::new_v4(),
        };
        self.insert(id, asset);
        Handle::<A>::Weak(id)
    }
}

fn setup_sync(args: Args, app: &mut App) {
    if args.command.is_none() {
        return;
    }
    app.add_plugins(SyncPlugin);
    app.sync_component::<Name>();
    app.sync_component::<Aabb>();
    app.sync_component::<Visibility>();
    app.sync_component::<Transform>();
    app.sync_component::<Wireframe>();
    app.sync_component::<PointLight>();
    app.sync_component::<Handle<StandardMaterial>>();
    app.sync_component::<Handle<Mesh>>();
    app.sync_materials(true);
    app.sync_meshes(true);

    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 4000;
    match &args.command {
        Some(Command::Host {
            world_file: _,
            headless: _,
        }) => app.add_plugins(ServerPlugin {
            ip: localhost,
            port,
        }),
        Some(Command::Join { ip }) => app.add_plugins(ClientPlugin {
            ip: ip.clone().to_owned(),
            port,
        }),
        _ => app,
    };
}

#[derive(Component)]
struct LoadedSceneItem;

fn loaded_scene_item_propagate(
    query: Query<(Entity, &Children), With<LoadedSceneItem>>,
    mut commands: Commands,
) {
    for (e, childs) in query.iter() {
        commands
            .get_entity(e)
            .unwrap()
            .remove::<LoadedSceneItem>()
            .insert(SyncMark);
        for c in childs {
            commands
                .get_entity(*c)
                .unwrap()
                .insert(LoadedSceneItem)
                .insert(SyncMark);
        }
    }
}

fn loaded_scene_item_cleanup(
    query: Query<Entity, (With<LoadedSceneItem>, With<SyncDown>)>,
    mut commands: Commands,
) {
    for e in query.iter() {
        commands.get_entity(e).unwrap().remove::<LoadedSceneItem>();
    }
}

fn load_world_from_args(
    args: Res<Args>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    match &args.command {
        Some(Command::Host {
            world_file,
            headless: _,
        }) => {
            let scene = assets.load(world_file.to_owned() + "#Scene0");
            commands.spawn((
                SceneBundle {
                    scene,
                    ..Default::default()
                },
                LoadedSceneItem,
            ));
        }
        _ => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.addu(shape::Plane::from_size(50.0).into()),
                    material: materials.addu(Color::rgb(0.3, 0.5, 0.3).into()),
                    ..default()
                },
                SyncMark,
                Name::new("Ground"),
                SyncExclude::<Name>::default(),
            ));
            commands.spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(4.0, 8.0, 4.0),
                    ..default()
                },
                SyncMark,
                Name::new("Light"),
            ));
            commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        }
    }
}
