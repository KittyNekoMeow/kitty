use crate::def::{NumberOf, Piece, Pieces, Sides, EMPTY};

use self::gamestate::GameState;

pub mod def;
mod gamestate;
mod history;
#[derive(Debug, Clone)]
pub struct Board {
    pub piece_list: [Piece; NumberOf::SQUARES],
    pub game_state: GameState
}

impl Board {
    pub fn new() -> Self {
        Self { 
            piece_list: [Pieces::NONE; NumberOf::SQUARES],
            game_state: GameState::new()
         }
    }
}