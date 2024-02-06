use bevy::prelude::*;
use crate::wall;

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_Y_OFFSET: f32 = -320.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle{
    paddle: Paddle,
    sprite: SpriteBundle,
}

impl PaddleBundle {
    pub fn new() -> PaddleBundle {
        PaddleBundle {
            paddle: Paddle,
            sprite: SpriteBundle {
                transform: Transform { 
                    translation: Vec3 { x: 0.0, y: PADDLE_Y_OFFSET, z: 0.0 },
                    scale: PADDLE_SIZE,
                    ..default()
                },
                sprite: Sprite { 
                    color: PADDLE_COLOR,
                    ..default()
                },
                ..default()
            }
        }
    }
}

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = wall::LEFT_WALL + wall::WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = wall::RIGHT_WALL - wall::WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}