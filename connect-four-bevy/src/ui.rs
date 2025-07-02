use crate::{events::*, game_logic::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct MyTurnIndicator;

#[derive(Resource, Default)]
pub struct GameScore {
    pub red_wins: u32,
    pub yellow_wins: u32,
    pub draws: u32,
}

#[derive(Component)]
pub struct GameStatusText;
#[derive(Component)]
pub struct CurrentPlayerText;
#[derive(Component)]
pub struct ScoreText;
#[derive(Component)]
pub struct ResetButton;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root UI node for layout
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            // Top indicator
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Your turn!",
                    TextStyle {
                        font: asset_server.load("default_font.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            }).insert(MyTurnIndicator);

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            });

            // Bottom reset button
            parent.spawn(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: Color::BLUE.into(),
                ..Default::default()
            })
            .insert(ResetButton)
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Reset",
                        TextStyle {
                            font: asset_server.load("default_font.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
            });
        });
}


pub fn handle_game_reset(
    _game_state: ResMut<GameState>,
    _score: ResMut<GameScore>,
    _reset_events: EventReader<GameResetEvent>,
) {
    // existing implementation
}

pub fn handle_reset_button(/* params */) {
    // existing implementation
}

pub fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut piece_drop_events: EventWriter<PieceDropEvent>,
    _reset_events: EventWriter<GameResetEvent>,
    game_state: Res<GameState>,
    ws_tx: Res<crate::WsTxChannel>,
    my_player: Res<crate::MyPlayerInfo>,
) {
    let key_to_column = [
        (KeyCode::Key1, 0),
        (KeyCode::Key2, 1),
        (KeyCode::Key3, 2),
        (KeyCode::Key4, 3),
        (KeyCode::Key5, 4),
        (KeyCode::Key6, 5),
        (KeyCode::Key7, 6),
    ];
    let is_my_turn = match my_player.color {
        Some(crate::game_logic::Player::One) => {
            game_state.current_player == crate::game_logic::Player::One
        }
        Some(crate::game_logic::Player::Two) => {
            game_state.current_player == crate::game_logic::Player::Two
        }
        _ => false,
    };
    for (key, column) in key_to_column.iter() {
        if keyboard_input.just_pressed(*key)
            && game_state.status == GameStatus::Playing
            && !game_state.is_column_full(*column)
            && is_my_turn
        {
            piece_drop_events.send(PieceDropEvent { column: *column });
            if let Some(tx) = &ws_tx.0 {
                let _ = tx.send(crate::WsMsg::PlayerMove { col: *column });
            }
        }
    }
}

pub fn update_ui(/* params */) {
    // existing implementation
}

// ... rest of previous content ...

pub fn update_my_turn_indicator(
    game_state: Res<GameState>,
    my_player: Res<crate::MyPlayerInfo>,
    mut q: Query<&mut Text, With<MyTurnIndicator>>,
) {
    let is_my_turn = match my_player.color {
        Some(Player::One) => game_state.current_player == Player::One,
        Some(Player::Two) => game_state.current_player == Player::Two,
        _ => false,
    };
    if let Ok(mut text) = q.get_single_mut() {
        if is_my_turn {
            text.sections[0].value = "Your turn!".to_owned();
        } else {
            text.sections[0].value = "Waiting...".to_owned();
        }
    }
}
