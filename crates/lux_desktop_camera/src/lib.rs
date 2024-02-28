use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct DesktopCameraPlugin;

impl Plugin for DesktopCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyMaps::default());
        app.add_systems(PreUpdate, (noclip_movement, noclip_look));
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
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::Space,
            down: KeyCode::ControlLeft,
        }
    }
}

#[derive(Component)]
pub struct NoClip {
    /// Movement speed in units per second
    pub speed: f32,
    pub mouse_speed: f32,
    pub mouse_vertical: f32,
    pub mouse_horizontal: f32,
}

impl Default for NoClip {
    fn default() -> Self {
        Self {
            speed: 1.0,
            mouse_speed: 10.0,
            mouse_vertical: 0.0,
            mouse_horizontal: 0.0,
        }
    }
}

fn forward(rot: &Quat) -> Vec3 {
    rot.mul_vec3(-Vec3::Z).normalize()
}

fn forward_noy(rot: &Quat) -> Vec3 {
    let v = forward(rot);
    Vec3::new(v.x, 0.0, v.z).normalize()
}

fn left(rot: &Quat) -> Vec3 {
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_noy(rot))
        .normalize()
}

fn noclip_movement(
    mut query: Query<(&mut Transform, &NoClip)>,
    input: Res<ButtonInput<KeyCode>>,
    maps: Res<KeyMaps>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }
    for (mut t, clip) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(maps.forward) {
            direction += forward(&t.rotation);
        }
        if input.pressed(maps.backward) {
            direction -= forward(&t.rotation);
        }
        if input.pressed(maps.left) {
            direction += left(&t.rotation);
        }
        if input.pressed(maps.right) {
            direction -= left(&t.rotation);
        }
        if input.pressed(maps.up) {
            direction += Vec3::Y;
        }
        if input.pressed(maps.down) {
            direction -= Vec3::Y;
        }
        direction = direction.normalize();
        if direction.is_nan() {
            continue;
        }
        t.translation += direction * clip.speed * time.delta_seconds();
    }
}

