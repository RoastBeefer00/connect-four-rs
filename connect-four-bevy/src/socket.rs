use std::time::Duration;

use bevy::{prelude::*, tasks::futures_lite::FutureExt};
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use connect_four_lib::web_socket::WsMsg;
use crossbeam_channel::{unbounded, Receiver, Sender};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
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

pub struct SocketIOPlugin {
    pub server_url: String,
}

impl Plugin for SocketIOPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = unbounded::<WsMsg>();

        app.add_plugins(TokioTasksPlugin::default())
            .insert_resource(SocketIOMessageSender(sender))
            .insert_resource(SocketIOMessageReceiver(receiver))
            .add_event::<SocketMessageEvent>()
            .add_systems(Startup, setup_socketio_client)
            .add_systems(
                Update,
                (handle_incoming_messages, update_other_players_system),
            );
    }
}

fn setup_socketio_client(
    _commands: Commands,
    runtime: ResMut<TokioTasksRuntime>,
    sender: Res<SocketIOMessageSender>,
    receiver: Res<SocketIOMessageReceiver>,
    // mut event_writer: EventWriter<SocketMessageEvent>,
) {
    let to_bevy_rx = sender.0.clone();
    let from_bevy_tx = receiver.0.clone();

    runtime.spawn_background_task(move |_ctx| async move {
        let move_callback = move |payload: Payload, _socket: Client| {
            let sender_clone = to_bevy_rx.clone();
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
            .on("move", move_callback) // Define a handler for the "message" event
            .connect()
            .await
            .expect("Connection failed");

        info!("Socket.IO connected!");

        // Main loop for the socket.io client
        loop {
            // Send messages from Bevy to the server
            if let Ok(msg) = from_bevy_tx.try_recv() {
                socket
                    .emit("message", serde_json::json!({ "data": msg }))
                    .await
                    .expect("Server unreachable");
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });
}

fn handle_incoming_messages(
    receiver: Res<SocketIOMessageReceiver>,
    mut event_writer: EventWriter<SocketMessageEvent>,
) {
    while let Ok(msg) = receiver.0.try_recv() {
        event_writer.write(SocketMessageEvent(msg));
    }
}

fn update_other_players_system(mut events: EventReader<SocketMessageEvent>) {
    for event in events.read() {
        match &event.0 {
            WsMsg::PlayerJoin { id, color } => {
                info!("Player {} has joined as color {:?}", id, color);
            }
            WsMsg::PlayerLeave { id } => {
                info!("Player {} has left", id);
            }
            WsMsg::PlayerMove { id, col } => {
                info!("Player {} has made a move on column {:?}", id, col);
            }
        }
    }
}
