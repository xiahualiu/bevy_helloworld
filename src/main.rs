use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use paddle::{move_paddle, PaddleBundle};
use wall::WallBundle;
use wall::WallLocation;
use ball::{move_ball, BallBundle};

// Add modules
mod ball;
mod brick;
mod wall;
mod paddle;
mod collider;
mod scoreboard;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn setup(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>)
{
    // Set windows size
    let mut window = windows.single_mut();
    window.resolution.set(wall::RIGHT_WALL-wall::LEFT_WALL, wall::TOP_WALL-wall::BOTTOM_WALL);

    // Set camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR)},
        ..default()
    });

    // Setup boundary walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));

    // Setup ball
    commands.spawn(BallBundle::new(meshes, materials));

    // Setup paddle
    commands.spawn(PaddleBundle::new());
} 


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddle)
        .add_systems(Update, move_ball)
        .run();
}