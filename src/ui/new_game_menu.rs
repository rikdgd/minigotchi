use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::ui::button::Button;

pub async fn render_new_game_menu() -> GameState {
    let mut name_buffer = String::new();
    
    let confirm_btn = Button {
        pos: Vec2::new(75.0, 150.0),
        text: String::from("confirm"),
        ..Default::default()
    };

    loop {
        let mouse_pos = mouse_position();
        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));

        confirm_btn.render(mouse_pos.into());
        let col_rect = confirm_btn.collision_rect();
        
        if col_rect.contains(mouse_pos.into()) && is_mouse_button_pressed(MouseButton::Left) {
            break;
        }

        next_frame().await;
    }
    
    GameState::new(&name_buffer)
}
