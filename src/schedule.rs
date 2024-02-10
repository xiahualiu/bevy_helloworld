use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InGameSet {
    UserInput,
    CollisionDetection,
    EntityUpdates,
    DespawnEntities
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
                InGameSet::DespawnEntities,
            ).chain()
        );
    }
}