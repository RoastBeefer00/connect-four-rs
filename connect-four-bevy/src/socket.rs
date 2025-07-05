use std::time::Duration;

use bevy::{prelude::*, tasks::futures_lite::FutureExt};
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use connect_four_lib::web_socket::WsMsg;
use crossbeam_channel::{unbounded, Receiver, Sender};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};

use crate::{
    events::PieceDropEvent,
    game_logic::{GameState, GameStatus, Player},
    MyPlayerInfo,
};

// A resource to hold the sender end of our channel for outbound messages.
#[derive(Resource)]
pub struct SocketIOMessageSender(pub Sender<WsMsg>);

// A resource to hold the receiver end of our channel for inbound messages.
#[derive(Resource)]
pub struct SocketIOMessageReceiver(pub Receiver<WsMsg>);

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
        app.add_plugins(TokioTasksPlugin::default())
            .add_event::<SocketMessageEvent>()
            .add_event::<SendToServerEvent>()
            .add_systems(Startup, setup_socketio_client)
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
    runtime: ResMut<TokioTasksRuntime>,
    // sender: Res<SocketIOMessageSender>,
    // receiver: Res<SocketIOMessageReceiver>,
    // mut event_writer: EventWriter<SocketMessageEvent>,
) {
    // Create channels for communication
    let (outbound_sender, outbound_receiver) = unbounded::<WsMsg>();
    let (inbound_sender, inbound_receiver) = unbounded::<WsMsg>();

    // Add resources to Bevy
    commands.insert_resource(SocketIOMessageSender(outbound_sender));
    commands.insert_resource(SocketIOMessageReceiver(inbound_receiver));

    runtime.spawn_background_task(move |_ctx| async move {
        let callback = move |_event: rust_socketio::Event, payload: Payload, _client: Client| {
            let sender_clone = inbound_sender.clone();
            async move {
                if let Payload::Text(values) = payload {
                    for value in values {
                        println!("received value {}", value);
                        let msg = serde_json::from_value::<WsMsg>(value)
                            .expect("unable to serialize message");
                        sender_clone.try_send(msg).unwrap();
                    }
                }
            }
            .boxed()
        };

        let socket = ClientBuilder::new("http://127.0.0.1:3000") // Replace with your server URL
            .on_any(callback) // Define a handler for the "message" event
            .connect()
            .await
            .expect("Connection failed");

        info!("Socket.IO connected!");

        // Main loop for the socket.io client
        loop {
            // Send messages from Bevy to the server
            if let Ok(msg) = outbound_receiver.try_recv() {
                let message_type = match msg {
                    WsMsg::PlayerJoin {
                        id: _,
                        color: _,
                        active_player: _,
                    } => "join",
                    WsMsg::PlayerLeave { id: _ } => "leave",
                    WsMsg::PlayerMove { id: _, col: _ } => "move",
                    WsMsg::GameOver { winner: _ } => "gameover",
                };
                socket
                    .emit(message_type, serde_json::to_string(&msg).unwrap())
                    .await
                    .expect("Server unreachable");
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });
}

// System to handle outbound messages (Bevy -> Server)
fn send_messages_to_server(
    mut events: EventReader<SendToServerEvent>,
    sender: Res<SocketIOMessageSender>,
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
    receiver: Res<SocketIOMessageReceiver>,
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
) {
    for event in socket_events.read() {
        match &event.0 {
            WsMsg::PlayerJoin {
                id,
                color,
                active_player,
            } => {
                info!("Player {} has joined as color {:?}", id, color);
                if my_player.id.to_string() == *id {
                    my_player.color = Some(color.into());
                }
                game_state.status = GameStatus::Playing;
                game_state.current_player = active_player.into();
            }
            WsMsg::PlayerLeave { id } => {
                info!("Player {} has left", id);
            }
            WsMsg::PlayerMove { id, col, row } => {
                info!("Player {} has made a move on column {:?}", id, col);
                piece_event_writer.write(PieceDropEvent {
                    column: col.to_owned(),
                    row: row.to_owned(),
                });
            }
            WsMsg::GameOver { winner } => {
                let player = Player::from(winner);
                info!("Player {} wins the game!", player);
                game_state.status = GameStatus::Won(player);
            }
        }
    }
}
