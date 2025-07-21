use crate::{
    board::{Board, Column, Row},
    errors::GameError,
    player::Player,
};

use strum::IntoEnumIterator;

pub struct Game {
    board: Board,
    status: GameStatus,
    current_player: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::default(),
            status: GameStatus::Playing,
            current_player: Player::One,
        }
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn is_over(&self) -> bool {
        self.status == GameStatus::Won(Player::One) || self.status == GameStatus::Won(Player::Two)
    }

    pub fn end_game(&mut self) {
        self.status = GameStatus::Won(self.current_player());
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn swap_players(&mut self) {
        match self.current_player() {
            Player::One => self.current_player = Player::Two,
            Player::Two => self.current_player = Player::One,
            _ => {}
        }
    }

    pub fn make_move(&mut self, col: &Column) -> Result<(Column, Row), GameError> {
        if self.board.is_slot_full(col) {
            return Err(GameError::ColumnIsFull);
        }
        for row in Row::iter().rev() {
            if self.board.get(row, *col).is_none() {
                self.board.insert_piece(row, *col, self.current_player());
                if self.check_for_winner().is_some() {
                    self.end_game();
                }
                self.swap_players();

                return Ok((*col, row));
            }
        }

        // Should never reach this
        Ok((Column::One, Row::One))
    }

    pub fn get_winner(&self) -> Option<Player> {
        if let GameStatus::Won(winner) = self.status {
            Some(winner)
        } else {
            None
        }
    }

    pub fn check_for_winner(&self) -> Option<Player> {
        use crate::board::{Column, Row};
        use strum::IntoEnumIterator;

        let row_idx = |row: Row| usize::from(row);
        let col_idx = |col: Column| usize::from(col);

        for row in Row::iter() {
            for col in Column::iter() {
                let slot = self.board.get(row, col);
                if let Some(player) = slot {
                    // Horizontal
                    if col_idx(col) <= 3
                        && (0..4).all(|i| {
                            self.board
                                .get(row, Column::iter().nth(col_idx(col) + i).unwrap())
                                == Some(player)
                        })
                    {
                        return Some(player);
                    }
                    // Vertical
                    if row_idx(row) <= 2
                        && (0..4).all(|i| {
                            self.board
                                .get(Row::iter().nth(row_idx(row) + i).unwrap(), col)
                                == Some(player)
                        })
                    {
                        return Some(player);
                    }
                    // Diagonal down-right
                    if col_idx(col) <= 3
                        && row_idx(row) <= 2
                        && (0..4).all(|i| {
                            self.board.get(
                                Row::iter().nth(row_idx(row) + i).unwrap(),
                                Column::iter().nth(col_idx(col) + i).unwrap(),
                            ) == Some(player)
                        })
                    {
                        return Some(player);
                    }
                    // Diagonal down-left
                    if col_idx(col) >= 3
                        && row_idx(row) <= 2
                        && (0..4).all(|i| {
                            self.board.get(
                                Row::iter().nth(row_idx(row) + i).unwrap(),
                                Column::iter().nth(col_idx(col) - i).unwrap(),
                            ) == Some(player)
                        })
                    {
                        return Some(player);
                    }
                }
            }
        }
        None
    }

    pub fn surrender(&mut self, player_surrendering: Player) {
        if player_surrendering == Player::One {
            self.status = GameStatus::Won(Player::Two)
        } else if player_surrendering == Player::Two {
            self.status = GameStatus::Won(Player::One)
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    Won(Player),
    Draw,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_win() {
        let mut game = Game {
            board: Board::new(),
            status: GameStatus::Playing,
            current_player: Player::One,
        };
        let row = Row::Six;
        for col in [Column::One, Column::Two, Column::Three, Column::Four] {
            game.board.insert_piece(row, col, Player::One);
        }
        assert_eq!(game.check_for_winner(), Some(Player::One));
    }

    #[test]
    fn test_vertical_win() {
        let mut game = Game {
            board: Board::new(),
            status: GameStatus::Playing,
            current_player: Player::Two,
        };
        let col = Column::Four;
        for row in [Row::Six, Row::Five, Row::Four, Row::Three] {
            game.board.insert_piece(row, col, Player::Two);
        }
        assert_eq!(game.check_for_winner(), Some(Player::Two));
    }

    #[test]
    fn test_diagonal_down_right_win() {
        let mut game = Game {
            board: Board::new(),
            status: GameStatus::Playing,
            current_player: Player::One,
        };
        game.board.insert_piece(Row::Six, Column::One, Player::One);
        game.board.insert_piece(Row::Five, Column::Two, Player::One);
        game.board
            .insert_piece(Row::Four, Column::Three, Player::One);
        game.board
            .insert_piece(Row::Three, Column::Four, Player::One);
        assert_eq!(game.check_for_winner(), Some(Player::One));
    }

    #[test]
    fn test_diagonal_down_left_win() {
        let mut game = Game {
            board: Board::new(),
            status: GameStatus::Playing,
            current_player: Player::Two,
        };
        game.board
            .insert_piece(Row::Three, Column::Four, Player::Two);
        game.board
            .insert_piece(Row::Four, Column::Three, Player::Two);
        game.board.insert_piece(Row::Five, Column::Two, Player::Two);
        game.board.insert_piece(Row::Six, Column::One, Player::Two);
        assert_eq!(game.check_for_winner(), Some(Player::Two));
    }

    #[test]
    fn test_no_winner() {
        let mut game = Game {
            board: Board::new(),
            status: GameStatus::Playing,
            current_player: Player::One,
        };
        game.board.insert_piece(Row::Six, Column::One, Player::One);
        game.board.insert_piece(Row::Six, Column::Two, Player::Two);
        game.board
            .insert_piece(Row::Six, Column::Three, Player::One);
        game.board.insert_piece(Row::Six, Column::Four, Player::Two);
        assert_eq!(game.check_for_winner(), None);
    }

    #[test]
    fn test_draw_game() {
        use crate::board::{Column, Row};
        let mut game = Game {
            board: Board::new(),
            status: GameStatus::Playing,
            current_player: Player::One,
        };
        // Pattern that fills the board avoiding any 4-in-a-row for both players
        // Board fill absolutely guaranteed to avoid any connect four:
        let fill = [
            [
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
            ], // Row::Six
            [
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
            ], // Row::Five
            [
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
            ], // Row::Four (repeat previous for safety)
            [
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
            ], // Row::Three
            [
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
            ], // Row::Two (repeat previous for safety)
            [
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
                Player::One,
                Player::Two,
            ], // Row::One
        ];
        let rows = [
            Row::Six,
            Row::Five,
            Row::Four,
            Row::Three,
            Row::Two,
            Row::One,
        ];
        let cols = [
            Column::One,
            Column::Two,
            Column::Three,
            Column::Four,
            Column::Five,
            Column::Six,
            Column::Seven,
        ];
        for (i, row) in rows.iter().enumerate() {
            for (j, col) in cols.iter().enumerate() {
                game.board.insert_piece(*row, *col, fill[i][j]);
            }
        }
        assert_eq!(game.check_for_winner(), None);
        game.status = GameStatus::Draw;
        assert_eq!(game.status, GameStatus::Draw);
    }

    #[test]
    fn test_full_column_detection() {
        use crate::board::{Column, Row};
        let mut board = Board::new();
        let col = Column::One;
        for row in [
            Row::Six,
            Row::Five,
            Row::Four,
            Row::Three,
            Row::Two,
            Row::One,
        ] {
            board.insert_piece(row, col, Player::One);
        }
        assert!(board.is_slot_full(&col));
    }

    #[test]
    fn test_swap_player() {
        use crate::board::Column;
        let mut game = Game::new();
        assert!(game.current_player == Player::One);
        let _ = game.make_move(&Column::One);
        assert!(game.current_player == Player::Two);
        let _ = game.make_move(&Column::One);
        assert!(game.current_player == Player::One);
    }

    #[test]
    fn test_make_move() {
        use crate::board::{Column, Row};
        let mut game = Game::new();
        let (_, row) = game.make_move(&Column::One).unwrap();
        assert!(row == Row::Six);
        let (_, row) = game.make_move(&Column::One).unwrap();
        assert!(row == Row::Five);
    }
}
