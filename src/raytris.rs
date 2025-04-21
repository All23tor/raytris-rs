mod game;
mod menu;

use self::game::*;
use self::menu::*;
use raylib::prelude::*;

pub struct Raytris {
  rl: RaylibHandle,
  thread: RaylibThread,
}

impl Raytris {
  pub fn new() -> Self {
    let (mut rl, thread) = raylib::init()
      .size(Menu::INITIAL_RESOLUTION.0, Menu::INITIAL_RESOLUTION.1)
      .title("Raytris")
      .build();

    rl.set_target_fps(60);
    Raytris { rl, thread }
  }

  pub fn run(&mut self) {
    let mut rng = rand::thread_rng();
    let mut menu = Menu::new();
    while menu.run(&mut self.rl, &self.thread) == ExitCode::Game {
      let mut game = Game::new(&mut self.rl, &mut rng);
      game.run(&mut self.rl, &self.thread, &mut rng);
    }
  }
}
