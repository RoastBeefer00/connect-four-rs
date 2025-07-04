use bevy::prelude::*;
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use connect_four_lib::web_socket::WsMsg;
use crossbeam_channel::{unbounded, Receiver, Sender};
use rust_socketio::{client::Client, ClientBuilder, Payload};

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
            .add_systems(Update, handle_incoming_messages);
    }
}

fn setup_socketio_client(
    mut commands: Commands,
    runtime: ResMut<TokioTasksRuntime>,
    sender: Res<SocketIOMessageSender>,
    receiver: Res<SocketIOMessageReceiver>,
) {
    let to_server_rx = sender.0.clone();
    let from_server_tx = receiver.0.clone();

    runtime.spawn_background_task(move |mut ctx| async move {
        let callback = |payload: Payload, socket: Client| {
            if let Payload::Text(values) = payload {
                for value in values {
                    from_server_tx.try_recv().unwrap();
                }
            }
        };

        let mut socket = ClientBuilder::new("http://127.0.0.1:3000") // Replace with your server URL
            .on("message", callback) // Define a handler for the "message" event
            .connect()
            .expect("Connection failed");

        info!("Socket.IO connected!");

        // Main loop for the socket.io client
        // loop {
        //     // Send messages from Bevy to the server
        //     if let Ok(msg) = to_server_rx.try_recv() {
        //         socket
        //             .emit("message", serde_json::json!({ "data": msg }))
        //             .expect("Server unreachable");
        //     }
        //     tokio::time::sleep(Duration::from_millis(10)).await;
        // }
    });
}

fn handle_incoming_messages(
    receiver: Res<SocketIOMessageReceiver>,
    mut event_writer: EventWriter<SocketMessageEvent>,
) {
    while let Ok(msg) = receiver.0.try_recv() {
        event_writer.send(SocketMessageEvent(msg));
    }
}

// fn send_player_position_system(
//     sender: Res<SocketIOMessageSender>,
//     query: Query<&Transform, With<Player>>,
// ) {
//     for transform in query.iter() {
//         let position_data = format!(
//             "{},{},{}",
//             transform.translation.x, transform.translation.y, transform.translation.z
//         );
//         sender.0.send(position_data).unwrap();
//     }
// }

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
