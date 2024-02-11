use bevy::prelude::*;

use crate::state::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Spawn 
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), spawn_main_menu);
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        background_color: Color::YELLOW_GREEN.into(),
        ..default()
    });
}

fn despawn_main_menu() {}