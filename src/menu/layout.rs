use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};

pub fn setup(app: &mut App) {
    app.add_systems(
        Update,
        render_main_menu.run_if(in_state(super::MenuState::Main)),
    );
}

fn render_main_menu(mut contexts: EguiContexts, mut exit: EventWriter<AppExit>) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Quit").clicked() {
            exit.send(AppExit);
        }
    });
}
