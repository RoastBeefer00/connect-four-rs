use crate::player::Player;
use strum::IntoEnumIterator;
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

    pub fn get(&self, row: Row, col: Column) -> Option<Player> {
        self.0[usize::from(row)][usize::from(col)]
    }

    pub fn insert_piece(&mut self, row: Row, col: Column, piece: Player) {
        self.0[usize::from(row)][usize::from(col)] = Some(piece);
    }

    pub fn is_slot_full(&self, col: &Column) -> bool {
        let pieces = Row::iter()
            .filter(|row| self.get(*row, *col).is_some())
            .count();

        pieces == 6
    }
}

// Slots represent the columns that you drop pieces into
#[derive(Debug, EnumIter, Clone, Copy)]
pub(crate) enum Column {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl From<Column> for usize {
    fn from(value: Column) -> Self {
        match value {
            Column::One => 1,
            Column::Two => 2,
            Column::Three => 3,
            Column::Four => 4,
            Column::Five => 5,
            Column::Six => 6,
            Column::Seven => 7,
        }
    }
}

impl From<&Column> for usize {
    fn from(value: &Column) -> Self {
        match value {
            Column::One => 1,
            Column::Two => 2,
            Column::Three => 3,
            Column::Four => 4,
            Column::Five => 5,
            Column::Six => 6,
            Column::Seven => 7,
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub(crate) enum Row {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl From<Row> for usize {
    fn from(value: Row) -> Self {
        match value {
            Row::One => 1,
            Row::Two => 2,
            Row::Three => 3,
            Row::Four => 4,
            Row::Five => 5,
            Row::Six => 6,
        }
    }
}

impl From<&Row> for usize {
    fn from(value: &Row) -> Self {
        match value {
            Row::One => 1,
            Row::Two => 2,
            Row::Three => 3,
            Row::Four => 4,
            Row::Five => 5,
            Row::Six => 6,
        }
    }
}
