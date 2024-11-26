use std::ops::Index;

use rand::{seq::SliceRandom, Rng};
use super::falling_piece::Tetromino;

#[derive(Clone)]
pub struct NextQueue {
  queue: Vec<Tetromino>,
}

impl NextQueue {
  pub const SIZE_OF_BAG: usize = 7;
  pub const NEXT_QUEUE_SIZE: usize = 5;

  pub fn new<R>(rng: &mut R) -> Self 
  where R: Rng + ?Sized {
    let queue = vec![];
    let mut new = Self {queue};
    new.push_new_bag(rng);
    new
  }
  
  pub fn push_new_bag_if_needed<R>(&mut self, rng: &mut R) 
  where R: Rng + ?Sized {
    if self.queue.len() < Self::SIZE_OF_BAG {
      self.push_new_bag(rng);
    }
  }

  pub fn get_next_tetromino(&mut self) -> Tetromino {
    self.queue.pop().unwrap()
  }

  fn push_new_bag<R>(&mut self, rng: &mut R)
  where R: Rng + ?Sized {
    use super::falling_piece::Tetromino::*;
    let mut bag = vec![I, O, T, S, Z, J, L];
    bag.shuffle(rng);
    bag.append(&mut self.queue);
    self.queue = bag;
  }
}

impl Index<usize> for NextQueue {
  type Output = Tetromino;
  fn index(&self, index: usize) -> &Self::Output {
    &self.queue[self.queue.len() - 1 - index]
  }
}