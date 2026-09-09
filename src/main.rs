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
mod coin_drops;

use macroquad::miniquad::conf::Icon;
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
        icon: Some(get_app_icon()),
        ..Default::default()
    }
}

fn get_app_icon() -> Icon {
    let small_bytes = include_bytes!("../resources/icon/icon_16x16.png");
    let mut small_buffer = [0u8; 1024];
    for (i, byte) in small_bytes.iter().enumerate() {
        small_buffer[i] = byte.to_owned();
    }

    let medium_bytes = include_bytes!("../resources/icon/icon_32x32.png");
    let mut medium_buffer = [0u8; 4096];
    for (i, byte) in medium_bytes.iter().enumerate() {
        medium_buffer[i] = byte.to_owned();
    }

    let big_bytes = include_bytes!("../resources/icon/icon_64x64.png");
    let mut big_buffer = [0u8; 16384];
    for (i, byte) in big_bytes.iter().enumerate() {
        big_buffer[i] = byte.to_owned();
    }
    
    Icon {
        small: small_buffer,
        medium: medium_buffer,
        big: big_buffer,
    }
}