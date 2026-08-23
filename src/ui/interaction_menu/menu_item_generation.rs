use crate::ui::interaction_menu::{CreatureInteraction, ITEM_CONTAINER_AREA, InteractionMenuItem};
use crate::food::Food;
use crate::creature_game::CreatureGame;


const BASE_ITEM_HIGHT: f32 = ITEM_CONTAINER_AREA.y + 10.0;

/// Generates and returns all the **InteractionMenuItem** that should be displayed in the
/// **InteractionMenu** when feeding the creature. This function makes sure all items have the
/// correct location on the screen, and that the correct food items are present and in the right
/// order.
pub fn gen_all_food_items() -> [InteractionMenuItem<Food>; 3] {
    let mut items = [
        InteractionMenuItem::new(Food::Soup),
        InteractionMenuItem::new(Food::Cookie),
        InteractionMenuItem::new(Food::Burger),
    ];

    for (i, item) in items.iter_mut().enumerate() {
        item.area.y = i as f32 * (InteractionMenuItem::<Food>::DIMENSIONS.height + 5.0) + BASE_ITEM_HIGHT;
    }

    items
}

pub fn gen_all_game_items() -> [InteractionMenuItem<CreatureGame>; 3] {
    set_item_heights([
        InteractionMenuItem::new(CreatureGame::Drawing),
        InteractionMenuItem::new(CreatureGame::Basketball),
        InteractionMenuItem::new(CreatureGame::Frisbee),
    ])
}

fn set_item_heights<T: CreatureInteraction>(mut items: [InteractionMenuItem<T>; 3]) -> [InteractionMenuItem<T>; 3] {
    for (i, item) in items.iter_mut().enumerate() {
        item.area.y = i as f32 * (InteractionMenuItem::<Food>::DIMENSIONS.height + 5.0) + BASE_ITEM_HIGHT;
    }
    
    items
}