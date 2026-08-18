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
    let mut items = get_all_items();
    
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
fn get_all_items() -> Vec<ShopItem> {
    vec![
        ShopItem::new(
            Box::new(CreatureColor::Black),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Red),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Green),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Blue),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Pink),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Yellow),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Cyan),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(CreatureColor::Rainbow),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
        ),
        ShopItem::new(
            Box::new(GameBackground::Plain),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
        ),
        ShopItem::new(
            Box::new(GameBackground::Fields),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
        ),
        ShopItem::new(
            Box::new(GameBackground::Shrooms),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
        ),
        ShopItem::new(
            Box::new(GameBackground::Cave),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
        ),
        ShopItem::new(
            Box::new(GameBackground::Ocean),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
        ),
        ShopItem::new(
            Box::new(GameBackground::Space),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
        ),
    ]
}