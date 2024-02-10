use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;

use crate::wall;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_setup);
    }
}

fn window_setup(
    mut commands: Commands,
    mut windows: Query<&mut Window>
)
{
    // Set windows size
    let mut window = windows.single_mut();
    window.resolution.set(wall::RIGHT_WALL-wall::LEFT_WALL, wall::TOP_WALL-wall::BOTTOM_WALL);

    // Set camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR)},
        ..default()
    });
} 