pub mod interaction_menu;
mod interaction_menu_item;

use macroquad::prelude::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};


pub use interaction_menu::InteractionMenu;
use interaction_menu_item::InteractionMenuItem;

pub const ITEM_CONTAINER_AREA: Rect = Rect {
    x: (SCREEN_WIDTH as f32 - 150.0) / 2.0,
    y: (SCREEN_HEIGHT as f32 - 120.0) / 2.0,
    w: 150.0,
    h: 120.0,
};

pub trait CreatureInteraction: Clone + Copy {
    fn name(&self) -> &str;
    fn sprite(&self) -> Texture2D;
}
