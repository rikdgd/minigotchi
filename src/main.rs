mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;

use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;


#[macroquad::main(main_window_conf)]
async fn main() {
    let game_state = GameState::from_file("./save-file.txt")
        .await
        .unwrap_or(GameState::new("temp"));

    let buttons = vec![
        Button { pos: Vec2::new(0.0, 180.0), ..Default::default() },
        Button { pos: Vec2::new(50.0, 180.0), ..Default::default() },
        Button { pos: Vec2::new(100.0, 180.0), ..Default::default() },
        Button { pos: Vec2::new(150.0, 180.0), ..Default::default() },
    ];

    loop {
        let delta_time = get_frame_time();
        let mouse_pos = mouse_position();

        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));

        let friend_texture = game_state.friend().shape().get_texture();
        draw_texture(&friend_texture, 10.0, 10.0, BLACK);

        for button in &buttons {
            button.render(mouse_position().into());
        }

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