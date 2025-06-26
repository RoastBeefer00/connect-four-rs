use crate::player::Player;
use strum_macros::EnumIter;
// Use array instead of vector since the board is a fixed size
// Each slot in the grid will either have nothing or the `Player` type
type BoardArray = [[Option<Player>; 7]; 6];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Board(BoardArray);

impl Board {
    pub fn new() -> Self {
        Board([[None; 7]; 6])
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Player> {
        self.0[row][col]
    }

    pub fn is_slot_full(&self, slot: Slot) -> bool {
        todo!();
    }
}

// Slots represent the columns that you drop pieces into
#[derive(Debug, EnumIter)]
pub(crate) enum Slot {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl From<Slot> for usize {
    fn from(value: Slot) -> Self {
        match value {
            Slot::One => 1,
            Slot::Two => 2,
            Slot::Three => 3,
            Slot::Four => 4,
            Slot::Five => 5,
            Slot::Six => 6,
            Slot::Seven => 7,
        }
    }
}
