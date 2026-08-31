use macroquad::prelude::*;
use crate::game_state::GameState;
use crate::shapes::CreatureShape;
use crate::ui::button::Button;
use crate::ui::creature_selection::CreatureSelection;
use crate::{BACKGROUND_COLOR, SCREEN_WIDTH, SCREEN_HEIGHT};


/// The **NewGameMenu** structure is used to render the new game menu where the user can select a
/// creature and give it a name. To do this it first renders the `CreatureSelection` menu to allow
/// the user to select a shape, and then the `NewGameMenu` asks the user for a name and creates a
/// `GameState` instance.
/// 
/// ## Fields:
/// * `name_buffer` - The name buffer is used to store the name entered by the user.
/// * `selected_shape` - Holds the shape that was selected by the user using the `CreatureSelection`
///   menu
/// * `backspace_timer` - The backspace timer is used to debounce the backspace key, making it easier
///   to edit the creature's name.
/// * `confirm_btn` - The button the user clicks to confirm game creation.
/// 
/// ## Methods:
/// * `render` - The render method renders the menu, and simultaniously updates the state.
/// 
/// ## Example:
/// ```rust
/// async fn example() {
///     let menu: NewGameMenu = NewGameMenu::default();
///     let state: GameState = menu.render().await;
/// 
///     // TODO: Use the new GameState to run the game
/// }
/// ```
#[derive(Debug, Clone)]
pub struct NewGameMenu {
    name_buffer: String,
    selected_shape: Option<CreatureShape>,
    backspace_timer: f32,
    confirm_btn: Button,
}

impl NewGameMenu {
    const NAME_MESSAGE: &str = "Enter name:";
    
    /// Renders the new game menu and updates it state.
    pub async fn render(&mut self) -> GameState {
        // First render the shape selection ui and get the user selected shape.
        let selected_shape = CreatureSelection::default().render().await;
        self.selected_shape = Some(selected_shape);
        
        let message_text_size = measure_text(
            Self::NAME_MESSAGE, 
            None,
            20,
            1.0,
        );
        
        let new_game_state: GameState;
        
        loop {
            clear_background(BACKGROUND_COLOR);
            
            self.confirm_btn.render();
            
            draw_text(
                Self::NAME_MESSAGE,
                (SCREEN_WIDTH as f32 - message_text_size.width) / 2.0,
                (SCREEN_HEIGHT as f32 - message_text_size.height) / 2.0 - 45.0,
                20.0,
                BLACK,
            );
            
            let name_text_size = measure_text(&self.name_buffer, None, 28, 1.0);
            draw_text(
                &self.name_buffer,
                (SCREEN_WIDTH as f32 - name_text_size.width) / 2.0,
                (SCREEN_HEIGHT as f32 - name_text_size.height) / 2.0 - 15.0,
                28.0,
                BLACK,
            );
            
            next_frame().await;
            
            if let Some(state) = self.update() {
                new_game_state = state;
                break;
            }
        }
        
        new_game_state
    }
    
    fn update(&mut self) -> Option<GameState> {
        if self.confirm_btn.is_clicked() && !self.name_buffer.is_empty() {
            return self.try_create_game_state();
        }

        // Store user input into buffer
        while let Some(char) = get_char_pressed() {
            self.name_buffer.push(char);
        }

        self.backspace_timer += get_frame_time();
        if is_key_down(KeyCode::Backspace) && self.backspace_timer > 0.02 {
            self.name_buffer.pop();
            self.backspace_timer = 0.0;
        }
        
        None
    }
    
    fn try_create_game_state(&self) -> Option<GameState> {
        Some(GameState::new(
            &self.name_buffer, 
            self.selected_shape?
        ))
    }
}

impl Default for NewGameMenu {
    fn default() -> Self {
        Self {
            name_buffer: String::new(),
            selected_shape: None,
            confirm_btn: Button {
                pos: Vec2::new(75.0, 150.0),
                text: String::from("confirm"),
                ..Default::default()
            },
            backspace_timer: 0.0,
        }
    }
}
