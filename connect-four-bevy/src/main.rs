use bevy::prelude::*;
use clap::Parser;
use connect_four_lib::player::Player;
use connect_four_lib::web_socket::WsMsg;
use rust_socketio::client::Client as SocketIoClient;
use rust_socketio::Payload;
use std::sync::Arc;
use tokio::sync::Mutex;

mod board;
mod events;
mod game_logic;
mod ui;

use board::*;
use events::*;
use game_logic::*;
use ui::*;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Run offline (no websocket)
    #[arg(long)]
    offline: bool,
}

fn main() {
    let args = Args::parse();
    let (client, rx) = if !args.offline {
        // You will need to refactor create_socketio_client to be synchronous, or spawn a background thread.
        // For now, set to None, or use a placeholder for demonstration.
        (None, None)
    } else {
        (None, None)
    };
    // Track our player id
    let my_player = MyPlayerInfo {
        id: None,
        color: None,
    };
    App::new()
        .insert_resource(WsRxChannel(rx))
        .insert_resource(WsTxChannel(client))
        .insert_resource(MyPlayerInfo { ..my_player })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Connect Four".into(),
                resolution: (800.0, 700.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .init_resource::<GameScore>()
        .add_event::<PieceDropEvent>()
        .add_event::<GameResetEvent>()
        .add_event::<PieceAnimationComplete>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (setup_camera, setup_board, setup_ui))
        .add_systems(
            Update,
            (
                handle_input,
                handle_piece_drop,
                handle_game_reset,
                handle_reset_button,
                handle_keyboard_input,
                update_ui,
                ui::update_my_turn_indicator,
                animate_pieces,
                cleanup_pieces,
                handle_ws_messages,
            ),
        )
        .run();
}

fn handle_ws_messages(
    ws_rx: Res<WsRxChannel>,
    mut status_query: Query<
        &mut bevy::text::Text,
        (With<ui::GameStatusText>, Without<ui::CurrentPlayerText>),
    >,
    mut piece_drop_events: EventWriter<PieceDropEvent>,
    mut my_player: ResMut<MyPlayerInfo>,
) {
    if let Some(rx) = &ws_rx.0 {
        // NOTE: tokio mpsc Receiver does not implement Sync or Clone, so this is a workaround for demo only
        // In production, handle receiving logic in an async task and dispatch events into Bevy properly
        // Here, we simply try_recv in-place for the Option.
        let mut rx = rx;
        while let Ok(msg) = rx.try_recv() {
            match msg {
                WsMsg::PlayerJoin { id, color } => {
                    println!(
                        "[WS][DEBUG] Received PlayerJoin: id={} color={:?}",
                        id, color
                    );
                    my_player.id.get_or_insert(id.clone());
                    my_player.color = Some(color);
                    if let Ok(mut text) = status_query.get_single_mut() {
                        text.sections[0].value = format!("Player joined: {id}, Color: {:?}", color);
                        text.sections[0].style.color = bevy::prelude::Color::GREEN;
                    }
                }
                WsMsg::PlayerLeave { id } => {
                    if let Ok(mut text) = status_query.get_single_mut() {
                        text.sections[0].value = format!("Player left: {id}");
                        text.sections[0].style.color = bevy::prelude::Color::RED;
                    }
                }
                WsMsg::PlayerMove { id: _, col } => {
                    piece_drop_events.send(PieceDropEvent { column: col });
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSide {
    Me,
    Other,
}

#[derive(Resource)]
pub struct WsRxChannel(pub Option<tokio::sync::mpsc::Receiver<WsMsg>>);

#[derive(Resource, Clone, Default)]
pub struct MyPlayerInfo {
    pub id: Option<String>,
    pub color: Option<Player>,
}

#[derive(Resource, Clone)]
pub struct WsTxChannel(pub Option<Arc<SocketIoClient>>);

async fn create_socketio_client() -> (Arc<SocketIoClient>, tokio::sync::mpsc::Receiver<WsMsg>) {
    use rust_socketio::ClientBuilder;
    use tokio::sync::mpsc;
    use uuid::Uuid;
    let (tx, rx) = mpsc::channel(32);
    let my_id = Uuid::new_v4().to_string();
    let tx = Arc::new(Mutex::new(tx));

    let client = ClientBuilder::new("http://localhost:3000")
        .on("joined", {
            let tx = tx.clone();
            move |payload, _| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Payload::Text(data) = payload {
                        if let Ok(msg) = serde_json::from_str::<WsMsg>(&data) {
                            let _ = tx.lock().await.send(msg).await;
                        }
                    }
                });
            }
        })
        .on("leave", {
            let tx = tx.clone();
            move |payload, _| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Payload::Text(data) = payload {
                        if let Ok(msg) = serde_json::from_str::<WsMsg>(&data) {
                            let _ = tx.lock().await.send(msg).await;
                        }
                    }
                });
            }
        })
        .on("move", {
            let tx = tx.clone();
            move |payload, _| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Payload::Text(data) = payload {
                        if let Ok(msg) = serde_json::from_str::<WsMsg>(&data) {
                            let _ = tx.lock().await.send(msg).await;
                        }
                    }
                });
            }
        })
        .connect()
        .expect("Failed to connect to socketio server");

    // Send join message after connecting
    let join_msg = WsMsg::PlayerJoin {
        id: my_id,
        color: Player::One,
    };
    let json = serde_json::to_string(&join_msg).unwrap();
    client.emit("join", json).expect("Failed to emit join");

    (Arc::new(client), rx)
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
