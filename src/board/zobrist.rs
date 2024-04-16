use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::def::{NumberOf, Piece, Side, Sides, Square, EMPTY};

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
       let mut random = SmallRng::from_seed([125; 32]); 
       let mut zobrist_random = Self {
            ran_pieces: [[[EMPTY; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; Sides::BOTH],
            ran_castling: [EMPTY; NumberOf::CASTLING_PERMISSIONS],
            ran_side: [EMPTY; Sides::BOTH],
            ran_enpass: [EMPTY; NumberOf::SQUARES +1]
        };

        zobrist_random.ran_pieces.iter_mut().for_each(|side|{
        side.iter_mut().for_each(|piece|{
        piece.iter_mut().for_each(|square|
        *square = random.gen::<u64>())
         })
      });
      zobrist_random.ran_castling
      .iter_mut()
      .for_each(|permision| *permision = random.gen::<u64>());

      zobrist_random.ran_side
      .iter_mut()
      .for_each(|side| *side = random.gen::<u64>());

      zobrist_random.ran_enpass
      .iter_mut()
      .for_each(|ep| *ep = random.gen::<u64>());
      zobrist_random
    }
    pub fn piece(&self, piece: Piece, side: Side, square: Square) -> ZobristKey {
        self.ran_pieces[side][piece][square]
    }
    pub fn side(&self, side: Side) -> ZobristKey {
        self.ran_side[side]
    }
    pub fn castling(&self, castling_perm: u8) -> ZobristKey {
        self.ran_castling[castling_perm as usize]
    }
    pub fn en_passant(&self, enpass: Option<u8>) -> ZobristKey {
        match enpass {
            Some(ep) => self.ran_enpass[ep as usize],
            None => self.ran_enpass[NumberOf::SQUARES]
        }
    }
}