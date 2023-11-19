use bevy::prelude::*;

pub struct DesktopCameraPlugin;

impl Plugin for DesktopCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyMaps::default());
    }
}

#[derive(Resource)]
pub struct KeyMaps {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
}

impl Default for KeyMaps {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            up: KeyCode::Space,
            down: KeyCode::ControlLeft,
        }
    }
}

#[derive(Component)]
pub struct NoClip {
    pub speed: f32,
}

impl Default for NoClip {
    fn default() -> Self {
        Self { speed: 1f32 }
    }
}

#[cfg(test)]
mod test {
    use bevy::input::InputPlugin;

    use super::*;

    #[test]
    fn initial_state() {
        let mut app = setup();
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0f32);
        assert_eq!(pos.translation.y, 0f32);
        assert_eq!(pos.translation.z, 0f32);
    }

    #[test]
    fn move_forward() {
        let mut app = setup();
        press(&mut app, KeyCode::W);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0f32);
        assert_eq!(pos.translation.y, 0f32);
        // TODO 1 is not enough, needs to be proportional to time pressed
        assert_eq!(pos.translation.z, 1f32);
    }

    fn press(app: &mut App, k: KeyCode) {
        let input = &mut app.world.resource_mut::<Input<KeyCode>>();
        input.press(k);
        // TODO need to tell beyv how much time is elapsed between updates
        app.update();
        let input = &mut app.world.resource_mut::<Input<KeyCode>>();
        input.release(k);
        app.update();
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(InputPlugin);
        app.add_plugins(DesktopCameraPlugin);
        app.world.spawn((
            SpatialBundle::default(),
            Camera3d::default(),
            NoClip::default(),
        ));
        app
    }

    fn get_camera_position(app: &mut App) -> &Transform {
        return app
            .world
            .query_filtered::<&Transform, With<Camera3d>>()
            .iter(&app.world)
            .next()
            .unwrap();
    }
}
