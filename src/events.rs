use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct UpdateScoreEvent {
    pub score: u32,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateScoreEvent>();
    }
}
