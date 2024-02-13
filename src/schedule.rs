use crate::state::GameState;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InGameSet {
    CheckGameStatus,
    UserInput,
    CollisionDetection,
    EntityUpdates,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::CheckGameStatus,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
                InGameSet::DespawnEntities,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}
