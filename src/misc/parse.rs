use crate::{board::def::SQUARE_NAMES, def::Square};

pub fn algebraric_square_to_number(algebraric_square: &str) -> Option<Square> {
    let a = &algebraric_square[..];
    SQUARE_NAMES.iter().position(|&element| element == a)
}