fn noclip_look(
    mut query: Query<(&mut Transform, &mut NoClip)>,
    mut mouse: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }
    let mut total = Vec2::ZERO;
    for motion in mouse.read() {
        total += motion.delta;
    }
    if total == Vec2::ZERO {
        return;
    }
    for (mut t, mut clip) in query.iter_mut() {
        let y = clip.mouse_speed * time.delta_seconds() * total.y;
        let x = clip.mouse_speed * time.delta_seconds() * total.x;
        clip.mouse_vertical += y;
        clip.mouse_horizontal -= x;
        clip.mouse_vertical = clip.mouse_vertical.clamp(-90.0, 90.0);
        t.rotation = Quat::from_axis_angle(Vec3::Y, clip.mouse_horizontal.to_radians())
            * Quat::from_axis_angle(-Vec3::X, clip.mouse_vertical.to_radians());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::{
        input::{mouse::MouseMotion, InputPlugin},
        time::TimePlugin,
    };
    use std::time::Duration;

    #[test]
    fn initial_state() {
        let mut app = setup(1.0, 10.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_forward() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::KeyW, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, -1.0);
    }

    #[test]
    fn move_backward() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::KeyS, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 1.0);
    }

    #[test]
    fn move_left() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::KeyA, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, -1.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_right() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::KeyD, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 1.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_up() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::Space, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 1.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn move_down() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::ControlLeft, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, -1.0);
        assert_eq!(pos.translation.z, 0.0);
    }

    #[test]
    fn consider_speed() {
        let mut app = setup(2.0, 10.0);
        press(&mut app, KeyCode::KeyW, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, -2.0);
    }

    #[test]
    fn consider_time() {
        let mut app = setup(1.0, 10.0);
        press(&mut app, KeyCode::KeyW, 1.5);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, -1.5);
    }

    #[test]
    fn no_rotation() {
        let mut app = setup(1.0, 10.0);
        mouse_move(&mut app, Vec2::new(0.0, 0.0), 1.0);
        let rot = get_camera(&mut app);
        assert_eq!(rot.rotation.x, 0.0);
        assert_eq!(rot.rotation.y, 0.0);
        assert_eq!(rot.rotation.z, 0.0);
    }

    #[test]
    fn rotate_up() {
        let mut app = setup(1.0, 10.0);
        mouse_move(&mut app, Vec2::new(0.0, 1.0), 1.0);
        let rot = get_camera(&mut app);
        assert_eq!(
            rot.rotation,
            Quat::from_axis_angle(-Vec3::X, 10f32.to_radians())
        );
    }

    #[test]
    fn rotate_down() {
        let mut app = setup(1.0, 10.0);
        mouse_move(&mut app, Vec2::new(0.0, -1.0), 1.0);
        let rot = get_camera(&mut app);
        assert_eq!(
            rot.rotation,
            Quat::from_axis_angle(-Vec3::X, -10f32.to_radians())
        );
    }

    #[test]
    fn rotate_left() {
        let mut app = setup(1.0, 10.0);
        mouse_move(&mut app, Vec2::new(1.0, 0.0), 1.0);
        let rot = get_camera(&mut app);
        assert_eq!(
            rot.rotation,
            Quat::from_axis_angle(Vec3::Y, -10f32.to_radians())
        );
    }

    #[test]
    fn rotate_right() {
        let mut app = setup(1.0, 10.0);
        mouse_move(&mut app, Vec2::new(-1.0, 0.0), 1.0);
        let rot = get_camera(&mut app);
        assert_eq!(
            rot.rotation,
            Quat::from_axis_angle(Vec3::Y, 10f32.to_radians())
        );
    }

    #[test]
    fn move_left_rotated() {
        let mut app = setup(1.0, 10.0);
        mouse_rotated(&mut app, Quat::from_rotation_y(90.0f32.to_radians()));
        press(&mut app, KeyCode::KeyA, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, 1.0);
    }

    #[test]
    fn move_right_rotated() {
        let mut app = setup(1.0, 10.0);
        mouse_rotated(&mut app, Quat::from_rotation_y(90.0f32.to_radians()));
        press(&mut app, KeyCode::KeyD, 1.0);
        let pos = get_camera(&mut app);
        assert_eq!(pos.translation.x, 0.0);
        assert_eq!(pos.translation.y, 0.0);
        assert_eq!(pos.translation.z, -1.0);
    }

    fn press(app: &mut App, k: KeyCode, time_ms: f32) {
        let input = &mut app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(k);
        app.world
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(time_ms));
        app.update();
        let input = &mut app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.release(k);
        app.update();
    }

    fn mouse_move(app: &mut App, delta: Vec2, time_ms: f32) {
        app.world.send_event(MouseMotion { delta });
        app.world
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(time_ms));
        app.update();
    }

    fn mouse_rotated(app: &mut App, rot: Quat) {
        let mut t = app
            .world
            .query_filtered::<&mut Transform, With<NoClip>>()
            .iter_mut(&mut app.world)
            .next()
            .unwrap();
        t.rotation = rot;
    }

    fn setup(speed: f32, mouse_speed: f32) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins.build().disable::<TimePlugin>());
        app.add_plugins(InputPlugin);
        app.add_plugins(DesktopCameraPlugin);
        app.world.spawn((
            SpatialBundle::default(),
            NoClip {
                speed,
                mouse_speed,
                mouse_vertical: 0.0,
                mouse_horizontal: 0.0,
            },
        ));
        app.insert_resource::<Time>(Time::new_with(()));
        app.update();
        app
    }

    fn get_camera(app: &mut App) -> &Transform {
        return app
            .world
            .query_filtered::<&Transform, With<NoClip>>()
            .iter(&app.world)
            .next()
            .unwrap();
    }
}
