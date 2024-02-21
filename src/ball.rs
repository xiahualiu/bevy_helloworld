use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;

use crate::collider::Collider;
use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::wall;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(50.0, -200.0, 1.0);
pub const BALL_DIAMETER: f32 = 20.0;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            BallPlugin::move_ball.in_set(InGameSet::EntityUpdates),
        )
        .add_systems(OnEnter(GameState::PrepGame), BallPlugin::spawn_ball)
        .add_systems(OnEnter(GameState::MainMenu), BallPlugin::despawn_all_balls)
        .add_systems(OnExit(GameState::EndMenu), BallPlugin::despawn_all_balls)
        .add_systems(
            Update,
            BallPlugin::check_end_game.in_set(InGameSet::CheckGameStatus),
        )
        .add_systems(
            Update,
            BallPlugin::handle_ball_loss.in_set(InGameSet::DespawnEntities),
        );
    }
}

impl BallPlugin {
    fn move_ball(mut query: Query<(&mut Transform, &mut Ball)>, time: Res<Time>) {
        for (mut transform, ball) in &mut query {
            transform.translation.x += ball.velocity.x * time.delta_seconds();
            transform.translation.y += ball.velocity.y * time.delta_seconds();
        }
    }

    // Spawn a ball
    fn spawn_ball(
        mut commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn(BallBundle::new(meshes, materials));
    }

    // Despawn all balls
    fn despawn_all_balls(mut commands: Commands, query: Query<Entity, With<Ball>>) {
        for ball_entity in &query {
            commands.entity(ball_entity).despawn();
        }
    }

    fn handle_ball_loss(
        mut commands: Commands,
        ball_query: Query<(Entity, &Transform), With<Ball>>,
    ) {
        for (ball_entity, transform) in &ball_query {
            if transform.translation.y > wall::TOP_WALL
                || transform.translation.y < wall::BOTTOM_WALL
                || transform.translation.x < wall::LEFT_WALL
                || transform.translation.x > wall::RIGHT_WALL
            {
                commands.entity(ball_entity).despawn();
            }
        }
    }

    fn check_end_game(ball_query: Query<&Ball>, mut next_state: ResMut<NextState<GameState>>) {
        if ball_query.is_empty() {
            next_state.set(GameState::EndMenu);
        }
    }
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub last_col_entity: Entity,
}

#[derive(Bundle)]
struct BallBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    ball: Ball,
    collider: Collider,
}

impl BallBundle {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> BallBundle {
        BallBundle {
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_DIAMETER/2.0))),
                material: materials.add(ColorMaterial {
                    color: BALL_COLOR,
                    ..Default::default()
                }),
                transform: Transform::from_translation(BALL_STARTING_POSITION),
                ..default()
            },
            ball: Ball {
                velocity: INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED,
                last_col_entity: Entity::PLACEHOLDER,
            },
            collider: Collider,
        }
    }
}
