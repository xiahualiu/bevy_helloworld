use bevy::prelude::*;
use crate::collider;

pub const WALL_THICKNESS: f32 = 10.0;
// x coordinates
pub const LEFT_WALL: f32 = -360.0;
pub const RIGHT_WALL: f32 = 360.0;
// y coordinates
pub const BOTTOM_WALL: f32 = -360.0;
pub const TOP_WALL: f32 = 360.0;
// Wall color
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub enum WallLocation {
    Left,
    Right,
    Top
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_wall);
    }
}

impl WallLocation {
    // Return the translation vec for the wall sprite
    pub fn position(&self) -> Vec3 {
        match self {
            WallLocation::Left => Vec3 {x:LEFT_WALL+WALL_THICKNESS/2.0, y:1.0, z:0.0},
            WallLocation::Right => Vec3 {x:RIGHT_WALL-WALL_THICKNESS/2.0, y:0.0, z:0.0},
            WallLocation::Top  => Vec3 {x:0.0, y:TOP_WALL-WALL_THICKNESS/2.0, z: 0.0},
        }
    }
    // Return the size of the given wall sprite
    pub fn size(&self) -> Vec3 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        match self {
            WallLocation::Left => Vec3 { x: WALL_THICKNESS, y: arena_height, z: 1.0 },
            WallLocation::Right => Vec3 { x: WALL_THICKNESS, y: arena_height, z: 1.0 },
            WallLocation::Top => Vec3 { x: arena_width, y: WALL_THICKNESS, z: 1.0 },
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: collider::Collider,
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(),
                    scale: location.size(),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: collider::Collider,
        }
    }
}

fn spawn_wall(mut commands: Commands) {
    // Setup boundary walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
}