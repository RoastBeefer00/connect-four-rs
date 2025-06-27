use crate::{
    board::{Board, Column, Row},
    errors::GameError,
    player::Player,
};

use strum::IntoEnumIterator;

pub(crate) struct Game {
    board: Board,
    status: GameStatus,
    current_player: Player,
}

impl Game {
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

    pub fn make_move(&mut self, col: &Column) -> Result<(), GameError> {
        if self.board.is_slot_full(&col) {
            return Err(GameError::ColumnIsFull);
        }
        for row in Row::iter().rev() {
            if self.get_board().get(row, *col).is_none() {
                self.get_board()
                    .insert_piece(row, *col, self.current_player());
                break;
            }
        }

        Ok(())
    }

    pub fn check_for_winner(&self) -> Option<Player> {
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    Won(Player),
    Draw,
}
