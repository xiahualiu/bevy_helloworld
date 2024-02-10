use bevy::prelude::*;
use crate::schedule::InGameSet;
use crate::wall;
use crate::collider::Collider;

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_Y_OFFSET: f32 = -320.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_PADDING: f32 = 10.0;
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddle)
            .add_systems(Update, handle_input.in_set(InGameSet::UserInput))
            .add_systems(Update, update_paddle.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component)]
pub struct Paddle {
    direction: f32,
}

#[derive(Bundle)]
pub struct PaddleBundle{
    paddle: Paddle,
    sprite: SpriteBundle,
    collider: Collider,
}

impl PaddleBundle {
    pub fn new() -> PaddleBundle {
        PaddleBundle {
            paddle: Paddle {direction: 0.0},
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
            },
            collider: Collider
        }
    }
}

fn spawn_paddle(mut commands: Commands) {
        commands.spawn(PaddleBundle::new());
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Paddle>,
) {
    let mut paddle = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        paddle.direction = -1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        paddle.direction = 1.0;
    }
    if !keyboard_input.any_pressed([KeyCode::Left, KeyCode::Right]) {
        paddle.direction = 0.0;
    }
}

fn update_paddle(
    mut query: Query<(&mut Transform, &Paddle)>,
    time: Res<Time>,
) {
    let (mut transform, paddle) = query.single_mut();
    let new_paddle_position = transform.translation.x + paddle.direction * PADDLE_SPEED * time.delta_seconds();
    let left_bound = wall::LEFT_WALL + wall::WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = wall::RIGHT_WALL - wall::WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;
    transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}