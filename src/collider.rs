use bevy::prelude::*;
use crate::brick::{Brick, BrickLevel};
use crate::ball::Ball;
use crate::schedule::InGameSet;

#[derive(Component)]
pub struct Collider;

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub collider_entity: Entity,
    pub ball_entity: Entity,
    pub normal_vec: Vec2
}

impl CollisionEvent {
    pub fn new(collider_entity: Entity, ball_entity: Entity, normal_vec: Vec2) -> CollisionEvent {
        CollisionEvent {
            collider_entity,
            ball_entity,
            normal_vec
        }
    }
}

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_systems(Update,
            (
                check_for_collision,
                handle_collision
            ).chain().in_set(InGameSet::CollisionDetection)
        );
    }
}

// Return normal vector
fn collision_with_square(ball_transform: &Transform, wall_transform: &Transform) -> Option<Vec2> {
    let wall_top = wall_transform.translation.y + wall_transform.scale.y / 2.0;
    let wall_bottom = wall_transform.translation.y - wall_transform.scale.y / 2.0;
    let wall_left = wall_transform.translation.x - wall_transform.scale.x / 2.0;
    let wall_right = wall_transform.translation.x + wall_transform.scale.x / 2.0;
    let ball_top = ball_transform.translation.y + ball_transform.scale.y / 2.0;
    let ball_bottom = ball_transform.translation.y - ball_transform.scale.y / 2.0;
    let ball_left = ball_transform.translation.x - ball_transform.scale.x / 2.0;
    let ball_right = ball_transform.translation.x + ball_transform.scale.x / 2.0;
    if ball_right < wall_left
        || ball_left > wall_right
        || ball_top < wall_bottom
        || ball_bottom > wall_top
    {
        return None;
    } else if ball_transform.translation.y > wall_top {
        // In the top strip
        if ball_transform.translation.x < wall_left {
            let dist_lt_vec = Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                - Vec2::new(wall_left, wall_top);
            if dist_lt_vec.length() < ball_transform.scale.x / 2.0 {
                return Some(dist_lt_vec);
            } else {
                return None;
            }
        } else if ball_transform.translation.x > wall_right {
            let dist_rt_vec = Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                - Vec2::new(wall_right, wall_top);
            if dist_rt_vec.length() < ball_transform.scale.x / 2.0 {
                return Some(dist_rt_vec);
            } else {
                return None;
            }
        } else {
            return Some(Vec2::new(0.0, 1.0));
        }
    } else if ball_transform.translation.y > wall_bottom {
        if ball_transform.translation.x < wall_left {
            return Some(Vec2::new(-1.0, 0.0));
        } else if ball_transform.translation.x > wall_right {
            return Some(Vec2::new(1.0, 0.0));
        } else {
            return None;
        }
    } else {
        // In the bottom strip
        if ball_transform.translation.x < wall_left {
            let dist_lb_vec = Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                - Vec2::new(wall_left, wall_bottom);
            if dist_lb_vec.length() < ball_transform.scale.x / 2.0 {
                return Some(dist_lb_vec);
            } else {
                return None;
            }
        } else if ball_transform.translation.x > wall_right {
            let dist_rb_vec = Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                - Vec2::new(wall_right, wall_bottom);
            if dist_rb_vec.length() < ball_transform.scale.x / 2.0 {
                return Some(dist_rb_vec);
            } else {
                return None;
            }
        } else {
            return Some(Vec2::new(0.0, -1.0));
        }
    }
}

pub fn check_for_collision(
    mut ball_query: Query<(Entity, &mut Ball, &Transform), With<Collider>>,
    mut collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    for (ball_entity, mut ball, ball_transform) in &mut ball_query {
        for (collider_entity, other_transform) in &mut collider_query {
            if ball.last_col_entity != collider_entity { // Prevent double collision
                if let Some(normal_vec)=collision_with_square(ball_transform, other_transform) {
                    collision_event_writer.send(CollisionEvent::new(
                        collider_entity,
                        ball_entity,
                        normal_vec.normalize())
                    );
                    ball.last_col_entity = collider_entity;
                }
            }
        }
    }
}

fn handle_collision(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut brick_query: Query<&mut Brick>,
    mut ball_query: Query<&mut Ball>
) {
    for &CollisionEvent {
        collider_entity,
        ball_entity,
        normal_vec
    } in collision_event_reader.read() {
        if let Ok(mut brick) = brick_query.get_mut(collider_entity) {
            match brick.level {
                BrickLevel::NONE => (),
                BrickLevel::LOW => brick.level=BrickLevel::NONE,
                BrickLevel::MID => brick.level=BrickLevel::LOW,
                BrickLevel::HIGH => brick.level=BrickLevel::MID,
                BrickLevel::SUPER => brick.level=BrickLevel::HIGH
            }
        };
        if let Ok(mut ball) = ball_query.get_mut(ball_entity) {
            ball.velocity = ball.velocity-2.0*normal_vec*normal_vec.dot(ball.velocity);
        };
    }
}

