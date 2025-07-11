use bevy::prelude::*;
use clap::Parser;
use connect_four_lib::web_socket::WsMsg;
use socket::SocketIOPlugin;

mod board;
mod events;
mod game_logic;
mod socket;
mod ui;

use board::*;
use events::*;
use game_logic::*;
use ui::*;
use uuid::Uuid;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Run offline (no websocket)
    #[arg(long)]
    offline: bool,
}

fn main() {
    // let args = Args::parse();
    // Track our player id
    let my_player = MyPlayerInfo {
        id: Uuid::new_v4(),
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
        .add_plugins(SocketIOPlugin)
        .init_resource::<GameState>()
        .init_resource::<GameScore>()
        .add_event::<PieceDropEvent>()
        .add_event::<ChangePlayerEvent>()
        .add_event::<GameResetEvent>()
        .add_event::<PieceAnimationComplete>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (setup_camera, setup_board, setup_ui))
        .add_systems(
            Update,
            (
                handle_input,
                handle_piece_drop,
                handle_change_player.after(handle_piece_drop),
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
    pub id: Uuid,
    pub color: Option<Player>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
