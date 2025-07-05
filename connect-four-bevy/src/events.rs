use bevy::prelude::*;

#[derive(Event)]
pub struct PieceDropEvent {
    pub column: usize,
    pub row: usize,
}

#[derive(Event)]
pub struct GameResetEvent;

#[derive(Event)]
pub struct PieceAnimationComplete {
    pub row: usize,
    pub column: usize,
}

#[derive(Event)]
pub struct GameOverEvent {
    pub winner: Option<crate::game_logic::Player>,
}
