mod menu;
mod game;

use raylib::prelude::*;
use self::menu::*;
use self::game::*;

pub struct Raytris {
  rl: RaylibHandle,
  thread: RaylibThread
}

impl Raytris {
  pub fn new() -> Self {
    let (mut rl, thread) = raylib::init()
    .size(Menu::INITIAL_RESOLUTION.0, Menu::INITIAL_RESOLUTION.1)
    .title("Raytris")
    .build();

    rl.set_target_fps(60);
    Raytris {rl, thread}
  }

  pub fn run(&mut self) {
    let mut menu = Menu::new();
    while menu.run(&mut self.rl, &self.thread) == ExitCode::Game {
      Game::new(&mut self.rl).run(&mut self.rl, &self.thread);
    }
  }
}