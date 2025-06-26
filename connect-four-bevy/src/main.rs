use bevy::prelude::*;

mod board;
mod events;
mod game_logic;
mod ui;

use board::*;
use events::*;
use game_logic::*;
use ui::*;

fn main() {
    App::new()
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
                animate_pieces,
                cleanup_pieces,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
