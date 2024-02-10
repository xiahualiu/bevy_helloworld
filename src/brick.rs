use bevy::prelude::*;
use crate::collider::Collider;
use crate::schedule::InGameSet;

// Brick spawn parameters
const BRICK_SIZE: Vec3 = Vec3::new(100., 30., 1.0);
pub const FIRST_ROW_BRICK_Y: f32 = 300.0;
pub const GAP_BETWEEN_BRICK_ROW: f32 = 60.0;
pub const GAP_BETWEEN_BRICK_COL: f32 = 120.0;

// Brick colors
pub const BRICK_LOW_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const BRICK_MID_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const BRICK_HIGH_COLOR: Color = Color::rgb(0.8, 0.8, 0.0);
pub const BRICK_SUPER_COLOR: Color = Color::rgb(0.4, 0.4, 0.5);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_brick)
            .add_systems(Update, update_brick_color.in_set(InGameSet::CollisionDetection))
            .add_systems(Update, despawn_brick.in_set(InGameSet::DespawnEntities));
    }
}

// Spawn brick at start up
fn spawn_brick(mut commands: Commands) {
    // Setup brick
    for col in -2..3 {
        let x = col as f32*GAP_BETWEEN_BRICK_COL;
        let y = FIRST_ROW_BRICK_Y - GAP_BETWEEN_BRICK_ROW*0 as f32;
        let brick_location = Vec3 {x, y, z: 0.0};
        commands.spawn(BrickBundle::new(BrickLevel::SUPER, brick_location));
    }
    for col in -2..3 {
        let x = col as f32*GAP_BETWEEN_BRICK_COL;
        let y = FIRST_ROW_BRICK_Y - GAP_BETWEEN_BRICK_ROW*1 as f32;
        let brick_location = Vec3 {x, y, z: 0.0};
        commands.spawn(BrickBundle::new(BrickLevel::HIGH, brick_location));
    }
    for col in -2..3 {
        let x = col as f32*GAP_BETWEEN_BRICK_COL;
        let y = FIRST_ROW_BRICK_Y - GAP_BETWEEN_BRICK_ROW*2 as f32;
        let brick_location = Vec3 {x, y, z: 0.0};
        commands.spawn(BrickBundle::new(BrickLevel::MID, brick_location));
    }
    for col in -2..3 {
        let x = col as f32*GAP_BETWEEN_BRICK_COL;
        let y = FIRST_ROW_BRICK_Y - GAP_BETWEEN_BRICK_ROW*3 as f32;
        let brick_location = Vec3 {x, y, z: 0.0};
        commands.spawn(BrickBundle::new(BrickLevel::LOW, brick_location));
    }
}

fn update_brick_color(
    mut query: Query<(&Brick, &mut Sprite)>
) {
    for (brick, mut sprite) in &mut query {
        sprite.color = match brick.level {
            BrickLevel::NONE => BRICK_LOW_COLOR,
            BrickLevel::LOW => BRICK_LOW_COLOR,
            BrickLevel::MID => BRICK_MID_COLOR,
            BrickLevel::HIGH => BRICK_HIGH_COLOR,
            BrickLevel::SUPER => BRICK_SUPER_COLOR
        };
    }
}

// If a brick reach NONE level, remove it
fn despawn_brick(
    mut commands: Commands,
    mut query: Query<(Entity, &Brick)>
) {
    for (entity, brick) in &mut query {
        if let BrickLevel::NONE = brick.level {
            commands.entity(entity).despawn();
        }
    }
}

pub enum BrickLevel {
    NONE, // Will be deleted after update
    LOW,
    MID,
    HIGH,
    SUPER,
}

impl BrickLevel {
    pub fn get_color(&self) -> Color {
        match self {
            BrickLevel::NONE => BRICK_LOW_COLOR,
            BrickLevel::LOW => BRICK_LOW_COLOR,
            BrickLevel::MID => BRICK_MID_COLOR,
            BrickLevel::HIGH => BRICK_HIGH_COLOR,
            BrickLevel::SUPER => BRICK_SUPER_COLOR
        }
    }
}

#[derive(Component)]
pub struct Brick {
    pub level: BrickLevel
}

#[derive(Bundle)]
pub struct BrickBundle {
    brick: Brick,
    sprite: SpriteBundle,
    collider: Collider
}

impl BrickBundle {
    pub fn new(hp: BrickLevel, location: Vec3) -> BrickBundle {
        let color = hp.get_color();
        BrickBundle { 
            brick: Brick { level: hp }, 
            sprite: SpriteBundle {
                sprite: Sprite { 
                    color: color,
                    ..default() 
                },
                transform: Transform {
                    translation: location,
                    scale: BRICK_SIZE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider
        }
    }
}
