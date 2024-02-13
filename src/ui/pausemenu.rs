use bevy::prelude::*;

use crate::state::GameState;
use crate::ui::assetloader::UiFont;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // Spawn
        app.add_systems(
            OnEnter(GameState::Paused),
            PauseMenuPlugin::spawn_pause_menu,
        )
        .add_systems(
            OnExit(GameState::Paused),
            PauseMenuPlugin::despawn_pause_menu,
        )
        .add_systems(
            Update,
            PauseMenuPlugin::button_interaction.run_if(in_state(GameState::Paused)),
        );
    }
}

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct QuitToMainButton;

impl PauseMenuPlugin {
    fn spawn_pause_menu(mut commands: Commands, font_handle_res: Res<UiFont>) {
        // Main node
        commands
            .spawn((
                PauseMenu,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    visibility: Visibility::Visible,
                    background_color: Color::rgba(0.98, 0.92, 0.84, 0.3).into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                // Spawn Play button
                parent
                    .spawn((
                        ContinueButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(300.0),
                                height: Val::Px(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::YELLOW_GREEN.into(),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: String::from("Resume"),
                                    style: TextStyle {
                                        font: font_handle_res.0.clone(),
                                        font_size: 40.0,
                                        color: Color::BLUE,
                                    },
                                }],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    });
                // Spawn Quit button
                parent
                    .spawn((
                        QuitToMainButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(300.0),
                                height: Val::Px(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::YELLOW_GREEN.into(),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: String::from("Quit to Main Menu"),
                                    style: TextStyle {
                                        font: font_handle_res.0.clone(),
                                        font_size: 40.0,
                                        color: Color::BLUE,
                                    },
                                }],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
    }

    fn despawn_pause_menu(mut commands: Commands, window_query: Query<Entity, With<PauseMenu>>) {
        let entity = window_query.get_single().unwrap();
        commands.entity(entity).despawn_recursive();
    }

    fn button_interaction(
        mut background_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                Option<&ContinueButton>,
                Option<&QuitToMainButton>,
            ),
            Changed<Interaction>,
        >,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        for (interact, mut backgroundcolor, is_resumebutton, is_quitbutton) in &mut background_query
        {
            match interact {
                Interaction::Hovered => *backgroundcolor = Color::ALICE_BLUE.into(),
                Interaction::Pressed => {
                    if let Some(_) = is_resumebutton {
                        next_state.set(GameState::InGame)
                    }
                    if let Some(_) = is_quitbutton {
                        next_state.set(GameState::MainMenu)
                    }
                }
                Interaction::None => *backgroundcolor = Color::YELLOW_GREEN.into(),
            }
        }
    }
}
