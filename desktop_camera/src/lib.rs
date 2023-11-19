use bevy::prelude::*;

pub struct DesktopCameraPlugin;

impl Plugin for DesktopCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyMaps::default());
        app.add_systems(PreUpdate, noclip_movement);
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
    /// Movement speed in units per second
    pub speed: f32,
}

impl Default for NoClip {
    fn default() -> Self {
        Self { speed: 1f32 }
    }
}

fn noclip_movement(
    mut query: Query<(&mut Transform, &NoClip)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if input.pressed(KeyCode::W) {
        for (mut t, clip) in query.iter_mut() {
            t.translation.z += clip.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::S) {
        for (mut t, clip) in query.iter_mut() {
            t.translation.z -= clip.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::A) {
        for (mut t, clip) in query.iter_mut() {
            t.translation.x -= clip.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::D) {
        for (mut t, clip) in query.iter_mut() {
            t.translation.x += clip.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::Space) {
        for (mut t, clip) in query.iter_mut() {
            t.translation.y += clip.speed * time.delta_seconds();
        }
    }
    if input.pressed(KeyCode::ControlLeft) {
        for (mut t, clip) in query.iter_mut() {
            t.translation.y -= clip.speed * time.delta_seconds();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::{input::InputPlugin, time::TimePlugin};
    use std::time::Duration;

    #[test]
    fn initial_state() {
        let mut app = setup(1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_forward() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::W, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 1.0);
    }

    #[test]
    fn move_backward() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::S, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, -1.0);
    }

    #[test]
    fn move_left() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::A, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, -1.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_right() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::D, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 1.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_up() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::Space, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 1.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_down() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::ControlLeft, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, -1.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn consider_speed() {
        let mut app = setup(2.0);
        press(&mut app, KeyCode::W, 1.0);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 2.0);
    }

    #[test]
    fn consider_time() {
        let mut app = setup(1.0);
        press(&mut app, KeyCode::W, 1.5);
        let pos = get_camera_position(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 1.5);
    }

    fn press(app: &mut App, k: KeyCode, time_ms: f32) {
        let input = &mut app.world.resource_mut::<Input<KeyCode>>();
        input.press(k);
        app.world
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(time_ms));
        app.update();
        let input = &mut app.world.resource_mut::<Input<KeyCode>>();
        input.release(k);
        app.update();
    }

    fn setup(speed: f32) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins.build().disable::<TimePlugin>());
        app.add_plugins(InputPlugin);
        app.add_plugins(DesktopCameraPlugin);
        app.world
            .spawn((SpatialBundle::default(), NoClip { speed }));
        app.insert_resource::<Time>(Time::new_with(()));
        app.update();
        app
    }

    fn get_camera_position(app: &mut App) -> &Transform {
        return app
            .world
            .query_filtered::<&Transform, With<NoClip>>()
            .iter(&app.world)
            .next()
            .unwrap();
    }
}
