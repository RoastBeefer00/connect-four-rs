use crate::events::*;
use crate::game_logic::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct CurrentPlayerText;

#[derive(Component)]
pub struct GameStatusText;

#[derive(Component)]
pub struct ResetButton;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct GameScore {
    pub red_wins: u32,
    pub yellow_wins: u32,
    pub draws: u32,
}

impl Default for GameScore {
    fn default() -> Self {
        Self {
            red_wins: 0,
            yellow_wins: 0,
            draws: 0,
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
    // UI Root
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top UI Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(25.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Current Player Display
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Current Player: ",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));

                            parent.spawn((
                                TextBundle::from_section(
                                    "Red",
                                    TextStyle {
                                        font_size: 24.0,
                                        color: Player::Red.color(),
                                        ..default()
                                    },
                                ),
                                CurrentPlayerText,
                            ));
                        });

                    // Game Title
                    parent.spawn(TextBundle::from_section(
                        "CONNECT FOUR",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Score Display
                    parent.spawn((
                        TextBundle::from_section(
                            "Red: 0 | Yellow: 0 | Draws: 0",
                            TextStyle {
                                font_size: 18.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ScoreText,
                    ));
                });

            // Bottom UI Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Game Status Text
                    parent.spawn((
                        TextBundle::from_section(
                            "Click a column to drop your piece!",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        GameStatusText,
                    ));

                    // Reset Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(150.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.3, 0.3, 0.7).into(),
                                ..default()
                            },
                            ResetButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

pub fn update_ui(
    game_state: Res<GameState>,
    score: Res<GameScore>,
    mut current_player_query: Query<&mut Text, With<CurrentPlayerText>>,
    mut status_query: Query<&mut Text, (With<GameStatusText>, Without<CurrentPlayerText>)>,
    mut score_query: Query<
        &mut Text,
        (
            With<ScoreText>,
            Without<CurrentPlayerText>,
            Without<GameStatusText>,
        ),
    >,
) {
    // Update current player display
    if let Ok(mut text) = current_player_query.get_single_mut() {
        let player_name = match game_state.current_player {
            Player::Red => "Red",
            Player::Yellow => "Yellow",
        };
        text.sections[0].value = player_name.to_string();
        text.sections[0].style.color = game_state.current_player.color();
    }

    // Update game status
    if let Ok(mut text) = status_query.get_single_mut() {
        text.sections[0].value = match game_state.status {
            GameStatus::Playing => "Click a column to drop your piece!".to_string(),
            GameStatus::Won(player) => {
                let player_name = match player {
                    Player::Red => "Red",
                    Player::Yellow => "Yellow",
                };
                format!("{} Player Wins! 🎉", player_name)
            }
            GameStatus::Draw => "It's a Draw! 🤝".to_string(),
        };

        text.sections[0].style.color = match game_state.status {
            GameStatus::Playing => Color::WHITE,
            GameStatus::Won(player) => player.color(),
            GameStatus::Draw => Color::YELLOW,
        };
    }

    // Update score display
    if let Ok(mut text) = score_query.get_single_mut() {
        text.sections[0].value = format!(
            "Red: {} | Yellow: {} | Draws: {}",
            score.red_wins, score.yellow_wins, score.draws
        );
    }
}

pub fn handle_reset_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut reset_events: EventWriter<GameResetEvent>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.2, 0.2, 0.5).into();
                reset_events.send(GameResetEvent);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.4, 0.4, 0.8).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.3, 0.3, 0.7).into();
            }
        }
    }
}

pub fn handle_game_reset(
    mut game_state: ResMut<GameState>,
    mut score: ResMut<GameScore>,
    mut reset_events: EventReader<GameResetEvent>,
) {
    for _ in reset_events.read() {
        // Update score based on previous game result
        match game_state.status {
            GameStatus::Won(Player::Red) => score.red_wins += 1,
            GameStatus::Won(Player::Yellow) => score.yellow_wins += 1,
            GameStatus::Draw => score.draws += 1,
            GameStatus::Playing => {} // Game was reset before finishing
        }

        game_state.reset();
    }
}

// Add keyboard shortcuts
pub fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut piece_drop_events: EventWriter<PieceDropEvent>,
    mut reset_events: EventWriter<GameResetEvent>,
    game_state: Res<GameState>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        reset_events.send(GameResetEvent);
    }

    // Number keys 1-7 for dropping pieces
    let key_to_column = [
        (KeyCode::Key1, 0),
        (KeyCode::Key2, 1),
        (KeyCode::Key3, 2),
        (KeyCode::Key4, 3),
        (KeyCode::Key5, 4),
        (KeyCode::Key6, 5),
        (KeyCode::Key7, 6),
    ];

    for (key, column) in key_to_column.iter() {
        if keyboard_input.just_pressed(*key) {
            if game_state.status == GameStatus::Playing && !game_state.is_column_full(*column) {
                piece_drop_events.send(PieceDropEvent { column: *column });
            }
        }
    }
}
