use crate::def::MAX_GAME_MOVE;

use super::gamestate::GameState;

pub struct History {
    list: [GameState; MAX_GAME_MOVE],
    count: usize
}

impl History {
    pub fn new() -> Self {
    Self {
        list: [GameState::new(); MAX_GAME_MOVE],
        count: 0
    }        
  }
  pub fn empty(&mut self) {
      self.list = [GameState::new(); MAX_GAME_MOVE];
      self.count = 0;
  }
  pub fn push(&mut self, gamestate: GameState) {
      self.list[self.count] = gamestate;
      self.count += 1;
  }
  pub fn pop(&mut self) -> GameState {
      self.count -= 1;
      self.list[self.count]
  }
}

