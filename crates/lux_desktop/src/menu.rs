use bevy::prelude::*;

pub struct MenuPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Default, Hash, States)]
pub enum MenuState {
    #[default]
    Off,
    Main,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.add_systems(
            PreUpdate,
            (
                esc_to_enter_menu.run_if(in_state(MenuState::Off)),
                esc_to_exit_menu.run_if(not(in_state(MenuState::Off))),
                ctrl_q,
            ),
        );
    }
}

fn esc_to_enter_menu(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<MenuState>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(MenuState::Main);
    }
}

fn esc_to_exit_menu(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<MenuState>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(MenuState::Off);
    }
}

fn ctrl_q(input: Res<ButtonInput<KeyCode>>, mut event: EventWriter<AppExit>) {
    if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && input.just_pressed(KeyCode::KeyQ)
    {
        event.send(AppExit::Success);
    }
}

#[cfg(test)]
mod test {
    use bevy::{input::InputPlugin, state::app::StatesPlugin};

    use super::*;

    #[test]
    fn test_initial_state() {
        let app = setup();
        state_is(&app, MenuState::Off);
    }

    #[test]
    fn test_transition_to_menu() {
        let mut app = setup();
        press_esc(&mut app);
        state_is(&app, MenuState::Main);
    }

    #[test]
    fn test_transition_back_from_menu() {
        let mut app = setup();
        press_esc(&mut app);
        press_esc(&mut app);
        state_is(&app, MenuState::Off);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(InputPlugin);
        app.add_plugins(StatesPlugin);
        app.add_plugins(MenuPlugin);
        app.update();
        app
    }

    fn state_is(app: &App, expected: MenuState) {
        let state = app.world().resource::<State<MenuState>>();
        assert_eq!(state.get(), &expected);
    }

    fn press_esc(app: &mut App) {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::Escape);
        input.release(KeyCode::Escape);
        app.update();
    }
}
