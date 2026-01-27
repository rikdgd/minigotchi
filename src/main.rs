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
use ui::play_area::PLAY_AREA_RECT;
use food::Food;
use movements::get_sleeping_location;
use utils::Location;
use ui::play_area::draw_play_area;
use shapes::sleeping_icon;
use movements::{CreatureMovement, EggHop};
use animations::creature_actions::{ActionAnimationType, CreatureActionAnimation};

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
    let mut sleeping_icon_movement = EggHop::new(get_sleeping_location(state.creature()).translate(-9.0, -16.0));

    // Enter the actual game loop
    loop {
        state.update();

        // If the creature has died, render the death screen and set the new state
        if !state.creature().alive() {
            state = render_death_screen(&state).await;
        }

        handle_button_click(&buttons, &mut state);
        
        clear_background(BACKGROUND_COLOR);
        
        // Draw the playing area the creature walks around in
        draw_play_area(state.creature());
        draw_creature(&mut state);

        // Draw the "Zz" texture when sleeping
        if state.creature().is_asleep() {
            let location = sleeping_icon_movement.next_position();
            draw_texture(&sleeping_icon(), location.x, location.y, WHITE);
        }
        
        // Draw the sickness icon when the creature is sick
        draw_sickness_icon(&state);
        
        // Draw the creatures name and age
        ui::draw_creature_name(&state);
        ui::draw_age_display(&state);

        for button in &buttons {
            button.get_button().render();
        }
        
        // If an animation is playing, render it
        if let Some(animation) = state.current_animation.as_mut() {
            if animation.playing() {
                animation.render();
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

fn draw_creature(state: &mut GameState) {
    // The creature shouldn't be drawn when an animation is playing.
    if state.current_animation.is_some() {
        return;
    }

    let creature_texture = state.creature().shape();
    let creature_location = if state.creature().is_asleep() {
        get_sleeping_location(state.creature())
    } else {
        state.creature_movement.next_position()
    };

    draw_texture_ex(
            &creature_texture,
            creature_location.x,
            creature_location.y,
            BLACK,
            DrawTextureParams {
                flip_x: state.creature_movement.mirror_sprite(),
                ..Default::default()
            }
        );
}

fn draw_sickness_icon(state: &GameState) {
    // The sprite has a width of 20 pixels: 'resources/status_icons/creature-sick.png'
    const SICKNESS_ICON_WIDTH: f32 = 20.0;
    
    if state.creature().is_sick() {
        let icon = shapes::creature_sick_icon();
        let draw_location = Location {
            x: PLAY_AREA_RECT.right() - SICKNESS_ICON_WIDTH - 2.0,
            y: PLAY_AREA_RECT.top() + 2.0,
        };
        
        draw_texture(
            &icon, 
            draw_location.x,
            draw_location.y, 
            BLACK,
        );
    }
}

fn handle_button_click(buttons: &[InteractionButton], game_state: &mut GameState) {
    if game_state.current_animation.is_some() {
        return;
    }
    
    for button in buttons {
        if button.get_button().is_clicked() {
            match button {
                InteractionButton::Energy(_) => game_state.creature_mut().toggle_sleep(),

                InteractionButton::Food(_) => {
                    let creature = game_state.creature_mut();
                    if !creature.is_asleep() && creature.food().value() != 100 {
                        let food = Food::new_random();
                        creature.eat(food);
                        game_state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Eating(food)));
                    }
                },
                InteractionButton::Joy(_) => {
                    let creature = game_state.creature_mut();
                    if !creature.is_asleep() && creature.joy().value() != 100 {
                        if creature.energy().value() >= creature::PLAYING_ENERGY_COST {
                            creature.play();
                            game_state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Play));

                        }

                    }
                },
                InteractionButton::Health(_) => {
                    let creature = game_state.creature_mut();
                    if !creature.is_asleep() && creature.is_sick() {
                        creature.take_medicine();
                        game_state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Health));
                    }
                },
            }
        }
    }
}