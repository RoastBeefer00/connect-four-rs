use crate::{events::*, game_logic::*, socket::SendToServerEvent, MyPlayerInfo};
use bevy::prelude::*;
use connect_four_lib::web_socket::WsMsg;

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

pub fn setup_ui(
    mut commands: Commands,
    player: Res<MyPlayerInfo>,
    mut sender: EventWriter<SendToServerEvent>,
) {
    // Root UI node for layout
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Top indicator
            parent
                .spawn((
                    Text::new("Your turn!"),
                    TextFont {
                        // font: asset_server.load("default_font.ttf"),
                        font_size: 30.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ))
                .insert(MyTurnIndicator);

            // Spacer
            parent.spawn((
                Node {
                    flex_grow: 1.0,
                    ..Default::default()
                },
                BackgroundColor(Color::NONE),
            ));

            // Bottom reset button
            parent
                .spawn((
                    Button,
                    Node {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .insert(ResetButton)
                .with_children(|button| {
                    button.spawn((
                        Text::new("Reset"),
                        TextFont {
                            // font: asset_server.load("default_font.ttf"),
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
    sender.write(SendToServerEvent(WsMsg::PlayerJoin {
        id: player.id.to_string(),
        color: connect_four_lib::player::Player::One,
    }));
}

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut piece_drop_events: EventWriter<PieceDropEvent>,
    _reset_events: EventWriter<GameResetEvent>,
    game_state: Res<GameState>,
    my_player: Res<crate::MyPlayerInfo>,
) {
    let key_to_column = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
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
            piece_drop_events.write(PieceDropEvent { column: *column });
        }
    }
}

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
    if let Ok(mut text) = q.single_mut() {
        if is_my_turn {
            **text = "Your turn!".to_owned();
        } else {
            **text = "Waiting...".to_owned();
        }
    }
}
