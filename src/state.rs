use crate::ball::Ball;
use crate::paddle::Paddle;
use bevy::prelude::*;

#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    LoadAsset,
    MainMenu,
    PrepGame,
    InGame,
    Paused,
    EndMenu,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_systems(Update, StatePlugin::in_game_input_events);
        app.add_systems(
            Update,
            StatePlugin::finish_prep.run_if(in_state(GameState::PrepGame)),
        );
    }
}

impl StatePlugin {
    fn in_game_input_events(
        mut next_state: ResMut<NextState<GameState>>,
        state: Res<State<GameState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            match state.get() {
                GameState::InGame => next_state.set(GameState::Paused),
                GameState::Paused => next_state.set(GameState::InGame),
                _ => (),
            }
        }
    }

    fn finish_prep(
        ball_query: Query<&Ball>,
        paddle_query: Query<&Paddle>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        if !ball_query.is_empty() && !paddle_query.is_empty() {
            game_state.set(GameState::InGame);
        }
    }
}
