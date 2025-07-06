use serde::{Deserialize, Serialize};

use crate::{board::BoardArray, player::Player};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsMsg {
    // Join message from server to client
    ServerJoin {
        // The ID generated on the client
        id: String,
        // The Player type assigned to the client
        client_player: Player,
        // The Player whose turn it is
        active_player: Player,
        // The current state of the board
        game_board: BoardArray,
    },
    // Join message from client to server
    ClientJoin {
        // The ID generated on the client
        id: String,
    },
    // Leave message
    PlayerLeave {
        // Client id on the player that left
        id: String,
    },
    // Move message from server to client
    ServerMove {
        // Client id of who made the move
        id: String,
        // Column move was made on
        col: usize,
        // Row piece should fall to
        row: usize,
        // Who becomes the active player after the move
        active_player: Player,
    },
    // Move message from client to server
    ClientMove {
        // Client id making the move
        id: String,
        // Column to make the move on (server handles game logic determining which row it will
        // reach)
        col: usize,
    },
    GameOver {
        // Player that won the game
        winner: Player,
    },
}
