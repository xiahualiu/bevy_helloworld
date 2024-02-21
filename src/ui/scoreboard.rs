use bevy::prelude::*;

use crate::events::UpdateScoreEvent;
use crate::state::GameState;
use crate::ui::assetloader::UiFont;
use crate::wall;

pub const SCOREBOARD_HEIGHT: f32 = 50.0;
const SCOREBOARD_WIDTH: f32 = wall::RIGHT_WALL - wall::LEFT_WALL + wall::WALL_THICKNESS;

#[derive(Resource)]
pub struct GameScore {
    pub score: u32,
}

#[derive(Component)]
struct ScoreBoard;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScore { score: 0 })
            .add_systems(
                OnEnter(GameState::PrepGame),
                (
                    ScoreBoardPlugin::spawn_scoreboard,
                    ScoreBoardPlugin::reset_score,
                ),
            )
            .add_systems(
                Update,
                ScoreBoardPlugin::update_scoreboard.run_if(on_event::<UpdateScoreEvent>()),
            );
        // .add_systems(Update, ScoreBoardPlugin::update_scoreboard.run_if(on_event()));
    }
}

impl ScoreBoardPlugin {
    fn reset_score(mut game_score: ResMut<GameScore>) {
        game_score.score = 0;
    }

    fn update_scoreboard(
        mut board_query: Query<&mut Text, With<ScoreBoard>>,
        mut game_score: ResMut<GameScore>,
        mut update_event: EventReader<UpdateScoreEvent>,
        font_handle_res: Res<UiFont>,
    ) {
        for event in update_event.read() {
            game_score.score += event.score;
        }
        for mut text in &mut board_query {
            text.sections = vec![TextSection {
                value: String::from("Score: ")+&game_score.score.to_string(),
                style: TextStyle {
                    font: font_handle_res.0.clone(),
                    font_size: 30.0,
                    color: Color::ANTIQUE_WHITE,
                },
            }];
        }
    }

    fn spawn_scoreboard(
        mut commands: Commands,
        font_handle_res: Res<UiFont>,
    ) {
        // Window Node
        commands
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|parent| {
                // Viewport Node
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            width: Val::Px(
                                wall::RIGHT_WALL - wall::LEFT_WALL + wall::WALL_THICKNESS,
                            ),
                            height: Val::Px(
                                wall::TOP_WALL - wall::BOTTOM_WALL
                                    + wall::WALL_THICKNESS
                                    + SCOREBOARD_HEIGHT,
                            ),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Top Menu Node
                        parent.spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Px(SCOREBOARD_WIDTH),
                                height: Val::Px(SCOREBOARD_HEIGHT),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::GRAY),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ScoreBoard,
                                TextBundle {
                                    text: Text {
                                        sections: vec![TextSection {
                                            value: String::from("Score:0"),
                                            style: TextStyle {
                                                font: font_handle_res.0.clone(),
                                                font_size: 30.0,
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        }],
                                        justify: JustifyText::Center,
                                        ..default()
                                    },
                                    ..default()
                                }
                            ));
                        });
                    });
            });
    }
}
