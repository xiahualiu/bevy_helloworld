use bevy::prelude::*;

use brick::BrickPlugin;
use collider::ColliderPlugin;
use schedule::SchedulePlugin;
use ball::BallPlugin;
use wall::WallPlugin;
use paddle::PaddlePlugin;
use window::WindowPlugin;

mod ball;
mod brick;
mod wall;
mod paddle;
mod collider;
mod schedule;
mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WindowPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(ColliderPlugin)
        .run();
}