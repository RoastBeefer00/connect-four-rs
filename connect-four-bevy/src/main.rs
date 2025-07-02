use bevy::prelude::*;
use clap::Parser;
use std::thread;

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
    let (ws_tx, ws_rx) = std::sync::mpsc::channel();
    let ws_rx_arc = Arc::new(Mutex::new(ws_rx));
    let mut ws_tx_channel = None;
    if !args.offline {
        let ws_tx2 = ws_tx.clone();
        thread::spawn(move || {
            ws_client(ws_tx2);
        });
        ws_tx_channel = Some(ws_tx);
    }
    // Track our player id
    let my_player = MyPlayerInfo {
        id: None,
        color: None,
    };
    App::new()
        .insert_resource(WsRxChannel(ws_rx_arc))
        .insert_resource(WsTxChannel(ws_tx_channel))
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
    if let Ok(rx) = ws_rx.0.lock() {
        while let Ok(msg) = rx.try_recv() {
            match msg {
                WsMsg::PlayerJoin { id, color } => {
                    println!("[WS][DEBUG] Received PlayerJoin: id={} color={:?}", id, color);
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
                WsMsg::PlayerMove { col } => {
                    piece_drop_events.send(PieceDropEvent { column: col });
                }
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WsMsg {
    PlayerJoin { id: String, color: Player },
    PlayerLeave { id: String },
    PlayerMove { col: usize }, // active player will be determined locally
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSide {
    Me,
    Other,
}

use std::sync::{mpsc::Receiver, Arc, Mutex};
#[derive(Resource, Clone)]
struct WsRxChannel(Arc<Mutex<Receiver<WsMsg>>>);

#[derive(Resource, Clone, Default)]
pub struct MyPlayerInfo {
    pub id: Option<String>,
    pub color: Option<crate::game_logic::Player>,
}

#[derive(Resource, Clone)]
pub struct WsTxChannel(pub Option<std::sync::mpsc::Sender<WsMsg>>);

fn ws_client(ws_tx: std::sync::mpsc::Sender<WsMsg>) {
    use uuid::Uuid;
    use ws::{connect, Message};
    connect("ws://localhost:3000/ws", |out| {
        println!("WebSocket connected!");
        // Generate random UUID for player id
        let my_id = Uuid::new_v4().to_string();
        // Dummy; server must fill color field
        let join_msg = WsMsg::PlayerJoin {
            id: my_id.clone(),
            color: crate::game_logic::Player::One,
        };
        let json = serde_json::to_string(&join_msg).unwrap();
        println!("Sending PlayerJoin: {}", json);
        out.send(Message::text(json)).ok();

        let ws_tx2 = ws_tx.clone();
        move |msg: ws::Message| {
            let text = msg.as_text()?;
            if let Ok(parsed) = serde_json::from_str::<WsMsg>(text) {
                ws_tx2.send(parsed).ok();
            } else {
                println!("Received unknown WS msg: {text}");
            }
            Ok(())
        }
    })
    .unwrap_or_else(|e| eprintln!("WS error: {e}"));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
