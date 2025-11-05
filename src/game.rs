use crate::{
    AudioHandles, BIRD_SIZE, BIRD_Z, GROUND_WIDTH, GameState, Ground, Scroll,
    components::{
        Bird, PIPE_SPAWN_TIME, PipeSpawnTimer, animate_bird, check_passed_pipe,
        check_pipe_collision, despawn_pipe, fall, jump, move_bird, spawn_pipe,
    },
    game_over::DespawnOnReset,
    has_user_input,
};
use bevy::prelude::*;

const SCROLL_SPEED: f32 = 125.0;
const DEATH_HEIGHT: f32 = -125.0;
pub const GAP_HEIGHT: f32 = 100.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayState {
    #[default]
    Normal,
    HitPipe,
}

#[derive(Resource, Default)]
pub struct Score(pub usize);

#[derive(Component)]
struct ScoreText;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayState>()
            .init_resource::<Score>()
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(
                PIPE_SPAWN_TIME,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(
                OnExit(GameState::Playing),
                (hit_sound, reset_score, reset_timer),
            )
            .add_systems(OnEnter(PlayState::HitPipe), hit_sound)
            .add_systems(
                Update,
                (
                    animate_bird,
                    jump.run_if(has_user_input),
                    check_passed_pipe,
                    check_pipe_collision,
                    spawn_pipe,
                    despawn_pipe,
                    flap_sound.run_if(has_user_input),
                    point_sound.run_if(resource_changed::<Score>),
                    update_score_text,
                    scroll,
                    reuse_ground,
                )
                    .run_if(in_state(GameState::Playing).and(in_state(PlayState::Normal))),
            )
            .add_systems(
                Update,
                (fall, move_bird, check_death).run_if(in_state(GameState::Playing)),
            );
    }
}

fn game_setup(
    mut commands: Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2 {
            x: BIRD_SIZE.x as u32,
            y: BIRD_SIZE.y as u32,
        },
        4,
        1,
        None,
        None,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("sprite/bird.png"),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: 0,
            }),
            ..default()
        },
        DespawnOnReset,
        Bird::default(),
        Transform::from_xyz(0., 0., BIRD_Z),
    ));

    commands
        .spawn((
            DespawnOnReset,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
        ))
        .with_children(|node| {
            node.spawn((
                ScoreText,
                Text::new("0"),
                TextFont {
                    font: asset_server.load("fonts/flappybird.ttf"),
                    font_size: 80.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });
    play_state.set(PlayState::Normal);
}

fn scroll(mut query: Query<&mut Transform, With<Scroll>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x -= SCROLL_SPEED * time.delta_secs();
    }
}

fn reuse_ground(mut query: Query<&mut Transform, With<Ground>>) {
    for mut transform in &mut query {
        if transform.translation.x < -GROUND_WIDTH {
            transform.translation.x += GROUND_WIDTH * 2.0;
        }
    }
}

// End the game if the bird is below the death height
fn check_death(bird: Query<&Transform, With<Bird>>, mut state: ResMut<NextState<GameState>>) {
    for bird in &bird {
        if bird.translation.y < DEATH_HEIGHT {
            state.set(GameState::GameOver);
        }
    }
}

fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: ResMut<Score>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = score.0.to_string();
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn reset_timer(mut timer: ResMut<PipeSpawnTimer>) {
    timer.0.reset();
}

fn flap_sound(audio_handles: Res<AudioHandles>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(audio_handles.flap.clone()));
}

fn hit_sound(audio_handles: Res<AudioHandles>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(audio_handles.hit.clone()));
}

fn point_sound(audio_handles: Res<AudioHandles>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(audio_handles.point.clone()));
}
