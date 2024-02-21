#![windows_subsystem = "windows"]
use bevy::prelude::*;

mod ball;
mod brick;
mod collider;
mod events;
mod paddle;
mod schedule;
mod state;
mod ui;
mod wall;
mod window;

use ball::BallPlugin;
use brick::BrickPlugin;
use collider::ColliderPlugin;
use events::EventPlugin;
use paddle::PaddlePlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use ui::{
    assetloader::AssetLoaderPlugin, endmenu::EndMenuPlugin, mainmenu::MainMenuPlugin,
    pausemenu::PauseMenuPlugin, scoreboard::ScoreBoardPlugin,
};
use wall::WallPlugin;
use window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EventPlugin)
        .add_plugins(WindowPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(EndMenuPlugin)
        .add_plugins(ScoreBoardPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(ColliderPlugin)
        .run();
}
