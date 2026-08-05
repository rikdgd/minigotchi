use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

use crate::items::{BuyableItem, ItemType};
use crate::items::inventory::Inventory;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameBackground {
    Plain,
}

impl GameBackground {
    pub fn get_background(&self) -> Texture2D {
        todo!()
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