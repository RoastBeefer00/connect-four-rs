//! Connect Four game library
//!
//! This library provides the core game logic and components for a Connect Four game
//! built with the Bevy game engine.

pub mod board;
pub mod events;
pub mod game_logic;
pub mod ui;

// Re-export commonly used types
pub use events::{GameOverEvent, GameResetEvent, PieceAnimationComplete, PieceDropEvent};
pub use game_logic::{GameState, GameStatus, Player};
