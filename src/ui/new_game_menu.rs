use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;

pub async fn render_new_game_menu() -> GameState {
    let mut name_buffer = String::new();
    let mut backspace_timer = 0.0;
    
    let confirm_btn = Button {
        pos: Vec2::new(75.0, 150.0),
        text: String::from("confirm"),
        ..Default::default()
    };

    loop {
        let mouse_pos = mouse_position();
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));

        // Render components
        confirm_btn.render(mouse_pos.into());

        let text_size = measure_text(&name_buffer, None, 28, 1.0);

        let text_x = (200.0 - text_size.width) / 2.0;
        let text_y = (200.0 - text_size.height) / 2.0 - 15.0;

        draw_text(
            &name_buffer,
            text_x,
            text_y,
            28.0,
            BLACK,
        );

        // Check for confirm button clicked
        let col_rect = confirm_btn.collision_rect();
        if col_rect.contains(mouse_pos.into()) && is_mouse_button_pressed(MouseButton::Left) {
            break;
        }

        // Store user input into buffer
        while let Some (char) = get_char_pressed() {
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
