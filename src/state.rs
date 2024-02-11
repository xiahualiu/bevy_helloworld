use bevy::prelude::*;

#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    EndMenu
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
        app.add_systems(Update, StatePlugin::in_game_input_events);
    }
}

impl StatePlugin {
    fn in_game_input_events(
        mut next_state: ResMut<NextState<GameState>>,
        state: Res<State<GameState>>,
        keyboard_input: Res<Input<KeyCode>>
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            match state.get() {
                GameState::InGame => next_state.set(GameState::Paused),
                GameState::Paused => next_state.set(GameState::InGame),
                _ => ()
            }
        }
    }
}