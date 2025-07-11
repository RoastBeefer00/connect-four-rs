use serde::{Deserialize, Serialize};
//
// This type can represent each player as well as the piece for each player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Player {
    One,
    Two,
    Spectator,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::One => write!(f, "Player one"),
            Player::Two => write!(f, "Player two"),
            Player::Spectator => write!(f, "Spectator"),
        }
    }
}
