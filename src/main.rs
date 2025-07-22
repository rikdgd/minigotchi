mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;
mod save_management;

use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;
use save_management::get_save_file_path;
use ui::render_new_game_menu;
use crate::ui::stat_display::stat_display;

pub const SCREEN_WIDTH: i32 = 200;
pub const SCREEN_HEIGHT: i32 = 200;


#[macroquad::main(main_window_conf)]
async fn main() {
    // Seed the random number generator
    rand::srand(miniquad::date::now() as u64);
    
    let save_file_path = get_save_file_path();

    let game_state = match GameState::from_file(&save_file_path).await {
        Ok(state) => state,
        Err(_) => render_new_game_menu().await,
    };

    render_game(game_state).await;
}

async fn render_game(mut state: GameState) {
    let buttons = vec![
        Button { pos: Vec2::new(0.0, 180.0), ..Default::default() },
        Button { pos: Vec2::new(50.0, 180.0), ..Default::default() },
        Button { pos: Vec2::new(100.0, 180.0), ..Default::default() },
        Button { pos: Vec2::new(150.0, 180.0), ..Default::default() },
    ];


    loop {
        state.update();
        let mouse_pos = mouse_position();

        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));

        let friend_texture = state.friend().shape();
        draw_texture(&friend_texture, 10.0, 10.0, BLACK);

        for button in &buttons {
            button.render(mouse_pos.into());
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        stat_display(state.friend());

        next_frame().await;
    }
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

