use bevy::prelude::*;
use bevy_sync::{SyncDown, SyncMark};

pub(crate) fn init(app: &mut App) {
    app.add_systems(Update, (propagate, cleanup, handle_mesh, handle_material));
}

pub(crate) fn import(file_name: &str, commands: &mut Commands, assets: &AssetServer) {
    let scene = assets.load(file_name.to_owned() + "#Scene0");
    commands
        .spawn((SceneBundle {
            scene,
            ..Default::default()
        },))
        .insert((
            LoadedSceneItem,
            LoadedSceneItemHandleMesh,
            LoadedSceneItemHandleMaterial,
        ));
}

#[derive(Component)]
struct LoadedSceneItem;

#[derive(Component)]
struct LoadedSceneItemHandleMesh;

#[derive(Component)]
struct LoadedSceneItemHandleMaterial;

fn propagate(query: Query<(Entity, &Children), With<LoadedSceneItem>>, mut commands: Commands) {
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
                .insert(LoadedSceneItemHandleMesh)
                .insert(LoadedSceneItemHandleMaterial)
                .insert(SyncMark);
        }
    }
}

fn cleanup(query: Query<Entity, (With<LoadedSceneItem>, With<SyncDown>)>, mut commands: Commands) {
    for e in query.iter() {
        commands.get_entity(e).unwrap().remove::<LoadedSceneItem>();
    }
}

fn handle_mesh() {}

fn handle_material() {}

#[cfg(test)]
mod test {
    use bevy::pbr::PbrPlugin;

    use super::*;

    fn load_test_world(mut commands: Commands, assets: Res<AssetServer>) {
        import("assets/cube.glb#Scene0", &mut commands, &assets);
    }

    #[test]
    fn test() {
        let mut app = setup_app();
        init(&mut app);
        app.add_systems(Startup, load_test_world);
        app.update();
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Scene>();
        app.init_asset::<Shader>();
        app.init_asset::<Mesh>();
        app.add_plugins(PbrPlugin::default());
        app
    }
}
