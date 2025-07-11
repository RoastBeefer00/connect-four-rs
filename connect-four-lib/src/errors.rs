use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum GameError {
    #[error("column is full")]
    ColumnIsFull,
    #[error("usize {0} is out of bounds")]
    OutOfBounds(usize),
}
