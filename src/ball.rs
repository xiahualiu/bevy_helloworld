use bevy::prelude::*;
use crate::collider::Collider;
use bevy::sprite::MaterialMesh2dBundle;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -300.0, 1.0);
pub const BALL_DIAMETER: f32 = 20.0;
pub const BALL_SPEED: f32 = 400.0;
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.0, 1.0);

pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
pub struct Ball {
    velocity: Vec2
}

#[derive(Bundle)]
pub struct BallBundle{
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
            ball: Ball {velocity: INITIAL_BALL_DIRECTION*BALL_SPEED},
            collider: Collider,
        }
    }
}

pub fn move_ball(
    mut query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>) 
{
    for (mut transform, ball) in &mut query {
        transform.translation.x += ball.velocity.x * time.delta_seconds();
        transform.translation.y += ball.velocity.y * time.delta_seconds();
    }
}