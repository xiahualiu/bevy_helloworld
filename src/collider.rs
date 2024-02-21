use bevy::prelude::*;

use crate::ball::{Ball, BALL_DIAMETER};
use crate::brick::{Brick, BrickLevel};
use crate::paddle::Paddle;
use crate::schedule::InGameSet;
use crate::wall::Wall;

#[derive(Component)]
pub struct Collider;
pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ColliderPlugin::check_for_collision.in_set(InGameSet::CollisionDetection),
        );
    }
}

impl ColliderPlugin {
    // Return normal vector
    fn collision_with_square(
        ball_transform: &Transform,
        wall_transform: &Transform,
    ) -> Option<Vec2> {
        let wall_top = wall_transform.translation.y + wall_transform.scale.y / 2.0;
        let wall_bottom = wall_transform.translation.y - wall_transform.scale.y / 2.0;
        let wall_left = wall_transform.translation.x - wall_transform.scale.x / 2.0;
        let wall_right = wall_transform.translation.x + wall_transform.scale.x / 2.0;
        let ball_top = ball_transform.translation.y + BALL_DIAMETER/ 2.0;
        let ball_bottom = ball_transform.translation.y - BALL_DIAMETER / 2.0;
        let ball_left = ball_transform.translation.x - BALL_DIAMETER / 2.0;
        let ball_right = ball_transform.translation.x + BALL_DIAMETER / 2.0;
        if ball_right < wall_left
            || ball_left > wall_right
            || ball_top < wall_bottom
            || ball_bottom > wall_top
        {
            return None;
        } else if ball_transform.translation.y > wall_top {
            // In the top strip
            if ball_transform.translation.x < wall_left {
                let dist_lt_vec =
                    Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                        - Vec2::new(wall_left, wall_top);
                if dist_lt_vec.length() < ball_transform.scale.x / 2.0 {
                    return Some(dist_lt_vec.normalize());
                } else {
                    return None;
                }
            } else if ball_transform.translation.x > wall_right {
                let dist_rt_vec =
                    Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                        - Vec2::new(wall_right, wall_top);
                if dist_rt_vec.length() < ball_transform.scale.x / 2.0 {
                    return Some(dist_rt_vec.normalize());
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
                let dist_lb_vec =
                    Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                        - Vec2::new(wall_left, wall_bottom);
                if dist_lb_vec.length() < ball_transform.scale.x / 2.0 {
                    return Some(dist_lb_vec.normalize());
                } else {
                    return None;
                }
            } else if ball_transform.translation.x > wall_right {
                let dist_rb_vec =
                    Vec2::new(ball_transform.translation.x, ball_transform.translation.y)
                        - Vec2::new(wall_right, wall_bottom);
                if dist_rb_vec.length() < ball_transform.scale.x / 2.0 {
                    return Some(dist_rb_vec.normalize());
                } else {
                    return None;
                }
            } else {
                return Some(Vec2::new(0.0, -1.0));
            }
        }
    }

    pub fn check_for_collision(
        mut ball_query: Query<(&mut Ball, &Transform)>,
        mut brick_query: Query<(&mut Brick, &Transform, Entity)>,
        paddle_query: Query<(&Paddle, &Transform, Entity)>,
        wall_query: Query<(&Wall, &Transform, Entity)>,
    ) {
        for (mut ball, ball_transform) in &mut ball_query {
            let mut found_collision_for_this_ball = false;
            for (mut brick, brick_transform, brick_entity) in &mut brick_query {
                if ball.last_col_entity != brick_entity {
                    // Prevent double collision
                    if let Some(normal_vec) =
                        ColliderPlugin::collision_with_square(ball_transform, brick_transform)
                    {
                        // Update brick health
                        match brick.level {
                            BrickLevel::NONE => (),
                            BrickLevel::LOW => brick.level = BrickLevel::NONE,
                            BrickLevel::MID => brick.level = BrickLevel::LOW,
                            BrickLevel::HIGH => brick.level = BrickLevel::MID,
                            BrickLevel::SUPER => brick.level = BrickLevel::HIGH,
                        };
                        // Update ball
                        ball.last_col_entity = brick_entity;
                        ball.velocity =
                            ball.velocity - 2.0 * normal_vec * normal_vec.dot(ball.velocity);
                        found_collision_for_this_ball = true;
                        break;
                    };
                }
            }
            if found_collision_for_this_ball {
                continue;
            }
            for (_, paddle_transform, paddle_entity) in &paddle_query {
                if ball.last_col_entity != paddle_entity {
                    // Prevent double collision
                    if let Some(normal_vec) =
                        ColliderPlugin::collision_with_square(ball_transform, paddle_transform)
                    {
                        ball.last_col_entity = paddle_entity;
                        ball.velocity =
                            ball.velocity - 2.0 * normal_vec * normal_vec.dot(ball.velocity);
                        found_collision_for_this_ball = true;
                        break;
                    };
                }
            }
            if found_collision_for_this_ball {
                continue;
            }
            for (_, wall_transform, wall_entity) in &wall_query {
                if ball.last_col_entity != wall_entity {
                    // Prevent double collision
                    if let Some(normal_vec) =
                        ColliderPlugin::collision_with_square(ball_transform, wall_transform)
                    {
                        ball.last_col_entity = wall_entity;
                        ball.velocity =
                            ball.velocity - 2.0 * normal_vec * normal_vec.dot(ball.velocity);
                        break;
                    };
                }
            }
        }
    }
}
