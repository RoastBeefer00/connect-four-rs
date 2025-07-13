use bevy::prelude::*;

use crate::game_logic::Player;

#[derive(Event)]
pub struct PieceDropEvent {
    pub column: usize,
    pub row: usize,
    pub player: Player,
}

#[derive(Event)]
pub struct ChangePlayerEvent {
    pub player: Player,
}

#[derive(Event)]
pub struct GameResetEvent;

#[derive(Event)]
pub struct PieceAnimationComplete {
    #[allow(dead_code)]
    pub row: usize,
    #[allow(dead_code)]
    pub column: usize,
}

#[derive(Event)]
pub struct GameOverEvent {
    #[allow(dead_code)]
    pub winner: Option<crate::game_logic::Player>,
}
