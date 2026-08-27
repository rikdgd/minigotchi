use macroquad::prelude::Texture2D;
use crate::include_texture;
use crate::ui::interaction_menu::CreatureInteraction;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreatureGame {
    Drawing,
    Basketball,
    Frisbee,
}

impl CreatureGame {
    pub fn points(&self) -> u8 {
        match self {
            CreatureGame::Drawing => 20,
            CreatureGame::Basketball => 25,
            CreatureGame::Frisbee => 30,
        }
    }
    
    pub fn energy_cost(&self) -> u8 {
        match self {
            CreatureGame::Drawing => 15,
            CreatureGame::Basketball => 20,
            CreatureGame::Frisbee => 25,
        }
    }
}

impl CreatureInteraction for CreatureGame {
    fn name(&self) -> &str {
        match self {
            CreatureGame::Drawing => "Drawing",
            CreatureGame::Basketball => "Basketball",
            CreatureGame::Frisbee => "Frisbee",
        }
    }

    fn sprite(&self) -> Texture2D {
        match self {
            CreatureGame::Drawing => include_texture!("../resources/animations/playing/drawing2.png"),
            CreatureGame::Basketball => include_texture!("../resources/animations/playing/basketball0.png"),
            CreatureGame::Frisbee => include_texture!("../resources/animations/playing/frisbee0.png"),
        }
    }

    fn menu_title() -> String {
        "Select game".to_string()
    }

    fn all_variants() -> [Self; 3] {
        [
            CreatureGame::Drawing,
            CreatureGame::Basketball, 
            CreatureGame::Frisbee,
        ]
    }
}