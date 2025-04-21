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

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
  Up,
  Left,
  Down,
  Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Shift {
  Left,
  Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RotationType {
  Clockwise,
  CounterClockwise,
  OneEighty,
}

pub type CoordinatePair = (i8, i8);
pub type OffsetTable = [CoordinatePair; 5];
pub type TetrominoMap = [CoordinatePair; 4];

impl Tetromino {
  pub fn initial_tetromino_map(&self) -> TetrominoMap {
    use Tetromino::*;
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

  pub fn get_tetromino_color(&self) -> raylib::color::Color {
    use raylib::color::{rcolor, Color};
    use Tetromino::*;
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
}

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

  pub fn shift(&mut self, shift: Shift) {
    self.position.0 -= match shift {
      Shift::Left => 1,
      Shift::Right => -1,
    };
  }

  pub fn turn(&mut self, rt: RotationType) {
    const C: RotationType = RotationType::Clockwise;
    const CC: RotationType = RotationType::CounterClockwise;
    const OE: RotationType = RotationType::OneEighty;

    for pcp in &mut self.tetromino_map {
      *pcp = match rt {
        C => (-pcp.1, pcp.0),
        CC => (pcp.1, -pcp.0),
        OE => (-pcp.0, -pcp.1),
      };
    }

    self.orientation = match (self.orientation, rt) {
      (Up, C) | (Left, OE) | (Down, CC) => Right,
      (Right, C) | (Up, OE) | (Left, CC) => Down,
      (Down, C) | (Right, OE) | (Up, CC) => Left,
      (Left, C) | (Down, OE) | (Right, CC) => Up,
    };
  }

  pub fn get_offset_table(&self) -> OffsetTable {
    match self.tetromino {
      Tetromino::I => match self.orientation {
        Up => [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
        Right => [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
        Down => [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
        Left => [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
      },
      Tetromino::O => match self.orientation {
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
