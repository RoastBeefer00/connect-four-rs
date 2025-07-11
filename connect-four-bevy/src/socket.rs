// use std::time::Duration;

use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use connect_four_lib::web_socket::WsMsg;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use crate::{
    events::{ChangePlayerEvent, PieceDropEvent},
    game_logic::{GameState, GameStatus, Player},
    ui::setup_ui,
    MyPlayerInfo,
};

// A resource to hold the sender end of our channel for outbound messages.
// pub struct WebSocketChannel {
//     sender: Sender<WsMsg>,
//     receiver: Receiver<WsMsg>,
// }
#[derive(Resource)]
pub struct SocketMessageSender(pub Sender<WsMsg>);

// A resource to hold the receiver end of our channel for inbound messages.
#[derive(Resource)]
pub struct SocketMessageReceiver(pub Receiver<WsMsg>);

// A custom Bevy event to represent a message received from the server.
#[derive(Event)]
pub struct SocketMessageEvent(pub WsMsg);
//
// Event for sending messages to server
#[derive(Event)]
pub struct SendToServerEvent(pub WsMsg);

pub struct SocketIOPlugin;

impl Plugin for SocketIOPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SocketMessageEvent>()
            .add_event::<SendToServerEvent>()
            .add_systems(Startup, setup_socketio_client.before(setup_ui))
            .add_systems(
                Update,
                (
                    receive_messages_from_server,
                    handle_server_messages,
                    send_messages_to_server,
                ),
            );
    }
}

fn setup_socketio_client(
    mut commands: Commands,
    player: Res<MyPlayerInfo>,
    mut sender: EventWriter<SendToServerEvent>,
) {
    // Create channels for communication
    let (outbound_sender, outbound_receiver) = async_channel::unbounded();
    let (inbound_sender, inbound_receiver) = async_channel::unbounded();

    // Add resources to Bevy
    commands.insert_resource(SocketMessageSender(outbound_sender));
    commands.insert_resource(SocketMessageReceiver(inbound_receiver));

    spawn_local(async move {
        info!("starting websocket connection");
        let ws = WebSocket::open("ws://127.0.0.1:3000").unwrap();
        info!("successfully made websocket connection");

        let (mut write, mut read) = ws.split();
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if let Ok(msg) = serde_json::from_str(&text) {
                        info!("sending message to bevy {:?}", msg);
                        let _ = inbound_sender.send(msg).await;
                    }
                }
            }
        });

        spawn_local(async move {
            // Send messages from Bevy to the server
            while let Ok(msg) = outbound_receiver.try_recv() {
                info!("waiting for messages to send to server");
                // let message_type = match msg {
                //     WsMsg::ClientJoin { id: _ } => "join",
                //     WsMsg::PlayerLeave { id: _ } => "leave",
                //     WsMsg::ClientMove { id: _, col: _ } => "move",
                //     WsMsg::GameOver { winner: _ } => "gameover",
                //     _ => "client_doesnt_send_these",
                // };
                if let Ok(str_msg) = serde_json::to_string(&msg) {
                    info!("sending message to server {:?}", msg);
                    let _ = write.send(Message::Text(str_msg)).await;
                }
            }
        });
        // tokio::time::sleep(Duration::from_millis(10)).await;
    });
    info!("writing join event");
    sender.write(SendToServerEvent(WsMsg::ClientJoin {
        id: player.id.to_string(),
    }));
}

// System to handle outbound messages (Bevy -> Server)
fn send_messages_to_server(
    mut events: EventReader<SendToServerEvent>,
    sender: Res<SocketMessageSender>,
) {
    for event in events.read() {
        // Send message through the channel to your SocketIO client
        if let Err(e) = sender.0.try_send(event.0.clone()) {
            warn!("Failed to send message to server: {}", e);
        }
    }
}

// System to handle inbound messages (Server -> Bevy)
fn receive_messages_from_server(
    receiver: Res<SocketMessageReceiver>,
    mut event_writer: EventWriter<SocketMessageEvent>,
) {
    while let Ok(msg) = receiver.0.try_recv() {
        event_writer.write(SocketMessageEvent(msg));
    }
}

fn handle_server_messages(
    mut socket_events: EventReader<SocketMessageEvent>,
    mut game_state: ResMut<GameState>,
    mut my_player: ResMut<MyPlayerInfo>,
    mut piece_event_writer: EventWriter<PieceDropEvent>,
    mut change_player_event_writer: EventWriter<ChangePlayerEvent>,
) {
    for event in socket_events.read() {
        match &event.0 {
            WsMsg::ServerJoin {
                id,
                client_player,
                active_player,
                game_board,
            } => {
                info!("Player {} has joined as color {:?}", id, client_player);
                game_state.get_state_from_lib(game_board);
                if my_player.id.to_string() == *id {
                    my_player.color = Some(client_player.into());
                }
                for (i, row) in game_state.board.iter().enumerate() {
                    for (j, col) in row.iter().enumerate() {
                        if let Some(piece) = col {
                            piece_event_writer.write(PieceDropEvent {
                                column: j,
                                row: i,
                                player: *piece,
                            });
                        }
                    }
                }
                game_state.current_player = active_player.into();
                game_state.status = GameStatus::Playing;
            }
            WsMsg::PlayerLeave { id } => {
                info!("Player {} has left", id);
            }
            WsMsg::ServerMove {
                id,
                col,
                row,
                active_player,
            } => {
                info!(
                    "Player {} has made a move on column {:?} and row {:?}",
                    id, col, row
                );
                let player: Player = Player::from(active_player);
                piece_event_writer.write(PieceDropEvent {
                    column: col.to_owned(),
                    row: row.to_owned(),
                    player,
                });
                change_player_event_writer.write(ChangePlayerEvent {
                    player: player.other().expect("unable to get other player"),
                });
            }
            WsMsg::GameOver { winner } => {
                let player = Player::from(winner);
                info!("Player {} wins the game!", player);
                game_state.status = GameStatus::Won(player);
            }
            _ => {}
        }
    }
}
