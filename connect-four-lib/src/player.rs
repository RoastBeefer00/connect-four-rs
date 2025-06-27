// This type can represent each player as well as the piece for each player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Player {
    One,
    Two,
}
