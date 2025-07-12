// use std::time::Duration;

use async_channel::{Receiver, Sender};
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy_tokio_tasks::TokioTasksRuntime;
use connect_four_lib::web_socket::WsMsg;
use futures::{SinkExt, StreamExt};
#[cfg(target_arch = "wasm32")]
use gloo_net::websocket::{futures::WebSocket, Message};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(not(target_arch = "wasm32"))]
pub use tokio_tungstenite::connect_async;

use crate::{
    events::{ChangePlayerEvent, PieceDropEvent},
    game_logic::{GameState, GameStatus, Player},
    ui::setup_ui,
    MyPlayerInfo,
};

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
        #[cfg(not(target_arch = "wasm32"))]
        app.add_event::<SocketMessageEvent>()
            .add_event::<SendToServerEvent>()
            .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .add_systems(Startup, setup_socketio_client.before(setup_ui))
            .add_systems(
                Update,
                (
                    receive_messages_from_server,
                    handle_server_messages,
                    send_messages_to_server,
                ),
            );

        #[cfg(target_arch = "wasm32")]
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
    #[cfg(not(target_arch = "wasm32"))] runtime: ResMut<TokioTasksRuntime>,
) {
    // Create channels for communication
    let (outbound_sender, outbound_receiver) = async_channel::unbounded();
    let (inbound_sender, inbound_receiver) = async_channel::unbounded();

    // Add resources to Bevy
    commands.insert_resource(SocketMessageSender(outbound_sender));
    commands.insert_resource(SocketMessageReceiver(inbound_receiver));

    #[cfg(target_arch = "wasm32")]
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
            while let Ok(msg) = outbound_receiver.recv().await {
                info!("waiting for messages to send to server");
                if let Ok(str_msg) = serde_json::to_string(&msg) {
                    info!("sending message to server {:?}", msg);
                    let _ = write.send(Message::Text(str_msg)).await;
                }
            }
        });
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        // use bevy_tokio_tasks::TokioTasksRuntime;
        // let runtime = TokioTasksRuntime::get();
        runtime.spawn_background_task(|_ctx| async move {
            info!("starting websocket connection");
            let (ws_stream, _) = connect_async("ws://127.0.0.1:3000")
                .await
                .expect("Failed to connect");
            info!("successfully made websocket connection");

            let (mut write, mut read) = ws_stream.split();
            let inbound_sender_clone = inbound_sender.clone();
            let read_task = tokio::spawn(async move {
                while let Some(msg) = read.next().await {
                    if let Ok(tokio_tungstenite::tungstenite::Message::Text(text)) = msg {
                        if let Ok(msg) = serde_json::from_str(&text) {
                            info!("sending message to bevy {:?}", msg);
                            let _ = inbound_sender_clone.send(msg).await;
                        }
                    }
                }
            });
            let write_task = tokio::spawn(async move {
                while let Ok(msg) = outbound_receiver.recv().await {
                    info!("waiting for messages to send to server");
                    if let Ok(str_msg) = serde_json::to_string(&msg) {
                        info!("sending message to server {:?}", msg);
                        let _ = write
                            .send(tokio_tungstenite::tungstenite::Message::Text(
                                str_msg.into(),
                            ))
                            .await;
                    }
                }
            });
            let _ = futures::future::join(read_task, write_task).await;
        });
    }
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
                if id == &my_player.id.to_string() {
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
