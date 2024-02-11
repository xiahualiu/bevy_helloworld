use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::{collider::Collider, schedule::InGameSet};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(50.0, -200.0, 1.0);
const BALL_DIAMETER: f32 = 20.0;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);

const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, move_ball.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub last_col_entity: Entity
}

#[derive(Bundle)]
struct BallBundle{
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    ball: Ball,
    collider: Collider
}

impl BallBundle {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>) -> BallBundle 
    {
        BallBundle {
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                material: materials.add(ColorMaterial { color: BALL_COLOR,..Default::default()}),
                transform: Transform::from_translation(BALL_STARTING_POSITION)
                    .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
                ..default()
            },
            ball: Ball {
                velocity: INITIAL_BALL_DIRECTION.normalize()*BALL_SPEED,
                last_col_entity: Entity::PLACEHOLDER,
            },
            collider: Collider,
        }
    }
}

fn move_ball(
    mut query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>) 
{
    for (mut transform, ball) in &mut query {
        transform.translation.x += ball.velocity.x * time.delta_seconds();
        transform.translation.y += ball.velocity.y * time.delta_seconds();
    }
}

// Spawn a ball
fn spawn_ball(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(BallBundle::new(meshes, materials));
}