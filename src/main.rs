mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;

use macroquad::prelude::*;
use crate::game_state::GameState;


#[macroquad::main(main_window_conf)]
async fn main() {
    let game_state = GameState::from_file("./save-file.txt")
        .await
        .unwrap_or(GameState::new("temp"));

    loop {
        clear_background(WHITE);

        draw_text(game_state.friend().name(), 100.0, 100.0, 14.0, DARKGRAY);

        let test = game_state.friend().shape().get_texture();

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