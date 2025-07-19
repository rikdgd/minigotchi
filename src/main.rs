mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;

use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;
use ui::render_new_game_menu;

pub const SCREEN_WIDTH: i32 = 200;
pub const SCREEN_HEIGHT: i32 = 200;


#[macroquad::main(main_window_conf)]
async fn main() {
    let game_state = GameState::from_file("./save-file.txt")
        .await
        .unwrap_or(render_new_game_menu().await);


    render_game(game_state).await;
}

async fn render_game(state: GameState) {
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

        let friend_texture = state.friend().shape().get_texture();
        draw_texture(&friend_texture, 10.0, 10.0, BLACK);

        for button in &buttons {
            button.render(mouse_pos.into());
        }

        next_frame().await;
    }
}

fn main_window_conf() -> Conf {
    Conf {
        window_title: "minigotchi".to_string(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}