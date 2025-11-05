use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowPlugin, WindowResolution},
};

use {components::MenuPlugin, game::GamePlugin, game_over::GameOverPlugin};

mod components;
mod game;
mod game_over;

const PIPE_Z: f32 = 1.0;
const GROUND_Z: f32 = 2.0;
const BIRD_Z: f32 = 3.0;
const UI_Z: f32 = 4.0;

const BIRD_SIZE: Vec2 = Vec2::new(34., 24.);
const PIPE_SIZE: Vec2 = Vec2::new(52., 320.);
const GROUND_WIDTH: f32 = 336.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
pub struct Scroll;

#[derive(Resource)]
pub struct AudioHandles {
    flap: Handle<AudioSource>,
    hit: Handle<AudioSource>,
    point: Handle<AudioSource>,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
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
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((MenuPlugin, GamePlugin, GameOverPlugin))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let background = Sprite::from_image(asset_server.load("sprite/background.png"));
    let ground = Sprite::from_image(asset_server.load("sprite/ground.png"));

    commands.spawn((background, Transform::from_xyz(0., 0., 1.)));

    for i in 0..2 {
        commands.spawn((
            ground.clone(),
            Ground,
            Scroll,
            Transform::from_xyz(i as f32 * GROUND_WIDTH, -200., GROUND_Z),
        ));
    }

    commands.insert_resource(AudioHandles {
        flap: asset_server.load("audio/flap.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        point: asset_server.load("audio/point.ogg"),
    });
}

#[must_use]
pub fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}

pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
