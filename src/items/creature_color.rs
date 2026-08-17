use std::f32::consts::PI;
use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use crate::items::{BuyableItem, ItemType};
use crate::items::inventory::Inventory;
use crate::utils::time::get_now_millis;


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CreatureColor {
    Black,
    Red,
    Green,
    Blue,
    Pink,
    Yellow,
    Cyan,
    Rainbow,
}

impl CreatureColor {
    pub fn get_color(&self) -> Color {
        match self {
            CreatureColor::Black => Color::new(0.0, 0.0, 0.0, 1.0),
            CreatureColor::Red => Color::new(0.6, 0.0, 0.0, 1.0),
            CreatureColor::Green => Color::new(0.0, 0.6, 0.0, 1.0),
            CreatureColor::Blue => Color::new(0.0, 0.0, 0.6, 1.0),
            CreatureColor::Pink => Color::new(0.8, 0.0, 0.8, 1.0),
            CreatureColor::Yellow => Color::new(0.9, 0.9, 0.0, 1.0),
            CreatureColor::Cyan => Color::new(0.0, 0.9, 0.9, 1.0),
            CreatureColor::Rainbow => Self::get_rainbow_color(),
        }
    }
    
    fn get_rainbow_color() -> Color {
        let x = (get_now_millis() % 1000_1000 / 100) as f32 * 0.25;

        let r = Self::rainbow_sinus(x);
        let g = Self::rainbow_sinus(x + PI * ( 2.0 / 3.0 ));
        let b = Self::rainbow_sinus(x + PI * ( 4.0 / 3.0 ));
        
        Color { r, g, b, a: 1.0 }
    }
    
    fn rainbow_sinus(x: f32) -> f32 {
        f32::sin(x) * 0.5 + 0.5
    }
}

impl BuyableItem for CreatureColor {
    fn name(&self) -> &str {
        match self {
            CreatureColor::Black => "Black color",
            CreatureColor::Red => "Red color",
            CreatureColor::Green => "Green color",
            CreatureColor::Blue => "Blue color",
            CreatureColor::Pink => "Pink color",
            CreatureColor::Yellow => "Yellow color",
            CreatureColor::Cyan => "Cyan color",
            CreatureColor::Rainbow => "Rainbow color",
        }
    }

    fn price(&self) -> u32 {
        match self {
            CreatureColor::Black => 0,
            CreatureColor::Red => 3,
            CreatureColor::Green => 3,
            CreatureColor::Blue => 3,
            CreatureColor::Pink => 5,
            CreatureColor::Yellow => 5,
            CreatureColor::Cyan => 5,
            CreatureColor::Rainbow => 10,
        }
    }

    fn item_type(&self) -> ItemType {
        ItemType::CreatureColor
    }

    fn add_to_inventory(&self, inventory: &mut Inventory) {
        inventory.creature_colors.push(*self);
    }
    
    fn try_equip(&self, inventory: &mut Inventory) -> Result<(), &str> {
        if !inventory.creature_colors.contains(self) {
            return Err("Not owned");
        }
        
        inventory.equipped_color = *self;
        Ok(())
    }
    
    fn is_equipped(&self, inventory: &Inventory) -> bool {
        inventory.equipped_color == *self
    }
}
