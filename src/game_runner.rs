use macroquad::prelude::*;

// use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::save_management::get_save_file_path;
use crate::ui::{NewGameMenu, render_death_screen};
use crate::ui::stat_display::stat_display;
use crate::ui::interaction_buttons::InteractionButton;
use crate::food::Food;
use crate::movements::get_sleeping_location;
use crate::ui::play_area::draw_play_area;
use crate::shapes::sleeping_icon;
use crate::movements::{CreatureMovement, EggHop};
use crate::animations::creature_actions::{ActionAnimationType, CreatureActionAnimation};
use crate::{creature, ui, BACKGROUND_COLOR};
use crate::ui::shop::ShopPage;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMenu {
    Main,
    Shop,
}

/// The **GameRunner** structure can be used to run the Minigotchi game. It has ownership of the
/// `GameState` instance used to run the game and makes sure the correct page/screen is rendered.
///
/// ## Example:
/// ```rust
/// #[macroquad::main]
/// async fn main() {
///     let mut runner = GameRunner::initiate().await;
///     runner.render_game().await;
/// }
/// ```
pub struct GameRunner {
    state: GameState,
    current_menu: GameMenu,
}

impl GameRunner {
    /// Creates a new `GameRunner` instance. To do so it first checks if a save file is available,
    /// as well as if the save is still valid. If it isn't it will display the correct game menu
    /// to let the user create a `GameState`.
    pub async fn initiate() -> Self {
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
            Err(_) => NewGameMenu::default().render().await,
        };

        Self {
            state: game_state,
            current_menu: GameMenu::Main,
        }
    }

    pub async fn render_game(&mut self) {
        // Set some state
        let buttons = InteractionButton::main_menu_buttons();
        let mut sleeping_icon_movement = EggHop::new(get_sleeping_location(self.state.creature()).translate(-9.0, -16.0));

        let shop_btn = ShopPage::shop_button();

        // Enter the actual game loop
        loop {
            self.state.update();

            // If the creature has died, render the death screen and set the new state
            if !self.state.creature().alive() {
                self.state = render_death_screen(&self.state).await;
            }

            self.handle_button_click(&buttons);

            clear_background(BACKGROUND_COLOR);

            // Draw the playing area the creature walks around in
            draw_play_area(self.state.creature());
            self.draw_creature();

            // Draw the "Zz" texture when sleeping
            if self.state.creature().is_asleep() {
                let location = sleeping_icon_movement.next_location();
                draw_texture(&sleeping_icon(), location.x, location.y, WHITE);
            }

            // Draw the creatures name and age
            ui::draw_creature_name(&self.state);
            ui::draw_age_display(&self.state);

            for button in &buttons {
                button.get_button().render();
            }

            // If an animation is playing, render it
            if let Some(animation) = self.state.current_animation.as_mut()
                && animation.playing(){
                animation.render();
            }

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            stat_display(self.state.creature());
            shop_btn.render();


            next_frame().await;
        }
    }

    fn draw_creature(&mut self) {
        // The creature shouldn't be drawn when an animation is playing.
        if self.state.current_animation.is_some() {
            return;
        }

        let creature_texture = self.state.creature().shape();
        let creature_location = if self.state.creature().is_asleep() {
            get_sleeping_location(self.state.creature())
        } else {
            self.state.creature_movement.next_location()
        };

        draw_texture_ex(
            &creature_texture,
            creature_location.x,
            creature_location.y,
            BLACK,
            DrawTextureParams {
                flip_x: self.state.creature_movement.mirror_sprite(),
                ..Default::default()
            }
        );
    }

    fn handle_button_click(&mut self, buttons: &[InteractionButton]) {
        if self.state.current_animation.is_some() {
            return;
        }

        for button in buttons {
            if button.get_button().is_clicked() {
                match button {
                    InteractionButton::Energy(_) => self.state.creature_mut().toggle_sleep(),

                    InteractionButton::Food(_) => {
                        let creature = self.state.creature_mut();
                        if !creature.is_asleep()
                            && creature.food().value() != 100
                            && !creature.is_sick()
                        {
                            let food = Food::new_random();
                            creature.eat(food);
                            self.state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Eating(food)));
                        }
                    },
                    InteractionButton::Joy(_) => {
                        let creature = self.state.creature_mut();
                        if !creature.is_asleep()
                            && creature.joy().value() != 100
                            && creature.energy().value() >= creature::PLAYING_ENERGY_COST
                        {
                            creature.play();
                            self.state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Play));
                        }
                    },
                    InteractionButton::Health(_) => {
                        let creature = self.state.creature_mut();
                        if !creature.is_asleep() && creature.is_sick() {
                            creature.heal();
                            self.state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Health));
                        }
                    },
                }
            }
        }
    }
}