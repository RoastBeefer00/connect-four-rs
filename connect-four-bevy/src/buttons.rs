use bevy::prelude::*;
use connect_four_lib::{player::Player, web_socket::WsMsg};

use crate::{socket::SendToServerEvent, MyPlayerInfo};

#[derive(Component)]
pub struct SurrenderButton;

#[derive(Component)]
pub struct NewGameButton;

pub fn surrender_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SurrenderButton>)>,
    my_player: Res<MyPlayerInfo>,
    mut send_to_server_event: EventWriter<SendToServerEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(player) = my_player.color {
                let player = Player::from(player);
                if player != Player::Spectator {
                    send_to_server_event
                        .write(SendToServerEvent(WsMsg::ClientSurrender { player }));
                }
            }
        }
    }
}

pub fn new_game_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut send_to_server_event: EventWriter<SendToServerEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            send_to_server_event.write(SendToServerEvent(WsMsg::NewGame));
        }
    }
}
