use crate::{game_logic::*, socket::SendToServerEvent, MyPlayerInfo};
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
#[derive(Component)]
pub struct NewGameButton;

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
                justify_content: JustifyContent::FlexStart,
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
                    flex_grow: 0.9,
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
                        padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .insert(ResetButton)
                .with_children(|button| {
                    button.spawn((
                        Text::new("Surrender"),
                        TextFont {
                            // font: asset_server.load("default_font.ttf"),
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
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
        if let GameStatus::Won(winner) = game_state.status {
            **text = format!("{} wins!", winner);
        } else if my_player.color == Some(Player::Spectator) {
            **text = "Spectating...".to_owned();
        } else if is_my_turn {
            **text = "Your turn!".to_owned();
        } else {
            **text = "Waiting...".to_owned();
        }
    }
}
