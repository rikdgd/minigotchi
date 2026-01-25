use macroquad::prelude::*;
use crate::ui::button::Button;


/// A wrapper for the four main menu buttons, each of the possible values has a body containing
/// a `Button` ui element.
pub enum InteractionButton {
    Food(Button),
    Joy(Button),
    Energy(Button),
    Health(Button),
}

impl InteractionButton {
    /// Returns a hardcoded vector containing the four `InteractionButtons` on the main menu.
    pub fn main_menu_buttons() -> Vec<InteractionButton> {
        vec![
            InteractionButton::Food(Button { 
                pos: Vec2::new(0.0, 180.0),
                text: "Eat".to_string(),
                ..Default::default() 
            }),
            InteractionButton::Joy(Button { 
                pos: Vec2::new(50.0, 180.0), 
                text: "Play".to_string(),
                ..Default::default() 
            }),
            InteractionButton::Energy(Button { 
                pos: Vec2::new(100.0, 180.0), 
                text: "Sleep".to_string(),
                ..Default::default() 
            }),
            InteractionButton::Health(Button { 
                pos: Vec2::new(150.0, 180.0),
                text: "Health".to_string(),
                ..Default::default() 
            }),
        ]
    }
    
    /// Gets the button element held by this `InteractionButton` wrapper.
    pub fn get_button(&self) -> &Button {
        match self {
            InteractionButton::Food(button) => button,
            InteractionButton::Joy(button) => button,
            InteractionButton::Energy(button) => button,
            InteractionButton::Health(button) => button,
        }
    }
}