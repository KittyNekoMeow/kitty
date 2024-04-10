use crate::def::{Bitboard, NumberOf, Piece, Pieces, Sides, EMPTY};

pub mod def;
pub mod fen;

pub struct Board {
    pub bb_pieces: [[Bitboard; NumberOf::PIECE_TYPES]; Sides::BOTH],
    pub bb_side: [Bitboard; Sides::BOTH],
    pub piece_list: [Piece; NumberOf::SQUARES]
}

impl Board {
    pub fn new() -> Self {
        Self { 
            bb_pieces: [[EMPTY; NumberOf::PIECE_TYPES]; Sides::BOTH], 
            bb_side: [EMPTY; Sides::BOTH], 
            piece_list: [Pieces::NONE; NumberOf::SQUARES]
         }
    }
}