use bevy::prelude::*;

use crate::{GameState, UI_Z, cleanup, has_user_input};

pub struct GameOverPlugin;

#[derive(Component, Default)]
pub struct DespawnOnReset;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(
                Update,
                goto_menu.run_if(in_state(GameState::GameOver).and(has_user_input)),
            )
            .add_systems(OnExit(GameState::GameOver), cleanup::<DespawnOnReset>);
    }
}

fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprite/game_over.png")),
        Transform::from_xyz(0., 80., UI_Z),
        DespawnOnReset,
    ));
}

fn goto_menu(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}
