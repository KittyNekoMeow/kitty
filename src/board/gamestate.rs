#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub active_color: u8,
    pub castling: u8,
    pub full_move_number: u8,
    pub half_move_clock: u8,
    pub en_passant: Option<u8>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            active_color: 0,
            castling: 0,
            full_move_number: 0,
            half_move_clock: 0,
            en_passant: None
        }
    }
}