use std::rc::Rc;
use crate::include_texture;
use crate::ui::shop::ShopItem;
use crate::items::{
    inventory::Inventory,
    creature_color::CreatureColor,
    game_background::GameBackground,
};


/// Generates all the `ShopItem` instances that are displayed on the `ShopPage`.
/// 
/// ## Parameters:
/// * `inventory` - A reference to the player's inventory. This is used to set the
///   state of each individual `ShopItem`.
pub fn generate_shop_items(inventory: &Inventory) -> Vec<ShopItem> {
    let mut items = gen_all_items();
    
    for (i, item) in items.iter_mut().enumerate() {
        item.set_index(i as u32);
    }
    
    for item in &mut items {
        if inventory.contains_item(&(*item.item)) {
            item.owned = true;
            
            if item.item.is_equipped(inventory) {
                item.equipped = true;
            }
        }
    }
    
    items
}

/// Generates a vector containing all the `ShopItem` instances that should be rendered in the `ShopPage`.
/// The ordering in this vector is **the same** order the items will have when displayed on the screen.
fn gen_all_items() -> Vec<ShopItem> {
    let color_sprite = Rc::new(include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"));
    let background_sprite = Rc::new(include_texture!("../../../resources/shop/item_sprites/background_item.png"));
    
    let mut items= Vec::new();
    
    for color in CreatureColor::ALL_VARIANTS {
        items.push(ShopItem::new(
            Box::new(color),
            Rc::clone(&color_sprite),
        ));
    }
    
    for background in GameBackground::ALL_VARIANTS {
        items.push(ShopItem::new(
            Box::new(background),
            Rc::clone(&background_sprite),
        ));
    }
    
    items
}