mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;
use ui::render_new_game_menu;

pub const SCREEN_WIDTH: i32 = 200;
pub const SCREEN_HEIGHT: i32 = 200;


#[macroquad::main(main_window_conf)]
async fn main() {
    let save_file_path = get_save_file_path();

    let game_state = match GameState::from_file(&save_file_path).await {
        Ok(state) => state,
        Err(_) => render_new_game_menu().await,
    };

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
        let mouse_pos = mouse_position();

        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));

        let friend_texture = state.friend().shape().get_texture();
        draw_texture(&friend_texture, 10.0, 10.0, BLACK);

        for button in &buttons {
            button.render(mouse_pos.into());
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }

    store_game_state(state).unwrap();
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

fn store_game_state(state: GameState) -> std::io::Result<()> {
    let file_path = get_save_file_path();

    let game_state = serde_json::to_string_pretty(&state)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;


    file.write_all(game_state.as_bytes())?;
    file.flush()?;

    Ok(())
}

fn get_save_file_path() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().expect("Executable must be in some directory");
    let data_file_path: PathBuf = exe_dir.join("save-file.txt");

    data_file_path.to_str().unwrap().to_string()
}