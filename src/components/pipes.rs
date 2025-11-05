use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use rand::{Rng, rng};

use crate::{
    BIRD_SIZE, PIPE_SIZE, PIPE_Z, Scroll,
    components::Bird,
    game::{GAP_HEIGHT, PlayState, Score},
    game_over::DespawnOnReset,
};

const PIPE_SPAWN_OFFSET: f32 = 180.0;
pub const PIPE_SPAWN_TIME: f32 = 1.2;

#[derive(Component)]
pub struct Pipe;

#[derive(Resource)]
pub struct PipeSpawnTimer(pub Timer);

#[derive(Component)]
pub struct ApproachingPipe;

pub fn spawn_pipe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
) {
    timer.0.tick(time.delta());

    if !timer.0.is_finished() {
        return;
    }

    let mut rng = rng();
    let y = rng.random_range(-50.0..50.0);

    let texture = asset_server.load::<Image>("sprite/pipe.png");

    commands.spawn((
        Sprite::from_image(texture.clone()),
        Pipe,
        ApproachingPipe,
        Scroll,
        DespawnOnReset,
        Transform::from_xyz(PIPE_SPAWN_OFFSET, y - 160.0, PIPE_Z),
    ));

    commands.spawn((
        Sprite {
            image: texture,
            flip_y: true,
            ..default()
        },
        Pipe,
        Scroll,
        DespawnOnReset,
        Transform::from_xyz(PIPE_SPAWN_OFFSET, y + 160.0 + GAP_HEIGHT, PIPE_Z),
    ));
}

// Despawn pipes that have moved off screen
pub fn despawn_pipe(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &query {
        if transform.translation.x < -PIPE_SPAWN_OFFSET {
            commands.entity(entity).despawn();
        }
    }
}

// Check if the bird has progressed passed a pipe and add to the score
pub fn check_passed_pipe(
    mut commands: Commands,
    mut score: ResMut<Score>,
    pipes: Query<(Entity, &Transform), With<ApproachingPipe>>,
    bird: Query<&Transform, With<Bird>>,
) {
    for (entity, pipe) in &pipes {
        if let Ok(bird) = bird.single()
            && pipe.translation.x + PIPE_SIZE.x / 2.0 < bird.translation.x - BIRD_SIZE.x / 2.0
        {
            commands.entity(entity).remove::<ApproachingPipe>();
            score.0 += 1;
            break;
        }
    }
}

// Check if the bird has collided with a pipe and end the game
pub fn check_pipe_collision(
    mut play_state: ResMut<NextState<PlayState>>,
    bird: Query<&Transform, With<Bird>>,
    pipes: Query<&Transform, With<Pipe>>,
) {
    for pipe in &pipes {
        if let Ok(bird) = bird.single() {
            let collision = Aabb2d::new(bird.translation.xy(), BIRD_SIZE / 2.00)
                .intersects(&Aabb2d::new(pipe.translation.xy(), PIPE_SIZE / 2.0));
            if collision {
                play_state.set(PlayState::HitPipe);
            }
        }
    }
}
