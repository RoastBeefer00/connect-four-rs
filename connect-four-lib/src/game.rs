use crate::{
    board::{Board, Slot},
    errors::GameError,
    player::Player,
};

pub(crate) struct Game {
    board: Board,
    winner: Option<Player>,
    current_player: Player,
}

impl Game {
    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_none()
    }

    pub fn end_game(&mut self) {
        self.winner = Some(self.current_player());
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn make_move(&mut self, slot: Slot) -> Result<(), GameError> {
        if self.board.is_slot_full(slot) {
            return Err(GameError::ColumnIsFull);
        }
        todo!();

        Ok(())
    }
}
