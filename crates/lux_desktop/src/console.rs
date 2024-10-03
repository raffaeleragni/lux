use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;

pub fn init(app: &mut App) {
    app.add_console_command::<Test, _>(test_command);
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "test")]
struct Test {}

fn test_command(mut log: ConsoleCommand<Test>) {
    let Some(Ok(Test {})) = log.take() else {
        return;
    };
    error!("test");
}
