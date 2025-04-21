#[derive(Clone, Copy, PartialEq)]
pub enum Tetromino {
  I,
  O,
  T,
  Z,
  S,
  J,
  L,
  Empty,
}

use Tetromino::*;
impl Tetromino {
  pub fn get_tetromino_color(&self) -> raylib::color::Color {
    use raylib::color::{rcolor, Color};
    match self {
      I => rcolor(49, 199, 239, 255),
      O => rcolor(247, 211, 8, 255),
      T => rcolor(173, 77, 156, 255),
      S => rcolor(66, 182, 66, 255),
      Z => rcolor(239, 32, 41, 255),
      J => rcolor(90, 101, 173, 255),
      L => rcolor(239, 121, 33, 255),
      _ => Color::BLANK,
    }
  }

  pub fn initial_tetromino_map(&self) -> TetrominoMap {
    match self {
      I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
      O => [(0, -1), (1, -1), (0, 0), (1, 0)],
      T => [(0, -1), (-1, 0), (0, 0), (1, 0)],
      S => [(0, -1), (1, -1), (-1, 0), (0, 0)],
      Z => [(-1, -1), (0, -1), (0, 0), (1, 0)],
      J => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
      L => [(1, -1), (-1, 0), (0, 0), (1, 0)],
      _ => [(0, 0), (0, 0), (0, 0), (0, 0)],
    }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
  Up,
  Left,
  Down,
  Right,
}

pub type CoordinatePair = (i8, i8);
pub type OffsetTable = [CoordinatePair; 5];
pub type TetrominoMap = [CoordinatePair; 4];

#[derive(Clone, Copy)]
pub struct FallingPiece {
  pub tetromino: Tetromino,
  pub orientation: Orientation,
  pub position: (i8, i8),
  pub tetromino_map: TetrominoMap,
}

use Orientation::*;
impl FallingPiece {
  pub fn new(tetromino: Tetromino, position: (i8, i8)) -> Self {
    let orientation = Orientation::Up;
    let tetromino_map = tetromino.initial_tetromino_map();
    FallingPiece {
      tetromino,
      orientation,
      position,
      tetromino_map,
    }
  }

  pub fn fall(&mut self) {
    self.position.1 += 1;
  }

  pub fn shift_left(&mut self) {
    self.position.0 -= 1;
  }

  pub fn shift_right(&mut self) {
    self.position.0 += 1;
  }

  pub fn turn_clockwise(&mut self) {
    for pcp in &mut self.tetromino_map {
      *pcp = (-pcp.1, pcp.0);
    }

    self.orientation = match self.orientation {
      Up => Right,
      Right => Down,
      Down => Left,
      Left => Up,
    };
  }

  pub fn turn_counter_clockwise(&mut self) {
    for pcp in &mut self.tetromino_map {
      *pcp = (pcp.1, -pcp.0);
    }

    self.orientation = match self.orientation {
      Up => Left,
      Right => Up,
      Down => Right,
      Left => Down,
    };
  }

  pub fn get_offset_table(&self) -> OffsetTable {
    match self.tetromino {
      I => match self.orientation {
        Up => [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
        Right => [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
        Down => [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
        Left => [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
      },
      O => match self.orientation {
        Up => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        Right => [(0, -1), (0, -1), (0, -1), (0, -1), (0, -1)],
        Down => [(-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1)],
        Left => [(-1, 0), (-1, 0), (-1, 0), (-1, 0), (-1, 0)],
      },
      _ => match self.orientation {
        Up => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        Right => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        Down => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        Left => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
      },
    }
  }
}

