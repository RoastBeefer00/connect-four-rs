use serde::{Deserialize, Serialize};

use crate::player::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsMsg {
    PlayerJoin { id: String, color: Player },
    PlayerLeave { id: String },
    PlayerMove { id: String, col: usize },
}
