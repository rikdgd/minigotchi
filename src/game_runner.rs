use macroquad::prelude::*;

use crate::game_state::GameState;
use crate::save_management::get_save_file_path;
use crate::ui::{NewGameMenu, render_death_screen, button::Button};
use crate::ui::interaction_menu::InteractionMenu;
use crate::ui::interaction_menu::menu_item_generation::gen_interaction_items;
use crate::ui::stat_display::stat_display;
use crate::ui::interaction_buttons::InteractionButton;
use crate::movements::get_sleeping_location;
use crate::ui::play_area::draw_play_area;
use crate::shapes::sleeping_icon;
use crate::movements::{CreatureMovement, EggHop};
use crate::animations::creature_actions::{ActionAnimationType, CreatureActionAnimation};
use crate::animations::emotions::{EmotionAnimation, EmotionAnimationType};
use crate::{ui, BACKGROUND_COLOR};
use crate::creature::GrowthStage;
use crate::food::Food;
use crate::creature_game::CreatureGame;
use crate::ui::shop::ShopPage;
use crate::utils::Location;
use crate::ui::play_area::play_area_background_color;


/// The **GameRunner** structure can be used to run the Minigotchi game. It has ownership of the
/// `GameState` instance used to run the game and makes sure the correct page/screen is rendered.
///
/// ## Example:
/// ```rust
/// use macroquad::prelude::*;
///
/// #[macroquad::main]
/// async fn main() {
///     let mut runner = GameRunner::initiate().await;
///     runner.run_game().await;
/// }
/// ```
pub struct GameRunner {
    state: GameState,
    
    interaction_buttons: [InteractionButton; 4],
    sleep_icon_movement: EggHop,
    shop_button: Button,
}

impl GameRunner {
    /// Creates a new `GameRunner` instance. To do so it first checks if a save file is available,
    /// as well as if the save is still valid. If it isn't it will display the correct game menu
    /// to let the user create a `GameState`. This GameState can then be used to create a new
    /// GameRunner.
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
            interaction_buttons: InteractionButton::main_menu_buttons(),
            sleep_icon_movement: EggHop::new(get_sleeping_location(game_state.creature()).translate(-9.0, -16.0)),
            shop_button: ShopPage::shop_button(),

            state: game_state,
        }
    }

    pub async fn run_game(&mut self) {
        loop {
            self.state.update();
            
            // If the creature has died, render the death screen and set the new state
            if !self.state.creature().alive() {
                self.state = render_death_screen(&self.state).await;
            }
            
            clear_background(BACKGROUND_COLOR);
            self.draw_main_ui();
            
            // If an animation is playing, render it
            if let Some(animation) = self.state.current_animation.as_mut()
                && animation.playing(){
                animation.render();
            }
            
            // Handle user inputs:
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            self.handle_button_click().await;
            self.handle_shop_button_click().await;
            
            next_frame().await;
        }
    }

    fn draw_main_ui(&mut self) {
        draw_play_area(self.state.creature());
        self.state.inventory.equipped_background.render();
        self.draw_creature();
        
        stat_display(self.state.creature());
        
        // Draw the "Zz" texture when sleeping
        if self.state.creature().is_asleep() {
            let location = self.sleep_icon_movement.next_location();
            draw_texture(&sleeping_icon(), location.x, location.y, BLACK);
        }
        
        // Draw the creatures name and age
        ui::draw_creature_name(&self.state);
        ui::draw_age_display(&self.state);
        
        for button in &self.interaction_buttons {
            button.get_button().render();
        }
        self.shop_button.render();
    }

    fn draw_creature(&mut self) {
        // The creature shouldn't be drawn when an animation is playing.
        if self.state.current_animation.is_some() {
            return;
        }

        let creature_texture = self.state.creature().texture();
        let creature_location = if self.state.creature().is_asleep() {
            get_sleeping_location(self.state.creature())
        } else {
            self.state.creature_movement.next_location()
        };
        
        self.draw_creature_backdrop(creature_location);
        draw_texture_ex(
            &creature_texture,
            creature_location.x,
            creature_location.y,
            self.state.inventory.equipped_color.get_color(),
            DrawTextureParams {
                flip_x: self.state.creature_movement.mirror_sprite(),
                ..Default::default()
            }
        );
    }
    
    fn draw_creature_backdrop(&self, draw_loc: Location) {
        if self.state.creature().growth_stage() != GrowthStage::Adult {
            return;
        }
        
        let backdrop = self.state.creature().shape().get_backdrop();
        draw_texture_ex(
            &backdrop,
            draw_loc.x,
            draw_loc.y,
            play_area_background_color(self.state.creature().is_asleep()),
            DrawTextureParams {
                flip_x: self.state.creature_movement.mirror_sprite(),
                ..Default::default()
            }
        );
    }

    async fn handle_button_click(&mut self) {
        if self.state.current_animation.is_some()
            || self.state.creature().growth_stage() == GrowthStage::Egg
        {
            return;
        }
        
        for button in &self.interaction_buttons {
            if button.get_button().is_clicked() {
                match button {
                    InteractionButton::Energy(_) => self.state.creature_mut().toggle_sleep(),
                    
                    InteractionButton::Food(_) => {
                        let creature = self.state.creature_mut();
                        if !creature.is_asleep()
                            && creature.food().value() != 100
                            && !creature.is_sick()
                        {
                            let mut menu: InteractionMenu<Food> = InteractionMenu::new(gen_interaction_items());
                            if let Some(food) = menu.render().await {
                                creature.eat(food);
                                self.state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Eating(food)));
                            }
                        }
                    },
                    InteractionButton::Joy(_) => {
                        let creature = self.state.creature_mut();
                        if !creature.is_asleep() && creature.joy().value() != 100 {
                            let mut menu: InteractionMenu<CreatureGame> = InteractionMenu::new(gen_interaction_items());
                            if let Some(game) = menu.render().await {
                                if game.energy_cost() <= creature.energy().value() {
                                    creature.play(game);
                                    self.state.set_animation(CreatureActionAnimation::new(ActionAnimationType::Play(game)));
                                } else {
                                    // Display a short animation to let the player know the interaction
                                    // was unsuccessful.
                                    self.state.set_animation(EmotionAnimation::new(EmotionAnimationType::Tired));
                                }
                            }
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
    
    async fn handle_shop_button_click(&mut self) {
        if !self.shop_button.is_clicked() {
            return;
        }
        
        let mut shop = ShopPage::new(&mut self.state.inventory);
        shop.render().await;
    }
}
