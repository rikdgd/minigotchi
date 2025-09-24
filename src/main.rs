mod friend;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;
mod save_management;
mod movements;

use macroquad::prelude::*;
use game_state::GameState;
use save_management::get_save_file_path;
use ui::{render_new_game_menu, render_death_screen};
use ui::stat_display::stat_display;
use ui::interaction_buttons::InteractionButton;
use food::Food;
use movements::{get_creature_movement, get_sleeping_location};
use utils::Location;
use ui::play_area::draw_play_area;
use shapes::sleeping_icon;
use movements::{CreatureMovement, EggHop};

pub const SCREEN_WIDTH: i32 = 200;
pub const SCREEN_HEIGHT: i32 = 200;
pub const CREATURE_BASE_LOCATION: Location = Location { x: 100.0, y: 50.0 };
pub const BACKGROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);


#[macroquad::main(main_window_conf)]
async fn main() {
    // Seed the random number generator
    rand::srand(miniquad::date::now() as u64);
    
    let save_file_path = get_save_file_path();

    let game_state = match GameState::from_file(&save_file_path).await {
        Ok(mut state) => {
            state.update();

            if !state.friend().alive() {
                render_death_screen(&state).await
            } else {
                state
            }
        },
        Err(_) => render_new_game_menu().await,
    };

    render_game(game_state).await;
}

async fn render_game(mut state: GameState) {
    let buttons = InteractionButton::main_menu_buttons();
    let mut creature_movement = get_creature_movement(
        state.friend(),
        CREATURE_BASE_LOCATION
    );
    let mut sleeping_icon_movement = EggHop::new(get_sleeping_location(state.friend()).translate(-8.0, -12.0));

    loop {
        state.update();
        handle_button_click(&buttons, &mut state);

        clear_background(BACKGROUND_COLOR);
        
        // Draw the playing area the creature walks around in
        draw_play_area(state.friend());
        
        // Draw the creature at the correct location:
        let friend_texture = state.friend().shape();
        let friend_location = if state.friend().is_asleep() {
            get_sleeping_location(state.friend())
        } else {
            creature_movement.next_position()
        };
        draw_texture(&friend_texture, friend_location.x, friend_location.y, BLACK);

        // Update the creature's movement when it changes growth stage
        if state.prev_growth_stage != state.friend().growth_stage() {
            creature_movement = get_creature_movement(
                state.friend(),
                CREATURE_BASE_LOCATION
            );

            state.prev_growth_stage = state.friend().growth_stage();
        }

        // Draw the "Zz" texture when sleeping
        if state.friend().is_asleep() {
            let location = sleeping_icon_movement.next_position();
            draw_texture(&sleeping_icon(), location.x, location.y, WHITE);
        }
        
        draw_text(state.friend().name(), 100.0, 20.0, 16.0, BLACK);

        for button in &buttons {
            button.get_button().render();
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

fn handle_button_click(buttons: &[InteractionButton], game_state: &mut GameState) {
    for button in buttons {
        if button.get_button().is_clicked() {
            match button {
                InteractionButton::Food(_) => game_state.friend_mut().eat(Food::new_random()),
                InteractionButton::Joy(_) => game_state.friend_mut().play(),
                InteractionButton::Energy(_) => game_state.friend_mut().toggle_sleep(),
                InteractionButton::Health(_) => game_state.friend_mut().take_medicine(),
            }
        }
    }
}