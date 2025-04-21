mod playfield;

use self::playfield::{falling_piece::*, next_queue::*, *};
use raylib::prelude::*;

pub struct Game {
  block_length: f32,
  position: Vector2,
  playfield: Playfield,
  paused: bool,
  undo_move_stack: Vec<Playfield>,
}

impl Game {
  const HEIGHT_SCALE_FACTOR: f32 = 0.8;
  pub fn new(rl: &RaylibHandle) -> Self {
    let block_length =
      rl.get_screen_height() as f32 * Self::HEIGHT_SCALE_FACTOR / Playfield::VISIBLE_HEIGHT as f32;
    let position = Vector2 {
      x: (rl.get_screen_width() as f32 - block_length * Playfield::WIDTH as f32) / 2.0,
      y: (rl.get_screen_height() as f32 - block_length * Playfield::VISIBLE_HEIGHT as f32) / 2.0,
    };
    let playfield = Playfield::new();
    let undo_move_stack = vec![playfield.clone()];
    Game {
      block_length,
      position,
      playfield,
      paused: false,
      undo_move_stack,
    }
  }

  pub fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
    while !rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) || !self.paused {
      self.update(rl);
      self.draw(rl, thread);
    }

    let _d = rl.begin_drawing(thread);
  }

  fn update(&mut self, rl: &RaylibHandle) {
    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
      && rl.is_key_pressed(KeyboardKey::KEY_Z)
      && !self.undo_move_stack.is_empty()
    {
      self.playfield = self.undo_move_stack.pop().unwrap();
      return;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
      self.paused = !self.paused;
    }

    if self.paused {
      return;
    }

    if self.playfield.update(rl) {
      self.undo_move_stack.push(self.playfield.clone());
    }
  }

  fn get_block_rectangle(&self, i: i8, j: i8) -> Rectangle {
    Rectangle::new(
      self.position.x + i as f32 * self.block_length,
      self.position.y + (j - Playfield::VISIBLE_HEIGHT as i8) as f32 * self.block_length,
      self.block_length,
      self.block_length,
    )
  }

  fn draw_rectangle_pretty(
    &self,
    d: &mut RaylibDrawHandle,
    rec: Rectangle,
    fill: Color,
    mut outline: Color,
  ) {
    if fill.a == 0 {
      return;
    }

    outline.a /= 8;
    d.draw_rectangle_rec(rec, fill);
    d.draw_rectangle(
      (rec.x + self.block_length / 3.0) as i32,
      (rec.y + self.block_length / 3.0) as i32,
      (rec.width / 3.0) as i32,
      (rec.height / 3.0) as i32,
      outline,
    );
    d.draw_rectangle_lines_ex(rec, self.block_length / 8.0, outline);
  }

  pub fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::LIGHTGRAY);

    self.draw_tetrion(&mut d);
    let ghost_piece = self.playfield.get_ghost_piece();
    self.draw_piece(
      &mut d,
      &ghost_piece.tetromino_map,
      Color::GRAY,
      ghost_piece.position,
    );
    let falling_piece = &self.playfield.falling_piece;
    self.draw_piece(
      &mut d,
      &falling_piece.tetromino_map,
      falling_piece.tetromino.get_tetromino_color(),
      falling_piece.position,
    );

    let font_size = self.block_length as i32 * 2;

    // Next coming pieces
    let next_text_block =
      self.get_block_rectangle(Playfield::WIDTH as i8 + 1, Playfield::VISIBLE_HEIGHT as i8);
    let mut next_queue_background = self.get_block_rectangle(
      Playfield::WIDTH as i8 + 1,
      Playfield::VISIBLE_HEIGHT as i8 + 2,
    );
    next_queue_background.width = self.block_length * 6.0;
    next_queue_background.height = self.block_length * (3 * NextQueue::NEXT_QUEUE_SIZE + 1) as f32;
    d.draw_rectangle_rec(next_queue_background, Color::GRAY);
    d.draw_rectangle_lines_ex(next_queue_background, self.block_length / 4.0, Color::BLACK);
    d.draw_text(
      "NEXT",
      next_text_block.x as i32,
      next_text_block.y as i32,
      font_size,
      Color::BLACK,
    );

    for id in 0..NextQueue::NEXT_QUEUE_SIZE {
      let current_tetromino = self.playfield.next_queue[id];
      self.draw_piece(
        &mut d,
        &current_tetromino.initial_tetromino_map(),
        current_tetromino.get_tetromino_color(),
        (
          Playfield::WIDTH as i8 + 3,
          3 * (id as i8 + 1) + Playfield::VISIBLE_HEIGHT as i8 + 1,
        ),
      );
    }

    // Draw hold piece
    let hold_text_block = self.get_block_rectangle(-7, Playfield::VISIBLE_HEIGHT as i8);
    d.draw_text(
      "HOLD",
      hold_text_block.x as i32,
      hold_text_block.y as i32,
      font_size,
      Color::BLACK,
    );
    let mut hold_piece_background =
      self.get_block_rectangle(-7, Playfield::VISIBLE_HEIGHT as i8 + 2);
    hold_piece_background.width = self.block_length * 6.0;
    hold_piece_background.height = self.block_length * 4.0;
    d.draw_rectangle_rec(hold_piece_background, Color::GRAY);
    d.draw_rectangle_lines_ex(hold_piece_background, self.block_length / 4.0, Color::BLACK);

    let hold_color = if self.playfield.can_swap {
      self.playfield.holding_piece.get_tetromino_color()
    } else {
      Color::DARKGRAY
    };
    self.draw_piece(
      &mut d,
      &self.playfield.holding_piece.initial_tetromino_map(),
      hold_color,
      (-5, 4 + Playfield::VISIBLE_HEIGHT as i8),
    );

    // Line Clear message
    if self.playfield.message.timer > 0 {
      let clear_text_block = self.get_block_rectangle(-10, Playfield::HEIGHT as i8);
      let color_scale_factor =
        self.playfield.message.timer as f32 / LineClearMessage::DURATION as f32;
      let mut text_color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: (255.0 * color_scale_factor) as u8,
      };

      let text_message = match self.playfield.message.message {
        MessageType::Single => "SINGLE",
        MessageType::Double => {
          text_color = Color {
            r: 235,
            g: 149,
            b: 52,
            a: text_color.a,
          };
          "DOUBLE"
        }
        MessageType::Triple => {
          text_color = Color {
            r: 88,
            g: 235,
            b: 52,
            a: text_color.a,
          };
          "TRIPLE"
        }
        MessageType::Tetris => {
          text_color = Color {
            r: 52,
            g: 164,
            b: 236,
            a: text_color.a,
          };
          "TETRIS"
        }
        MessageType::AllClear => {
          text_color = Color {
            r: 235,
            g: 52,
            b: 213,
            a: text_color.a,
          };
          "ALL CLEAR"
        }
        MessageType::Empty => "",
      };
      d.draw_text(
        text_message,
        clear_text_block.x as i32,
        clear_text_block.y as i32,
        font_size,
        text_color,
      );
    }

    // Combo
    if self.playfield.combo >= 2 {
      let combo_text_block = self.get_block_rectangle(-10, Playfield::HEIGHT as i8 - 2);
      let combo = format!("{}", self.playfield.combo);
      d.draw_text(
        "COMBO ",
        combo_text_block.x as i32,
        combo_text_block.y as i32,
        font_size,
        Color::BLUE,
      );
      d.draw_text(
        &combo,
        combo_text_block.x as i32 + d.measure_text("COMBO ", font_size as i32),
        combo_text_block.y as i32,
        font_size,
        Color::BLUE,
      );
    }

    // Back to Back (B2B)
    if self.playfield.b2b >= 2 {
      let b2b_text_block = self.get_block_rectangle(-10, Playfield::HEIGHT as i8 - 4);
      let b2b = format!("{}", self.playfield.b2b - 1);
      d.draw_text(
        "B2B ",
        b2b_text_block.x as i32,
        b2b_text_block.y as i32,
        font_size,
        Color::BLUE,
      );
      d.draw_text(
        &b2b,
        b2b_text_block.x as i32 + d.measure_text("B2B ", font_size),
        b2b_text_block.y as i32,
        font_size,
        Color::BLUE,
      );
    }

    // Score
    let score_text_block = self.get_block_rectangle(11, Playfield::HEIGHT as i8 - 2);
    d.draw_text(
      "SCORE: ",
      score_text_block.x as i32,
      score_text_block.y as i32,
      font_size,
      Color::BLACK,
    );
    let score_number_block = self.get_block_rectangle(11, Playfield::HEIGHT as i8);
    let score = format!("{:09}", self.playfield.score);
    d.draw_text(
      &score,
      score_number_block.x as i32,
      score_number_block.y as i32,
      font_size,
      Color::BLACK,
    );

    // Game over or paused
    if self.playfield.has_lost || self.paused {
      let screen_width = d.get_screen_width();
      let screen_height = d.get_screen_height();
      let font_size_big = self.block_length as i32 * 5;

      d.draw_rectangle(
        0,
        0,
        screen_width,
        screen_height,
        Color {
          r: 0,
          g: 0,
          b: 0,
          a: 100,
        },
      );

      if self.playfield.has_lost {
        d.draw_text(
          "YOU LOST",
          (screen_width - d.measure_text("YOU LOST", font_size_big)) / 2,
          screen_height / 2,
          font_size_big,
          Color::RED,
        );
      } else if self.paused {
        d.draw_text(
          "GAME PAUSED",
          (screen_width - d.measure_text("GAME PAUSED", font_size_big)) / 2,
          screen_height / 2,
          font_size_big,
          Color::BLUE,
        );
      }
      d.draw_text(
        "Press Esc to quit",
        (screen_width - d.measure_text("Press Esc to quit", font_size)) / 2,
        screen_height / 2 + font_size_big,
        font_size,
        Color::WHITE,
      );
    }
  }

  fn draw_tetrion(&self, d: &mut RaylibDrawHandle) {
    let tetrion = Rectangle::new(
      self.position.x,
      self.position.y,
      self.block_length * Playfield::WIDTH as f32,
      self.block_length * Playfield::VISIBLE_HEIGHT as f32,
    );
    d.draw_rectangle_rec(tetrion, Color::BLACK);

    for i in 1..Playfield::WIDTH as i8 {
      let rec = self.get_block_rectangle(i, Playfield::VISIBLE_HEIGHT as i8);
      d.draw_line_ex(
        Vector2 {
          x: rec.x.floor(),
          y: rec.y.floor(),
        },
        Vector2 {
          x: rec.x.floor(),
          y: (rec.y + Playfield::VISIBLE_HEIGHT as f32 * self.block_length).floor(),
        },
        self.block_length / 10.0,
        Color::DARKGRAY,
      );
    }

    for j in 1..Playfield::VISIBLE_HEIGHT as i8 {
      let rec = self.get_block_rectangle(0, j + Playfield::VISIBLE_HEIGHT as i8);
      d.draw_line_ex(
        Vector2 {
          x: rec.x.floor(),
          y: rec.y.floor(),
        },
        Vector2 {
          x: (rec.x + Playfield::WIDTH as f32 * self.block_length).floor(),
          y: rec.y.floor(),
        },
        self.block_length / 10.0,
        Color::DARKGRAY,
      );
    }

    for j in 0..Playfield::HEIGHT {
      for i in 0..Playfield::WIDTH {
        self.draw_rectangle_pretty(
          d,
          self.get_block_rectangle(i as i8, j as i8),
          self.playfield.grid[j][i].get_tetromino_color(),
          Color::BLACK,
        );
      }
    }
  }

  fn draw_piece(
    &self,
    d: &mut RaylibDrawHandle,
    map: &TetrominoMap,
    color: Color,
    offset: (i8, i8),
  ) {
    for coordinates in map {
      let i = coordinates.0 as i8 + offset.0;
      let j = coordinates.1 as i8 + offset.1;
      self.draw_rectangle_pretty(d, self.get_block_rectangle(i, j), color, Color::BLACK);
    }
  }
}
