mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;
mod save_management;
mod movements;

use macroquad::prelude::*;
use crate::game_state::GameState;
use save_management::get_save_file_path;
use ui::render_new_game_menu;
use crate::ui::stat_display::stat_display;
use ui::interaction_buttons::InteractionButton;
use crate::food::Food;
use crate::movements::get_creature_movement;
use crate::utils::Location;

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
    let buttons = InteractionButton::main_menu_buttons();
    let mut creature_movement = get_creature_movement(
        state.friend(),
        Location { x: 100.0, y: 50.0 }
    );

    loop {
        state.update();
        let mouse_pos = mouse_position();
        handle_button_click(&buttons, &mut state, mouse_pos.into());

        clear_background(Color::new(0.8, 0.8, 0.8, 1.0));

        let friend_texture = state.friend().shape();
        let friend_location = creature_movement.next_position();
        draw_texture(&friend_texture, friend_location.x, friend_location.y, BLACK);
        
        draw_text(state.friend().name(), 100.0, 20.0, 16.0, BLACK);

        for button in &buttons {
            button.get_button().render(mouse_pos.into());
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

fn handle_button_click(buttons: &[InteractionButton], game_state: &mut GameState, mouse_pos: Vec2) {
    for button in buttons {
        let button_area = button.get_button().collision_rect();
        if button_area.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left) {
            match button {
                InteractionButton::Food(_) => game_state.friend_mut().eat(Food::new_random()),
                InteractionButton::Joy(_) => game_state.friend_mut().play(),
                InteractionButton::Energy(_) => game_state.friend_mut().toggle_sleep(),
                InteractionButton::Health(_) => game_state.friend_mut().take_medicine(),
            }
        }
    }
}