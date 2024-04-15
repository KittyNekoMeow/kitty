use crate::def::{NumberOf, Sides, EMPTY};

type PieceRandoms = [[[u64; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; Sides::BOTH];
type CastlingRandom = [u64; NumberOf::CASTLING_PERMISSIONS];
type SideRandom = [u64; Sides::BOTH];
type EnPassRandom = [u64; NumberOf::SQUARES +1];
pub type ZobristKey = u64;
pub struct ZobristRandom {
    ran_pieces: PieceRandoms,
    ran_castling: CastlingRandom,
    ran_side: SideRandom,
    ran_enpass: EnPassRandom
}

impl ZobristRandom {
    pub fn new() -> Self {
        Self {
            ran_pieces: [[[EMPTY; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; Sides::BOTH],
            ran_castling: [EMPTY; NumberOf::CASTLING_PERMISSIONS],
            ran_side: [EMPTY; Sides::BOTH],
            ran_enpass: [EMPTY; NumberOf::SQUARES +1]
        }
    }
}