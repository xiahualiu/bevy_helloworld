use bevy::prelude::*;

use crate::state::GameState;

#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadAsset), asset_load);
    }
}

fn asset_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(UiFont(asset_server.load("fonts/FiraSans-Bold.ttf")));
    next_state.set(GameState::MainMenu);
}
