use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::{BuyableItem, ItemType};
use crate::items::inventory::Inventory;
use crate::ui::play_area::PLAY_AREA_RECT;

use crate::include_texture;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameBackground {
    Plain,
}

impl GameBackground {
    pub fn render(&self) {
        let bg_texture: Option<Texture2D> = match self {
            GameBackground::Plain => None,
        };
        
        if let Some(bg_texture) = bg_texture {
            draw_texture_ex(
                &bg_texture,
                PLAY_AREA_RECT.x,
                PLAY_AREA_RECT.y,
                Color { r: 0.0, g: 0.0, b: 0.0, a: 0.4 },
                DrawTextureParams::default(),
            );
        }
    }
}

impl BuyableItem for GameBackground {
    fn name(&self) -> &str {
        match self {
            GameBackground::Plain => "Plain background",
        }
    }

    fn price(&self) -> u32 {
        match self {
            GameBackground::Plain => 0,
        }
    }

    fn item_type(&self) -> ItemType {
        ItemType::Background
    }

    fn add_to_inventory(&self, inventory: &mut Inventory) {
        inventory.backgrounds.push(*self);
    }

    fn try_equip(&self, inventory: &mut Inventory) -> Result<(), &str> {
        if !inventory.backgrounds.contains(self) {
            return Err("Not owned");
        }
        
        inventory.equipped_background = *self;
        Ok(())
    }

    fn is_equipped(&self, inventory: &Inventory) -> bool {
        inventory.equipped_background == *self
    }
}