use crate::{board::fen, def::{Pieces, Sides, Square, FEN_START_POSITION}};
use core::fmt;
use std::{fmt::Display, ops::RangeInclusive};

use super::{def::{File, Rank, Squares, BB_SQUARES}, Board};

const FEN_PART_NM: usize = 6;
const LIST_OF_PIECES: &str = "kqrbnpKQRBKP";
const ENP_SQUARE_WHITE: RangeInclusive<Square> = Squares::A3..=Squares::H3;
const ENP_SQUARE_BLACK: RangeInclusive<Square> = Squares::A6..=Squares::H6;
const WHITE_OR_BLACK: &str ="wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const EM_DASH: char = '–';
const SPACE: char = ' ';

#[derive(Debug)]
pub enum FenError {
    IncorrectLength,
    Part1,
    Part2,
    Part3,
    Part4,
    Part5,
    Part6,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLength => "Error in FEN string: Must be 6 parts",
            Self::Part1 => "Error in FEN Part 1: Pieces or squares",
            Self::Part2 => "Error in FEN Part 2: Colors",
            Self::Part3 => "Error in FEN Part 3: Castling rights",
            Self::Part4 => "Error in FEN Part 4: En passant field",
            Self::Part5 => "Error in FEN Part 5: Half-move clock",
            Self::Part6 => "Error in FEN Part 6: Full-move number",
        };
        write!(f, "{error}")
    }
}

pub type FenResult = Result<(), FenError>;
pub type SplitResult = Result<Vec<String>, FenError>;
type FenPartParser = fn(board: &mut Board, part: &str);

impl Board {
    pub fn fen_setup(&mut self, fen_string: Option<&str>) -> FenResult {
        let i: FenResult = Ok(());
        return i;
    }
}

fn split_fen_string(fen_string: Option<&str>) -> SplitResult {
    const SHORT_FEN_LENGTH: usize = 4;

    let mut fen_string: Vec<String> = match fen_string {
        Some(fen) => fen,
        None => FEN_START_POSITION
    }
    .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
    .split(SPACE)
    .map(String::from)
    .collect();
    if fen_string.len() == SHORT_FEN_LENGTH {
        fen_string.append(&mut vec![String::from("0"), String::from("1")]);
    }
    if fen_string.len() != FEN_PART_NM {
        return Err(FenError::IncorrectLength);
    }
    Ok(fen_string)
}

/* fn create_fen_part_parsers() -> [FenPartParser; FEN_PART_NM] {
    [
        pieces,
        color,
        castling,
        en_passant,
        half_move_clock,
        full_move_number,
    ]
} */

fn pieces(board: &mut Board, part: &str) -> FenResult {
    let mut rank = Rank::R8 as u8;
    let mut file = File::A as u8;

    for i in part.chars() {
        let square = ((rank * 8) + file) as usize;
        match i {
            'K' => board.bb_pieces[Sides::WHITE][Pieces::KING] |= BB_SQUARES[square],
            'Q' => board.bb_pieces[Sides::WHITE][Pieces::QUEEN] |= BB_SQUARES[square],
            'R' => board.bb_pieces[Sides::WHITE][Pieces::ROOK] |= BB_SQUARES[square],
            'B' => board.bb_pieces[Sides::WHITE][Pieces::BISHOP] |= BB_SQUARES[square],
            'N' => board.bb_pieces[Sides::WHITE][Pieces::KNIGHT] |= BB_SQUARES[square],
            'P' => board.bb_pieces[Sides::WHITE][Pieces::PAWN] |= BB_SQUARES[square],
            'k' => board.bb_pieces[Sides::BLACK][Pieces::KING] |= BB_SQUARES[square],
            'q' => board.bb_pieces[Sides::BLACK][Pieces::QUEEN] |= BB_SQUARES[square],
            'r' => board.bb_pieces[Sides::BLACK][Pieces::ROOK] |= BB_SQUARES[square],
            'b' => board.bb_pieces[Sides::BLACK][Pieces::BISHOP] |= BB_SQUARES[square],
            'n' => board.bb_pieces[Sides::BLACK][Pieces::KNIGHT] |= BB_SQUARES[square],
            'p' => board.bb_pieces[Sides::BLACK][Pieces::PAWN] |= BB_SQUARES[square],
            '1'..='8' => {
                if let Some(x) = i.to_digit(10) {
                    file += x as u8;
                }
            }
            SPLITTER => {
                if file != 8 {
                    return Err(FenError::Part1);
                }
                rank -= 1;
                file = 0;
            }
            _ => return Err(FenError::Part1)
        }
        if LIST_OF_PIECES.contains(i) {
            file += 1;
        }
    }
    Ok(())
}
