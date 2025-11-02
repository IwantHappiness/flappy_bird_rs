#![allow(unused)]
use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowPlugin, WindowResolution},
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".to_string(),
                resolution: WindowResolution::new(288, 512),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    minimize: false,
                    maximize: false,
                    close: true,
                },
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .run();
}
