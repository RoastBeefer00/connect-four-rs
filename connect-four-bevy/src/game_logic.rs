use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }

    pub fn color(self) -> Color {
        match self {
            Player::Red => Color::rgb(0.8, 0.2, 0.2),
            Player::Yellow => Color::rgb(0.9, 0.9, 0.2),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    Won(Player),
    Draw,
}

#[derive(Resource, Debug)]
pub struct GameState {
    pub board: [[Option<Player>; 7]; 6],
    pub current_player: Player,
    pub status: GameStatus,
    pub move_count: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [[None; 7]; 6],
            current_player: Player::Red,
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

    pub fn drop_piece(&mut self, col: usize) -> Option<usize> {
        if col >= 7 || self.is_column_full(col) || self.status != GameStatus::Playing {
            return None;
        }

        // Find the lowest empty row in this column
        for row in (0..6).rev() {
            if self.board[row][col].is_none() {
                self.board[row][col] = Some(self.current_player);
                self.move_count += 1;

                // Check for win
                if self.check_win(row, col) {
                    self.status = GameStatus::Won(self.current_player);
                } else if self.move_count >= 42 {
                    self.status = GameStatus::Draw;
                } else {
                    self.current_player = self.current_player.other();
                }

                return Some(row);
            }
        }

        None
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        let player = self.board[row][col].unwrap();

        // Check horizontal
        if self.check_direction(row, col, 0, 1, player) >= 4 {
            return true;
        }

        // Check vertical
        if self.check_direction(row, col, 1, 0, player) >= 4 {
            return true;
        }

        // Check diagonal (top-left to bottom-right)
        if self.check_direction(row, col, 1, 1, player) >= 4 {
            return true;
        }

        // Check diagonal (top-right to bottom-left)
        if self.check_direction(row, col, 1, -1, player) >= 4 {
            return true;
        }

        false
    }

    fn check_direction(
        &self,
        row: usize,
        col: usize,
        d_row: i32,
        d_col: i32,
        player: Player,
    ) -> u32 {
        let mut count = 1; // Count the piece we just placed

        // Check in positive direction
        let mut r = row as i32 + d_row;
        let mut c = col as i32 + d_col;
        while r >= 0 && r < 6 && c >= 0 && c < 7 {
            if let Some(p) = self.board[r as usize][c as usize] {
                if p == player {
                    count += 1;
                    r += d_row;
                    c += d_col;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Check in negative direction
        let mut r = row as i32 - d_row;
        let mut c = col as i32 - d_col;
        while r >= 0 && r < 6 && c >= 0 && c < 7 {
            if let Some(p) = self.board[r as usize][c as usize] {
                if p == player {
                    count += 1;
                    r -= d_row;
                    c -= d_col;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        count
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Option<Player> {
        if row < 6 && col < 7 {
            self.board[row][col]
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert_eq!(game.current_player, Player::Red);
        assert_eq!(game.status, GameStatus::Playing);
        assert_eq!(game.move_count, 0);
    }

    #[test]
    fn test_drop_piece() {
        let mut game = GameState::new();
        let row = game.drop_piece(0);
        assert_eq!(row, Some(5)); // Should drop to bottom row
        assert_eq!(game.get_piece(5, 0), Some(Player::Red));
        assert_eq!(game.current_player, Player::Yellow);
    }

    #[test]
    fn test_column_full() {
        let mut game = GameState::new();
        // Fill column 0
        for _ in 0..6 {
            game.drop_piece(0);
        }
        assert!(game.is_column_full(0));
        assert_eq!(game.drop_piece(0), None);
    }

    #[test]
    fn test_horizontal_win() {
        let mut game = GameState::new();
        // Red wins horizontally in bottom row
        game.drop_piece(0); // Red
        game.drop_piece(0); // Yellow
        game.drop_piece(1); // Red
        game.drop_piece(1); // Yellow
        game.drop_piece(2); // Red
        game.drop_piece(2); // Yellow
        game.drop_piece(3); // Red - should win

        assert_eq!(game.status, GameStatus::Won(Player::Red));
    }

    #[test]
    fn test_vertical_win() {
        let mut game = GameState::new();
        // Red wins vertically in column 0
        game.drop_piece(0); // Red
        game.drop_piece(1); // Yellow
        game.drop_piece(0); // Red
        game.drop_piece(1); // Yellow
        game.drop_piece(0); // Red
        game.drop_piece(1); // Yellow
        game.drop_piece(0); // Red - should win

        assert_eq!(game.status, GameStatus::Won(Player::Red));
    }
}
