use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};

pub async fn render_new_game_menu() -> GameState {
    let mut name_buffer = String::new();
    let mut backspace_timer = 0.0;
    
    let confirm_btn = Button {
        pos: Vec2::new(75.0, 150.0),
        text: String::from("confirm"),
        ..Default::default()
    };

    loop {
        clear_background(BACKGROUND_COLOR);

        confirm_btn.render();

        let text_size = measure_text(&name_buffer, None, 28, 1.0);
        draw_text(
            &name_buffer,
            (SCREEN_WIDTH as f32 - text_size.width) / 2.0,
            (SCREEN_HEIGHT as f32 - text_size.height) / 2.0 - 15.0,
            28.0,
            BLACK,
        );

        if confirm_btn.is_clicked() && name_buffer.len() != 0 {
            break;
        }

        // Store user input into buffer
        while let Some(char) = get_char_pressed() {
            name_buffer.push(char);
        }

        backspace_timer += get_frame_time();
        if is_key_down(KeyCode::Backspace) && backspace_timer > 0.02 {
            name_buffer.pop();
            backspace_timer = 0.0;
        }

        next_frame().await;
    }
    
    GameState::new(&name_buffer)
}
