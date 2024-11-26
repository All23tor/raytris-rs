use raylib::prelude::*;

pub struct Menu {
  resolution: Resolution,
  window_resolution: (i32, i32)
}

#[derive(PartialEq)]
enum Resolution {
  Small,
  Medium,
  Big,
  Fullscreen
}

impl Resolution {
  pub fn next(&self) -> Self {
    match self {
      Self::Small => Self::Medium,
      Self::Medium => Self::Big,
      Self::Big => Self::Fullscreen,
      Self::Fullscreen => Self::Small
    }
  }
}

#[derive(PartialEq)]
pub enum ExitCode {
  Game,
  Exit
}

impl Menu {
  pub const INITIAL_RESOLUTION: (i32, i32) = (640, 360);

  pub fn new() -> Self {
    Menu {resolution: Resolution::Small, window_resolution: Self::INITIAL_RESOLUTION}
  }

  pub fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> ExitCode {
    while !rl.is_key_pressed(KeyboardKey::KEY_ENTER) && !rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
      self.update(rl);
      self.draw(rl, thread);
    }

    let exit_code = if rl.is_key_down(KeyboardKey::KEY_ENTER) {ExitCode::Game} else {ExitCode::Exit};
    
    let d = rl.begin_drawing(thread);
    drop(d);

    exit_code
  }

  fn update(&mut self, rl: &mut RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_F) {
      self.resize_screen(rl);
    }
  }

  fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let font_size = self.window_resolution.1 / 10;
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::LIGHTGRAY);
    d.draw_text("RAYTRIS",
      (self.window_resolution.0 - d.measure_text("RAYTRIS", font_size * 2)) / 2 ,
      self.window_resolution.1 / 2 - 3 * font_size,
      font_size as i32 * 2, Color::RED);
    let resolution = format!("{} x {}", self.window_resolution.0, self.window_resolution.1);
    d.draw_text(&resolution,
      (self.window_resolution.0 - d.measure_text(&resolution, font_size)) / 2,
      self.window_resolution.1 / 2,
      font_size, Color::BLUE); 
    d.draw_text("Press F to resize",
      (self.window_resolution.0 - d.measure_text("Press F to resize", font_size)) / 2,
      self.window_resolution.1 / 2 + font_size,
      font_size, Color::BLACK); 
    d.draw_text("Press Enter to Play",
      (self.window_resolution.0 - d.measure_text("Press Enter to Play", font_size)) / 2,
      self.window_resolution.1 / 2 + 2 * font_size,
      font_size, Color::BLACK); 
  }

  fn resize_screen(&mut self, rl: &mut RaylibHandle) {
    self.resolution = self.resolution.next();
    self.window_resolution = self.get_window_resolution(&rl);

    if rl.is_window_fullscreen() {
      rl.toggle_fullscreen();
    }

    rl.set_window_size(self.window_resolution.0, self.window_resolution.1);

    if self.resolution == Resolution::Fullscreen {
      rl.toggle_fullscreen();
    }
  }

  fn get_window_resolution(&self, rl: &RaylibHandle) -> (i32, i32) {
    match self.resolution {
        Resolution::Small => Menu::INITIAL_RESOLUTION,
        Resolution::Medium => (960, 540),
        Resolution::Big => (1280, 720),
        Resolution::Fullscreen => (rl.get_screen_width(), rl.get_screen_height())
    }
  }
}