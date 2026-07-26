use macroquad::color::Color;
use serde::{Serialize, Deserialize};
use crate::items::{BuyableItem, ItemType};
use crate::items::inventory::Inventory;


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CreatureColor {
    Red,
    Green,
    Blue,
}

impl CreatureColor {
    pub fn get_color(&self) -> Color {
        match self {
            CreatureColor::Red => Color::new(0.9, 0.0, 0.0, 1.0),
            CreatureColor::Green => Color::new(0.0, 0.9, 0.0, 1.0),
            CreatureColor::Blue => Color::new(0.0, 0.0, 0.9, 1.0),
        }
    }
}

impl BuyableItem for CreatureColor {
    fn name(&self) -> &str {
        match self {
            CreatureColor::Red => "Red color",
            CreatureColor::Green => "Green color",
            CreatureColor::Blue => "Blue color",
        }
    }

    fn price(&self) -> u32 {
        match self {
            CreatureColor::Red => 5,
            CreatureColor::Green => 5,
            CreatureColor::Blue => 5,
        }
    }

    fn item_type(&self) -> ItemType {
        ItemType::CreatureColor
    }

    fn add_to_inventory(&self, inventory: &mut Inventory) {
        inventory.creature_colors.push(*self);
    }
}
