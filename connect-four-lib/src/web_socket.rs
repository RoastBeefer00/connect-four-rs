use serde::{Deserialize, Serialize};

use crate::player::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsMsg {
    PlayerJoin {
        id: String,
        color: Player,
        active_player: Player,
    },
    PlayerLeave {
        id: String,
    },
    PlayerMove {
        id: String,
        col: usize,
        row: usize,
        active_player: Player,
    },
    GameOver {
        winner: Player,
    },
}
