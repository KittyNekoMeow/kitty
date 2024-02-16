pub type Bitboard = u64;
pub type Piece = usize;
pub type Square = usize;
pub type Side = usize;

pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

pub struct File;
impl File {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const C: usize = 2;
    pub const D: usize = 3;
    pub const E: usize = 4;
    pub const F: usize = 5;
    pub const G: usize = 6;
    pub const H: usize = 7;

}

pub struct Rank;
impl Rank {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R3: usize = 2;
    pub const R4: usize = 3;
    pub const R5: usize = 4;
    pub const R6: usize = 5;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}

pub struct Squares;
impl Squares {
    pub const A1: Square = 0;
    pub const A2: Square = 1;
    pub const A3: Square = 2;
    pub const A4: Square = 3;
    pub const A5: Square = 4;
    pub const A6: Square = 5;
    pub const A7: Square = 6;
    pub const A8: Square = 7;
    pub const B1: Square = 8;
    pub const B2: Square = 9;
    pub const B3: Square = 10;
    pub const B4: Square = 11;
    pub const B5: Square = 12;
    pub const B6: Square = 13;
    pub const B7: Square = 14;
    pub const B8: Square = 15;
    pub const C1: Square = 16;
    pub const C2: Square = 17;
    pub const C3: Square = 18;
    pub const C4: Square = 19;
    pub const C5: Square = 20;
    pub const C6: Square = 21;
    pub const C7: Square = 22;
    pub const C8: Square = 23;
    pub const D1: Square = 24;
    pub const D2: Square = 25;
    pub const D3: Square = 26;
    pub const D4: Square = 27;
    pub const D5: Square = 28;
    pub const D6: Square = 29;
    pub const D7: Square = 30;
    pub const D8: Square = 31;
    pub const E1: Square = 32;
    pub const E2: Square = 33;
    pub const E3: Square = 34;
    pub const E4: Square = 35;
    pub const E5: Square = 36;
    pub const E6: Square = 37;
    pub const E7: Square = 38;
    pub const E8: Square = 39;
    pub const F1: Square = 40;
    pub const F2: Square = 41;
    pub const F3: Square = 42;
    pub const F4: Square = 43;
    pub const F5: Square = 44;
    pub const F6: Square = 45;
    pub const F7: Square = 46;
    pub const F8: Square = 47;
    pub const G1: Square = 48;
    pub const G2: Square = 49;
    pub const G3: Square = 50;
    pub const G4: Square = 51;
    pub const G5: Square = 52;
    pub const G6: Square = 53;
    pub const G7: Square = 54;
    pub const G8: Square = 55;
    pub const H1: Square = 56;
    pub const H2: Square = 57;
    pub const H3: Square = 58;
    pub const H4: Square = 59;
    pub const H5: Square = 60;
    pub const H6: Square = 61;
    pub const H7: Square = 62;
    pub const H8: Square = 63;
}


pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
