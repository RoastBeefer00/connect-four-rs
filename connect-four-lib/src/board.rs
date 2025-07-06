use crate::player::Player;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
// Use array instead of vector since the board is a fixed size
// Each slot in the grid will either have nothing or the `Player` type
pub type BoardArray = [[Option<Player>; 7]; 6];

#[derive(Clone, Copy, Debug)]
pub struct Board(BoardArray);

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub fn get_board_array(&self) -> BoardArray {
        self.0
    }
}

// Slots represent the columns that you drop pieces into
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum Column {
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
            Column::One => 0,
            Column::Two => 1,
            Column::Three => 2,
            Column::Four => 3,
            Column::Five => 4,
            Column::Six => 5,
            Column::Seven => 6,
        }
    }
}

impl From<&Column> for usize {
    fn from(value: &Column) -> Self {
        match value {
            Column::One => 0,
            Column::Two => 1,
            Column::Three => 2,
            Column::Four => 3,
            Column::Five => 4,
            Column::Six => 5,
            Column::Seven => 6,
        }
    }
}

impl From<usize> for Column {
    fn from(value: usize) -> Self {
        match value {
            0 => Column::One,
            1 => Column::Two,
            2 => Column::Three,
            3 => Column::Four,
            4 => Column::Five,
            5 => Column::Six,
            6 => Column::Seven,
            _ => panic!("Column index out of bounds"),
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum Row {
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
            Row::One => 0,
            Row::Two => 1,
            Row::Three => 2,
            Row::Four => 3,
            Row::Five => 4,
            Row::Six => 5,
        }
    }
}

impl From<&Row> for usize {
    fn from(value: &Row) -> Self {
        match value {
            Row::One => 0,
            Row::Two => 1,
            Row::Three => 2,
            Row::Four => 3,
            Row::Five => 4,
            Row::Six => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board_new_empty() {
        let board = Board::new();
        for row in Row::iter() {
            for col in Column::iter() {
                assert_eq!(board.get(row, col), None);
            }
        }
    }

    #[test]
    fn test_insert_and_get_piece() {
        let mut board = Board::new();
        board.insert_piece(Row::Three, Column::Four, Player::One);
        assert_eq!(board.get(Row::Three, Column::Four), Some(Player::One));
        assert_eq!(board.get(Row::Two, Column::Four), None);
    }

    #[test]
    fn test_is_slot_full_false_and_true() {
        let mut board = Board::new();
        let col = Column::Three;
        // Initially false
        assert!(!board.is_slot_full(&col));
        // Fill entire column
        for row in Row::iter() {
            board.insert_piece(row, col, Player::Two);
        }
        assert!(board.is_slot_full(&col));
    }

    #[test]
    fn test_column_from_usize() {
        assert_eq!(usize::from(Column::One), 0);
        assert_eq!(usize::from(Column::Four), 3);
        assert_eq!(usize::from(Column::Seven), 6);
    }

    #[test]
    fn test_column_ref_from_usize() {
        let col = Column::Three;
        assert_eq!(usize::from(&col), 2);
        let col = Column::Five;
        assert_eq!(usize::from(&col), 4);
    }

    #[test]
    fn test_row_from_usize() {
        assert_eq!(usize::from(Row::One), 0);
        assert_eq!(usize::from(Row::Six), 5);
        assert_eq!(usize::from(Row::Four), 3);
    }

    #[test]
    fn test_row_ref_from_usize() {
        let row = Row::Two;
        assert_eq!(usize::from(&row), 1);
        let row = Row::Five;
        assert_eq!(usize::from(&row), 4);
    }
}
