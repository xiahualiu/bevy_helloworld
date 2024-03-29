use bevy::{app::AppExit, prelude::*};

use crate::state::GameState;
use crate::ui::assetloader::UiFont;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Spawn
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
        app.add_systems(
            Update,
            button_interaction.run_if(in_state(GameState::MainMenu)),
        );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

fn spawn_main_menu(mut commands: Commands, font_handle_res: Res<UiFont>) {
    // Main node
    commands
        .spawn((
            MainMenu,
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
                background_color: Color::ANTIQUE_WHITE.into(),
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
                    background_color: Color::ANTIQUE_WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Breakout"),
                                style: TextStyle {
                                    font: font_handle_res.0.clone(),
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
            // Spawn Play button
            parent
                .spawn((
                    PlayButton,
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
                                value: String::from("Play"),
                                style: TextStyle {
                                    font: font_handle_res.0.clone(),
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
                    QuitButton,
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
                                    font: font_handle_res.0.clone(),
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

fn despawn_main_menu(mut commands: Commands, window_query: Query<Entity, With<MainMenu>>) {
    let entity = window_query.get_single().unwrap();
    commands.entity(entity).despawn_recursive();
}

fn button_interaction(
    mut background_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&QuitButton>,
        ),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interact, mut backgroundcolor, is_playbutton, is_quitbutton) in &mut background_query {
        match interact {
            Interaction::Hovered => *backgroundcolor = Color::ALICE_BLUE.into(),
            Interaction::Pressed => {
                if let Some(_) = is_playbutton {
                    next_state.set(GameState::PrepGame);
                }
                if let Some(_) = is_quitbutton {
                    app_exit_writer.send(AppExit);
                }
            }
            Interaction::None => *backgroundcolor = Color::YELLOW_GREEN.into(),
        }
    }
}
