use bevy::prelude::*;
use connect_four_lib::board::BoardArray;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Component)]
pub enum Player {
    One,
    Two,
    Spectator,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::One => write!(f, "Red"),
            Player::Two => write!(f, "Yellow"),
            Player::Spectator => write!(f, "Spectator"),
        }
    }
}

impl Player {
    pub fn other(self) -> Option<Self> {
        match self {
            Player::One => Some(Player::Two),
            Player::Two => Some(Player::One),
            Player::Spectator => None,
        }
    }

    pub fn color(self) -> Option<Color> {
        match self {
            Player::One => Some(Color::srgb(0.8, 0.2, 0.2)),
            Player::Two => Some(Color::srgb(0.9, 0.9, 0.2)),
            Player::Spectator => None,
        }
    }
}

impl From<&connect_four_lib::player::Player> for Player {
    fn from(value: &connect_four_lib::player::Player) -> Self {
        match value {
            connect_four_lib::player::Player::One => Player::One,
            connect_four_lib::player::Player::Two => Player::Two,
            connect_four_lib::player::Player::Spectator => Player::Spectator,
        }
    }
}

impl From<Player> for connect_four_lib::player::Player {
    fn from(value: Player) -> Self {
        match value {
            Player::One => connect_four_lib::player::Player::One,
            Player::Two => connect_four_lib::player::Player::Two,
            Player::Spectator => connect_four_lib::player::Player::Spectator,
        }
    }
}

impl From<&Player> for connect_four_lib::player::Player {
    fn from(value: &Player) -> Self {
        match value {
            Player::One => connect_four_lib::player::Player::One,
            Player::Two => connect_four_lib::player::Player::Two,
            Player::Spectator => connect_four_lib::player::Player::Spectator,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    Won(Player),
    Draw,
}

type Board = [[Option<Player>; 7]; 6];

#[derive(Resource, Debug)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub status: GameStatus,
    pub move_count: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [[None; 7]; 6],
            current_player: Player::One,
            status: GameStatus::Playing,
            move_count: 0,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn is_column_full(&self, col: usize) -> bool {
        if col >= 7 {
            return true;
        }
        self.board[0][col].is_some()
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Option<Player> {
        if row < 6 && col < 7 {
            self.board[row][col]
        } else {
            None
        }
    }

    pub fn get_state_from_lib(
        &mut self,
        board: &[[std::option::Option<connect_four_lib::player::Player>; 7]; 6],
    ) {
        let mut new_board: [[Option<Player>; 7]; 6] = [[None; 7]; 6];
        for (i, row) in board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if let Some(player) = col {
                    new_board[i][j] = Some(player.into());
                }
            }
        }

        self.board = new_board;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert_eq!(game.current_player, Player::One);
        assert_eq!(game.status, GameStatus::Playing);
        assert_eq!(game.move_count, 0);
    }
}
