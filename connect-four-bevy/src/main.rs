use bevy::prelude::*;
use clap::Parser;
use connect_four_lib::web_socket::WsMsg;
use rust_socketio::client::Client as SocketIoClient;
use rust_socketio::Payload;
use std::sync::Arc;
use tokio::sync::Mutex;

mod board;
mod events;
mod game_logic;
mod socket;
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
        .add_plugins(SocketIoPlugin {
            server_url: "http://127.0.0.1:3000".to_string(),
        })
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
            ),
        )
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSide {
    Me,
    Other,
}

#[derive(Resource, Clone, Default)]
pub struct MyPlayerInfo {
    pub id: Option<String>,
    pub color: Option<Player>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
