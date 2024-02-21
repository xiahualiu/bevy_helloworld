use bevy::{app::AppExit, prelude::*};

use crate::state::GameState;
use crate::ui::scoreboard::GameScore;

pub struct EndMenuPlugin;

impl Plugin for EndMenuPlugin {
    fn build(&self, app: &mut App) {
        // Spawn
        app.add_systems(OnEnter(GameState::EndMenu), spawn_end_menu);
        app.add_systems(OnExit(GameState::EndMenu), despawn_end_menu);
        app.add_systems(
            Update,
            button_interaction.run_if(in_state(GameState::EndMenu)),
        );
    }
}

#[derive(Component)]
struct EndMenu;

#[derive(Component)]
struct Button {
    button_type: ButtonType,
}

enum ButtonType {
    RestartButton,
    MainMenuButton,
    QuitButton,
}

fn spawn_end_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_score: Res<GameScore>,
) {
    // Main node
    commands
        .spawn((
            EndMenu,
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
            // Spawn
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    visibility: Visibility::Visible,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Your Score: ") + &game_score.score.to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 80.0,
                                    color: Color::GRAY,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Spawn Restart button
            parent
                .spawn((
                    Button {
                        button_type: ButtonType::RestartButton,
                    },
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
                                value: String::from("Restart"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::BLUE,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Spawn MainMenu button
            parent
                .spawn((
                    Button {
                        button_type: ButtonType::MainMenuButton,
                    },
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
                                value: String::from("Main Menu"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::BLUE,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Spawn Quit button
            parent
                .spawn((
                    Button {
                        button_type: ButtonType::QuitButton,
                    },
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
                                value: String::from("Quit"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::BLUE,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn despawn_end_menu(mut commands: Commands, window_query: Query<Entity, With<EndMenu>>) {
    let entity = window_query.get_single().unwrap();
    commands.entity(entity).despawn_recursive();
}

fn button_interaction(
    mut background_query: Query<
        (&Interaction, &mut BackgroundColor, &Button),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interact, mut backgroundcolor, button) in &mut background_query {
        match interact {
            Interaction::Hovered => *backgroundcolor = Color::ALICE_BLUE.into(),
            Interaction::Pressed => match button.button_type {
                ButtonType::RestartButton => next_state.set(GameState::PrepGame),
                ButtonType::MainMenuButton => next_state.set(GameState::MainMenu),
                ButtonType::QuitButton => {
                    app_exit_writer.send(AppExit);
                }
            },
            Interaction::None => *backgroundcolor = Color::YELLOW_GREEN.into(),
        }
    }
}
