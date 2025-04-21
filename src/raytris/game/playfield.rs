pub mod falling_piece;
pub mod next_queue;

use rand::Rng;
use raylib::prelude::*;

use self::{
  falling_piece::{FallingPiece, RotationType, Shift, Tetromino},
  next_queue::NextQueue,
};

#[derive(Clone, Copy, Debug)]
pub enum MessageType {
  Single,
  Double,
  Triple,
  Tetris,
  AllClear,
  Empty,
}

#[derive(Clone, Copy, Debug)]
enum SpinType {
  No,
  Proper,
  Mini,
}

#[derive(Clone, Copy, Debug)]
pub struct LineClearMessage {
  pub message: MessageType,
  pub timer: u8,
  pub spin_type: SpinType,
}

impl LineClearMessage {
  pub const DURATION: u8 = 180;

  pub fn new() -> Self {
    Self {
      message: MessageType::Empty,
      timer: 0,
      spin_type: SpinType::No,
    }
  }
}

impl From<MessageType> for LineClearMessage {
  fn from(value: MessageType) -> Self {
    Self {
      message: value,
      timer: Self::DURATION,
      spin_type: SpinType::No,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Playfield {
  pub(super) grid: [[Tetromino; Self::WIDTH]; Self::HEIGHT],
  pub(super) falling_piece: FallingPiece,
  pub(super) holding_piece: Tetromino,
  pub(super) next_queue: NextQueue,
  pub(super) can_swap: bool,
  frames_since_last_fall: u8,
  lock_delay_frames: u8,
  lock_delay_moves: u8,
  signed_frames_pressed: i32,
  pub(super) combo: u8,
  pub(super) has_lost: bool,
  pub(super) score: u64,
  pub(super) b2b: u16,
  pub(super) message: LineClearMessage,
}

impl Playfield {
  pub const WIDTH: usize = 10;
  pub const HEIGHT: usize = 40;
  pub const VISIBLE_HEIGHT: usize = 20;
  const PIECE_SPAWN_POSITION: (i8, i8) =
    ((Self::WIDTH - 1) as i8 / 2, Self::VISIBLE_HEIGHT as i8 - 1);
  const DAS: u8 = 7;
  const SOFT_DROP_FRAMES: u8 = 1;
  const GRAVITY_FRAMES: u8 = 20;
  const MAX_LOCK_DELAY_FRAMES: u8 = 30;
  const MAX_LOCK_DELAY_MOVES: u8 = 15;

  pub fn new(rng: &mut impl Rng) -> Self {
    Self {
      grid: [[Tetromino::Empty; Self::WIDTH]; Self::HEIGHT],
      falling_piece: FallingPiece::new(Tetromino::Empty, Self::PIECE_SPAWN_POSITION),
      holding_piece: Tetromino::Empty,
      next_queue: NextQueue::new(rng),
      can_swap: true,
      frames_since_last_fall: 0,
      lock_delay_frames: 0,
      lock_delay_moves: 0,
      signed_frames_pressed: 0,
      combo: 0,
      has_lost: false,
      score: 0,
      b2b: 0,
      message: LineClearMessage::new(),
    }
  }

  pub fn restart(&mut self, rng: &mut impl Rng) {
    let last_score = self.score;
    *self = Self::new(rng);
    self.score = last_score;
  }

  pub fn update(&mut self, rl: &RaylibHandle, rng: &mut impl Rng) -> bool {
    if rl.is_key_pressed(KeyboardKey::KEY_R) {
      self.restart(rng);
    }
    if self.has_lost {
      return false;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_C) && self.can_swap {
      self.swap_tetromino();
    }

    self.update_timers();
    self.next_queue.push_new_bag_if_needed(rng);

    if self.falling_piece.tetromino == Tetromino::Empty {
      let new_tetromino = self.next_queue.get_next_tetromino();
      self.falling_piece = FallingPiece::new(new_tetromino, Playfield::PIECE_SPAWN_POSITION);
      self.frames_since_last_fall = 0;
      self.lock_delay_frames = 0;
      self.lock_delay_moves = 0;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
      self.shift_falling_piece(Shift::Left);
    } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
      self.shift_falling_piece(Shift::Right);
    }

    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
      if self.signed_frames_pressed < 0 {
        self.signed_frames_pressed = 0;
      }
      self.signed_frames_pressed += 1;
      while self.signed_frames_pressed > Self::DAS as i32 {
        if !self.shift_falling_piece(Shift::Left) {
          break;
        }
      }
    } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
      if self.signed_frames_pressed > 0 {
        self.signed_frames_pressed = 0;
      }
      self.signed_frames_pressed -= 1;
      while -self.signed_frames_pressed > Self::DAS as i32 {
        if !self.shift_falling_piece(Shift::Right) {
          break;
        }
      }
    } else {
      self.signed_frames_pressed = 0;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_UP) {
      self.check_rotation_collision(RotationType::Clockwise);
    } else if rl.is_key_pressed(KeyboardKey::KEY_Z) {
      self.check_rotation_collision(RotationType::CounterClockwise);
    } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
      self.check_rotation_collision(RotationType::OneEighty);
    };

