use macroquad::prelude::*;

use crate::items::BuyableItem;
use crate::{include_texture, SCREEN_WIDTH};
use crate::items::inventory::Inventory;
use crate::items::creature_color::CreatureColor;


// The sprites for items in the shop are always 15x15 pixels
const ITEM_SPRITE_DIMENSION: f32 = 15.0;

/// The `ShopItem` structure can be used to render a buyable item in the shop menu.
/// It manages its own state and can be rendered using the `render()` function.
///
/// ## Fields:
/// * `item` - The `BuyableItem` instance that is sold via this ShopItem.
/// * `sprite` - The item's sprite that should be rendered in the shop
/// * `area` - The hitbox/size of the item when rendered on screen.
pub struct ShopItem {
    pub item: Box<dyn BuyableItem>,
    pub sprite: Texture2D,
    area: Rect,
}

impl ShopItem {
    const ITEM_WIDTH: f32 = (SCREEN_WIDTH as f32 * 0.9).round();
    const ITEM_HEIGHT: f32 = 30.0;
    const X_LOCATION: f32 = 10.0;
    const ITEM_NAME_FONT_SIZE: f32 = 16.0;
    const PRICE_FONT_SIZE: f32 = 14.0;
    
    /// Returns a new `ShopItem` instance.
    /// 
    /// ## Parameters:
    /// * `item` - The item that can be bought using the new ShopItem.
    /// * `sprite` - The sprite that should be rendered in the new ShopItem.
    /// * `item_index` - The index for this item in the list of all shop items. This is used
    ///   to determine at what height the ShopItem should be rendered.
    pub fn new(item: Box<dyn BuyableItem>, sprite: Texture2D, item_index: u32) -> Self {
        Self {
            item,
            sprite,
            area: Rect::new(
                Self::X_LOCATION,
                item_index as f32 * (Self::ITEM_HEIGHT * 1.2).round() + 10.0,
                Self::ITEM_WIDTH,
                Self::ITEM_HEIGHT,
            ),
        }
    }

    pub fn is_clicked(&self) -> bool {
        self.area.contains(mouse_position().into()) && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.area.x,
            self.area.y,
            self.area.w,
            self.area.h,
            Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }
        );

        draw_texture_ex(
            &self.sprite,
            self.area.x + 8.0,
            (self.area.y + (self.area.h - ITEM_SPRITE_DIMENSION) / 2.0).round(),
            BLACK,
            DrawTextureParams::default(),
        );

        draw_text(
            self.item.name(),
            self.area.x + 16.0 + ITEM_SPRITE_DIMENSION,
            self.area.y + (self.area.h + Self::ITEM_NAME_FONT_SIZE / 2.0) / 2.0,
            Self::ITEM_NAME_FONT_SIZE,
            BLACK,
        );

        let price_txt = format!("{}$", self.item.price());
        let price_dimensions = measure_text(&price_txt, None, Self::PRICE_FONT_SIZE as u16, 1.0);
        draw_text(
            &price_txt,
            self.area.x + self.area.w - price_dimensions.width - 8.0,
            (self.area.y + (self.area.h + Self::PRICE_FONT_SIZE / 2.0) / 2.0).round(),
            Self::PRICE_FONT_SIZE,
            Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
        );
    }
}

pub fn get_unowned_shop_items(inv: &Inventory) -> Vec<ShopItem> {
    let mut new_items = default_shop_items();
    new_items.retain(|item| !inv.contains_item(&item.item));
    new_items
}

fn default_shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem::new(
            Box::new(CreatureColor::Red),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            0,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Green),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            1,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Blue),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            2,
        ),
    ]
}
