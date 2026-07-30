use macroquad::prelude::*;

use crate::items::BuyableItem;
use crate::{include_texture, SCREEN_WIDTH};
use crate::items::inventory::Inventory;
use crate::items::creature_color::CreatureColor;


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
    const ITEM_HEIGHT: f32 = 20.0;
    const X_LOCATION: f32 = 10.0;
    
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
            self.area.x + 2.0,
            self.area.y + 2.0,
            BLACK,
            DrawTextureParams::default(),
        );

        draw_text(
            self.item.name(),
            self.area.x + 10.0,
            self.area.y + 2.0,
            16.0,
            BLACK,
        );

        draw_text(
            &format!("price: {}", self.item.price()),
            self.area.x + self.area.w - 10.0,
            self.area.y + 10.0,
            14.0,
            Color { r: 0.85, g: 0.85, b: 0.85, a: 1.0 },
        );
    }
}

pub fn get_unowned_shop_items(inv: &Inventory) -> Vec<ShopItem> {
    // TODO: Filter items based on owned items from inventory
    default_shop_items().into()
}

fn default_shop_items() -> [ShopItem; 3] {
    [
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
