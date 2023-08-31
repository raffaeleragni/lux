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
        app.add_state::<MenuState>();
        app.add_systems(
            PreUpdate,
            (
                exc_to_enter_menu.run_if(in_state(MenuState::Off)),
                esc_to_exit_menu.run_if(not(in_state(MenuState::Off))),
            ),
        );
    }
}

fn exc_to_enter_menu(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<MenuState>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(MenuState::Main);
    }
}

fn esc_to_exit_menu(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<MenuState>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(MenuState::Off);
    }
}

#[cfg(test)]
mod test {
    use bevy::input::InputPlugin;

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
        app.add_plugins(MenuPlugin);
        app.update();
        app
    }

    fn state_is(app: &App, expected: MenuState) {
        let state = app.world.resource::<State<MenuState>>();
        assert_eq!(state.get(), &expected);
    }

    fn press_esc(app: &mut App) {
        let input = &mut app.world.resource_mut::<Input<KeyCode>>();
        input.press(KeyCode::Escape);
        input.release(KeyCode::Escape);
        app.update();
    }
}
