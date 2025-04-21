use std::ops::Index;

use super::falling_piece::Tetromino;
use rand::{seq::SliceRandom, Rng};

#[derive(Clone)]
pub struct NextQueue {
  queue: Vec<Tetromino>,
}

impl NextQueue {
  pub const NEXT_QUEUE_SIZE: usize = 5;
  pub const SIZE_OF_BAG: usize = 7;

  pub fn new(rng: &mut impl Rng) -> Self {
    use Tetromino::*;
    let mut queue = vec![I, O, T, S, Z, L, J];
    queue.shuffle(rng);
    Self { queue }
  }

  fn push_new_bag(&mut self, rng: &mut impl Rng) {
    use super::falling_piece::Tetromino::*;
    let mut new_bag = vec![I, O, T, S, Z, J, L];
    new_bag.shuffle(rng);
    new_bag.append(&mut self.queue);
    self.queue = new_bag;
  }

  pub fn push_new_bag_if_needed(&mut self, rng: &mut impl Rng) {
    if self.queue.len() < Self::SIZE_OF_BAG {
      self.push_new_bag(rng);
    }
  }

  pub fn get_next_tetromino(&mut self) -> Tetromino {
    self.queue.pop().unwrap()
  }
}

impl Index<usize> for NextQueue {
  type Output = Tetromino;
  fn index(&self, index: usize) -> &Self::Output {
    &self.queue[self.queue.len() - 1 - index]
  }
}
