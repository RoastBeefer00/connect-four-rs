use serde::{Deserialize, Serialize};
//
// This type can represent each player as well as the piece for each player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Player {
    One,
    Two,
}
