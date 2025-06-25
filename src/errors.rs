use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("column is full")]
    ColumnIsFull,
}
