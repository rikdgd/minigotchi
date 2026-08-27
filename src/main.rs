mod creature;
mod food;
mod creature_game;
mod shapes;
mod utils;
mod game_state;
mod ui;
mod save_management;
mod movements;
mod animations;
mod items;
mod game_runner;
mod creature_personality;

use macroquad::prelude::*;
use game_runner::GameRunner;
use utils::Location;


pub const SCREEN_WIDTH: i32 = 200;
pub const SCREEN_HEIGHT: i32 = 200;
pub const CREATURE_BASE_LOCATION: Location = Location { x: 100.0, y: 50.0 };
pub const BACKGROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);


#[macroquad::main(main_window_conf)]
async fn main() {
    let mut runner = GameRunner::initiate().await;
    runner.run_game().await;
}

fn main_window_conf() -> Conf {
    Conf {
        window_title: String::from("minigotchi"),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
