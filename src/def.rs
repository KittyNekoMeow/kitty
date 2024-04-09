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

pub struct Castling;
impl Castling {
    pub const WK: usize = 1;
    pub const WQ: usize = 2;
    pub const BK: usize = 4;
    pub const BQ: usize = 8;
    pub const ALL: usize = 15;
}

pub struct NumberOf;
impl NumberOf {
    pub const PIECE_TYPES: usize = 6;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SQUARES: usize = 64;
    pub const CASTLING_PERMISSIONS: usize = 16;
}

pub const EMPTY: u64 = 0;
pub const MAX_GAME_MOVE: usize = 2048;
pub const MAX_LEGAL_MOVE: u8 = 255;
pub const MAX_DEPTH: u8 = 254;
pub const MAX_MOVE_RULE: u8 = 100;

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
