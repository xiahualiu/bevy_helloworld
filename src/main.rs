use bevy::prelude::*;

mod state;
mod ball;
mod brick;
mod wall;
mod paddle;
mod collider;
mod schedule;
mod window;
mod ui;

use brick::BrickPlugin;
use collider::ColliderPlugin;
use schedule::SchedulePlugin;
use ball::BallPlugin;
use state::StatePlugin;
use wall::WallPlugin;
use paddle::PaddlePlugin;
use window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StatePlugin)
        .add_plugins(WindowPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(ColliderPlugin)
        .run();
}