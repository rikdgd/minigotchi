mod friend;
mod food;
mod shapes;
mod utils;

use macroquad::prelude::*;
use crate::shapes::CreatureShapes;

#[macroquad::main(main_window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_text("Hello, Macroquad!", 100.0, 100.0, 14.0, DARKGRAY);

        let test = CreatureShapes::Squid.get_texture();

        draw_texture(&test, 10.0, 10.0, BLACK);

        next_frame().await;
    }
}

fn main_window_conf() -> Conf {
    Conf {
        window_title: "minigotchi".to_string(),
        window_width: 200,
        window_height: 200,
        window_resizable: false,
        ..Default::default()
    }
}