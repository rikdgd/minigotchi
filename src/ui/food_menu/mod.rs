mod food_menu;
mod food_menu_item;

use macroquad::prelude::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};


pub use food_menu::FoodMenu;
use food_menu_item::FoodMenuItem;

pub const ITEM_CONTAINER_AREA: Rect = Rect {
    x: (SCREEN_WIDTH as f32 - 150.0) / 2.0,
    y: (SCREEN_HEIGHT as f32 - 120.0) / 2.0,
    w: 150.0,
    h: 120.0,
};
