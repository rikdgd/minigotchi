use macroquad::prelude::Texture2D;
use crate::include_texture;
use crate::ui::interaction_menu::CreatureInteraction;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Food {
    Soup,
    Cookie,
    Burger,
}

impl Food {
    pub fn points(&self) -> u8 {
        match self {
            Food::Soup => 20,
            Food::Cookie => 30,
            Food::Burger => 40,
        }
    }
}

impl CreatureInteraction for Food {
    fn name(&self) -> &str {
        match self {
            Food::Soup => "Soup",
            Food::Cookie => "Cookie",
            Food::Burger => "Burger",
        }
    }

    fn sprite(&self) -> Texture2D {
        match self {
            Food::Soup => include_texture!("../resources/animations/eating/soup0.png"),
            Food::Cookie => include_texture!("../resources/animations/eating/cookie0.png"),
            Food::Burger => include_texture!("../resources/animations/eating/burger0.png"),
        }
    }
    
    fn menu_title() -> String {
        "Select food".to_string()
    }
}