    let mut old_piece = self.falling_piece;

    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
      while self.check_falling_collisions() {
        old_piece = self.falling_piece;
        self.score += 2;
        self.falling_piece.fall();
      }

      self.falling_piece = old_piece;
      self.solidify_falling_piece();
      self.lock_delay_moves = 0;
      self.lock_delay_frames = 0;
      self.clear_lines();

      return true;
    }

    let mut is_fall_step = false;

    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
      if self.frames_since_last_fall >= Self::SOFT_DROP_FRAMES {
        self.frames_since_last_fall = 0;
        is_fall_step = true;
      }
    } else if self.frames_since_last_fall >= Self::GRAVITY_FRAMES {
      self.frames_since_last_fall = 0;
      is_fall_step = true;
    }

    self.falling_piece.fall();
    if self.check_falling_collisions() {
      if is_fall_step {
        self.lock_delay_frames = 0;
        self.lock_delay_moves = 0;
      } else {
        self.falling_piece = old_piece
      }

      return false;
    }

    let mut has_piece_solidified = false;
    self.falling_piece = old_piece;

    if self.lock_delay_frames > Self::MAX_LOCK_DELAY_FRAMES
      || self.lock_delay_moves > Self::MAX_LOCK_DELAY_MOVES
    {
      self.solidify_falling_piece();
      has_piece_solidified = true;
      self.clear_lines();
    }

    has_piece_solidified
  }

  fn swap_tetromino(&mut self) {
    let current_tetromino = self.falling_piece.tetromino;
    self.falling_piece = FallingPiece::new(self.holding_piece, Self::PIECE_SPAWN_POSITION);
    self.holding_piece = current_tetromino;
    self.can_swap = false;
    self.frames_since_last_fall = 0;
    self.lock_delay_frames = 0;
    self.lock_delay_moves = 0;
  }

  fn update_timers(&mut self) {
    self.frames_since_last_fall += 1;
    self.lock_delay_frames += 1;
    if self.message.timer > 0 {
      self.message.timer -= 1;
    }
  }

  fn shift_falling_piece(&mut self, shift: Shift) -> bool {
    let old_piece = self.falling_piece;
    self.falling_piece.shift(shift);

    let mut passed_check = true;
    for pair in &self.falling_piece.tetromino_map {
      let i = pair.0 + self.falling_piece.position.0;
      let j = pair.1 + self.falling_piece.position.1;

      if i < 0
        || i >= Playfield::WIDTH as i8
        || self.grid[j as usize][i as usize] != Tetromino::Empty
      {
        passed_check = false;
        break;
      }
    }

    if !passed_check {
      self.falling_piece = old_piece;
    } else {
      self.lock_delay_frames = 0;
      self.lock_delay_moves += 1;
    }

    passed_check
  }

  fn check_rotation_collision(&mut self, rotation_type: RotationType) {
    let mut could_rotate = false;
    let old_piece = self.falling_piece.clone();
    let start_offset_values = self.falling_piece.get_offset_table();

    self.falling_piece.turn(rotation_type);
    let end_offset_values = self.falling_piece.get_offset_table();

    for offset_number in 0..start_offset_values.len() {
      let mut passed = true;

      let new_horizontal_position = self.falling_piece.position.0
        + start_offset_values[offset_number].0
        - end_offset_values[offset_number].0;
      let new_vertical_position = self.falling_piece.position.1
        - start_offset_values[offset_number].1
        + end_offset_values[offset_number].1;

      for coordinates in &self.falling_piece.tetromino_map {
        let i = coordinates.0 + new_horizontal_position;
        let j = coordinates.1 + new_vertical_position;

        if i < 0
          || i >= Self::WIDTH as i8
          || j >= Self::HEIGHT as i8
          || self.grid[j as usize][i as usize] != Tetromino::Empty
        {
          passed = false;
          break;
        }
      }

      if passed {
        self.falling_piece.position.0 = new_horizontal_position;
        self.falling_piece.position.1 = new_vertical_position;
        could_rotate = true;
        break;
      }
    }

    if !could_rotate {
      self.falling_piece = old_piece;
    } else {
      self.lock_delay_frames = 0;
      self.lock_delay_moves += 1;
    }
  }

  fn check_falling_collisions(&self) -> bool {
    for pair in &self.falling_piece.tetromino_map {
      let i = pair.0 + self.falling_piece.position.0;
      let j = pair.1 + self.falling_piece.position.1;
      if j > (Self::HEIGHT - 1) as i8 || self.grid[j as usize][i as usize] != Tetromino::Empty {
        return false;
      }
    }

    true
  }

  fn solidify_falling_piece(&mut self) {
    let mut passed = false;
    for pair in &mut self.falling_piece.tetromino_map {
      let i = pair.0 + self.falling_piece.position.0;
      let j = pair.1 + self.falling_piece.position.1;
      self.grid[j as usize][i as usize] = self.falling_piece.tetromino;

      if j as usize >= Self::VISIBLE_HEIGHT {
        passed = true;
      }
    }

    let new_tetromino = self.next_queue.get_next_tetromino();
    self.falling_piece = FallingPiece::new(new_tetromino, Self::PIECE_SPAWN_POSITION);
    self.can_swap = true;

    for coordinates in &self.falling_piece.tetromino_map {
      let i = coordinates.0 + self.falling_piece.position.0;
      let j = coordinates.1 + self.falling_piece.position.1;
      if self.grid[j as usize][i as usize] != Tetromino::Empty {
        passed = false;
      }
      break;
    }

    self.has_lost = !passed;

    self.frames_since_last_fall = 0;
    self.lock_delay_frames = 0;
    self.lock_delay_moves = 0;
  }

  fn clear_lines(&mut self) {
    let mut rows_to_clear = vec![];
    for j in 0..Self::HEIGHT {
      let mut all_true = true;
      for mino in self.grid[j] {
        if mino == Tetromino::Empty {
          all_true = false;
          break;
        }
      }

      if all_true {
        rows_to_clear.push(j);
        if rows_to_clear.len() >= 4 {
          break;
        }
      }
    }

    let size = rows_to_clear.len();

    if size == 0 {
      self.combo = 0;
      return;
    }

    if size != 4 {
      self.combo = 0;
    } else {
      self.b2b += 1;
    }

    self.combo += 1;
    self.score += self.combo as u64 * 50;

    let b2b_factor = if self.b2b >= 2 { 1.5 } else { 1.0 };

    if size == 1 {
      self.message = MessageType::Single.into();
      self.score += (100.0 * b2b_factor) as u64;
    } else if size == 2 {
      self.message = MessageType::Double.into();
      self.score += (300.0 * b2b_factor) as u64;
    } else if size == 3 {
      self.message = MessageType::Triple.into();
      self.score += (500.0 * b2b_factor) as u64;
    } else if size == 4 {
      self.message = MessageType::Tetris.into();
      self.score += (800.0 * b2b_factor) as u64;
    }

    self.clear_rows(&mut rows_to_clear, 0);

    if self.is_all_clear() {
      self.message = MessageType::AllClear.into();
      self.score += (3500.0 * b2b_factor) as u64;
    }
  }

  fn clear_rows(&mut self, row_ids: &mut Vec<usize>, count: usize) {
    if row_ids.is_empty() {
      return;
    }

    let rows_to_clear = row_ids.last().unwrap() + count;

    for row in (1..rows_to_clear + 1).rev() {
      self.grid[row] = self.grid[row - 1];
    }

    for mino in &mut self.grid[0] {
      *mino = Tetromino::Empty;
    }

    row_ids.pop();
    self.clear_rows(row_ids, count + 1);
  }

  fn is_all_clear(&self) -> bool {
    self
      .grid
      .iter()
      .all(|row| row.iter().all(|mino| *mino != Tetromino::Empty))
  }

  pub fn get_ghost_piece(&self) -> FallingPiece {
    let mut ghost_piece = self.falling_piece;
    let mut old_ghost_piece = self.falling_piece;
    loop {
      ghost_piece.fall();
      let mut passed = true;
      for pair in &ghost_piece.tetromino_map {
        let i = pair.0 + ghost_piece.position.0;
        let j = pair.1 + ghost_piece.position.1;
        if j > Playfield::HEIGHT as i8 - 1 || self.grid[j as usize][i as usize] != Tetromino::Empty
        {
          passed = false;
          break;
        }
      }
      if !passed {
        ghost_piece = old_ghost_piece;
        break;
      }
      old_ghost_piece = ghost_piece;
    }

    ghost_piece
  }
}
