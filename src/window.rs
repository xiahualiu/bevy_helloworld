use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use crate::ui::scoreboard;
use crate::wall;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_setup);
    }
}

fn window_setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set windows size
    let mut window = windows.single_mut();
    window.resolution.set(
        wall::RIGHT_WALL - wall::LEFT_WALL + wall::WALL_THICKNESS,
        wall::TOP_WALL - wall::BOTTOM_WALL + wall::WALL_THICKNESS + scoreboard::SCOREBOARD_HEIGHT,
    );
    window.title = String::from("Breakout! v1.0 made by Fried Rice");

    // Set camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
            ..default()
        },
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: scoreboard::SCOREBOARD_HEIGHT / 2.0,
                z: 0.0,
            },
            ..default()
        },
        ..default()
    });
}
