use bevy::prelude::*;

const JUMP_AMOUNT: f32 = 1.5;
const FALL_SPEED: f32 = 5.0;
const FALL_VELOCITY_LIMIT: f32 = -2.0;
const MOVE_SPEED: f32 = 200.0;
const BIRD_ANIMATION_SPEED: f32 = 4.0;

#[derive(Component, Default)]
pub struct Bird {
    velocity: f32,
}

pub fn jump(mut bird: Query<(&mut Bird, &Transform)>, window: Query<&Window>) {
    for (mut bird, transform) in &mut bird {
        if let Ok(window) = window.single()
            && transform.translation.y + 300. < window.height()
        {
            bird.velocity = JUMP_AMOUNT;
        }
    }
}

pub fn fall(mut bird: Query<&mut Bird, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut bird {
        bird.velocity -= FALL_SPEED * time.delta_secs();
        bird.velocity = bird.velocity.max(FALL_VELOCITY_LIMIT);
    }
}

pub fn move_bird(mut bird: Query<(&mut Transform, &Bird), With<Bird>>, time: Res<Time>) {
    for (mut transform, bird) in &mut bird {
        transform.translation.y += bird.velocity * MOVE_SPEED * time.delta_secs();
    }
}

pub fn animate_bird(mut bird: Query<&mut Sprite, With<Bird>>, time: Res<Time>) {
    for mut sprite in &mut bird {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = (time.elapsed_secs() * BIRD_ANIMATION_SPEED) as usize % 4;
        }
    }
}
