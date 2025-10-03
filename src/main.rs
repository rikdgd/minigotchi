mod creature;
mod food;
mod shapes;
mod utils;
mod game_state;
mod ui;
mod save_management;
mod movements;
mod animations;

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
use crate::animations::Animation;
use crate::animations::creature_eat::CreatureEatAnimation;

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

            if !state.creature().alive() {
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
    // Set some state
    let buttons = InteractionButton::main_menu_buttons();
    let mut creature_movement = get_creature_movement(
        state.creature(),
        CREATURE_BASE_LOCATION
    );
    let mut sleeping_icon_movement = EggHop::new(get_sleeping_location(state.creature()).translate(-9.0, -16.0));
    let mut current_animation: Option<Box<dyn Animation>> = None;

    // Enter the actual game loop
    loop {
        state.update();
        handle_button_click(&buttons, &mut state, &mut current_animation);

        clear_background(BACKGROUND_COLOR);
        
        // Draw the playing area the creature walks around in
        draw_play_area(state.creature());
        
        // Draw the creature at the correct location:
        let creature_texture = state.creature().shape();
        let creature_location = if state.creature().is_asleep() {
            get_sleeping_location(state.creature())
        } else {
            creature_movement.next_position()
        };
        draw_texture(&creature_texture, creature_location.x, creature_location.y, BLACK);

        // Update the creature's movement when it changes growth stage
        if state.prev_growth_stage != state.creature().growth_stage() {
            creature_movement = get_creature_movement(
                state.creature(),
                CREATURE_BASE_LOCATION
            );

            state.prev_growth_stage = state.creature().growth_stage();
        }

        // Draw the "Zz" texture when sleeping
        if state.creature().is_asleep() {
            let location = sleeping_icon_movement.next_position();
            draw_texture(&sleeping_icon(), location.x, location.y, WHITE);
        }
        
        // Draw the creatures name and age
        ui::draw_creature_name(&state);
        ui::draw_age_display(&state);

        for button in &buttons {
            button.get_button().render();
        }
        
        // If an animation is playing, render it
        if let Some(animation) = current_animation.as_mut() {
            if animation.playing() {
                animation.render();
            } else {
                current_animation = None;
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        stat_display(state.creature());

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

fn handle_button_click(buttons: &[InteractionButton], game_state: &mut GameState, animation: &mut Option<Box<dyn Animation>>) {
    let creature = game_state.creature_mut();

    for button in buttons {
        if button.get_button().is_clicked() {
            match button {
                InteractionButton::Energy(_) => creature.toggle_sleep(),

                InteractionButton::Food(_) => {
                    if !creature.is_asleep() && creature.food().value() != 100 {
                        creature.eat(Food::new_random());
                        *animation = Some(Box::new(CreatureEatAnimation::new()));
                    }
                },
                InteractionButton::Joy(_) => {
                    if !creature.is_asleep() && creature.joy().value() != 100 {
                        creature.play();
                    }
                },
                InteractionButton::Health(_) => {
                    if !creature.is_asleep() && creature.health().value() != 100 {
                        creature.take_medicine();
                    }
                },
            }
        }
    }
}