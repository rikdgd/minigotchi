use crate::ui::interaction_menu::{CreatureInteraction, ITEM_CONTAINER_AREA, InteractionMenuItem};
use crate::food::Food;
use crate::creature_game::CreatureGame;


const BASE_ITEM_HEIGHT: f32 = ITEM_CONTAINER_AREA.y + 10.0;

/// Generates and returns all the **InteractionMenuItem** that should be displayed in the
/// **InteractionMenu** when feeding the creature. This function makes sure all items have the
/// correct location on the screen, and that the correct food items are present and in the right
/// order.
pub fn gen_all_food_items() -> [InteractionMenuItem<Food>; 3] {
    set_item_heights([
        InteractionMenuItem::new(Food::Soup),
        InteractionMenuItem::new(Food::Cookie),
        InteractionMenuItem::new(Food::Burger),
    ])
}

/// Generates and returns all the **InteractionMenuItem** that should be displayed in the
/// **InteractionMenu** when playing with the creature. This function makes sure all items have the
/// correct location on the screen, and that the correct items are present and in the right
/// order.
pub fn gen_all_game_items() -> [InteractionMenuItem<CreatureGame>; 3] {
    set_item_heights([
        InteractionMenuItem::new(CreatureGame::Drawing),
        InteractionMenuItem::new(CreatureGame::Basketball),
        InteractionMenuItem::new(CreatureGame::Frisbee),
    ])
}

/// Takes ownership of an array containing `InteractionMenuItem`s and automatically sets their
/// drawing height based on their order in the array. The array with the modified
/// `InteractionMenuItem`s is then returned again.
fn set_item_heights<T: CreatureInteraction>(mut items: [InteractionMenuItem<T>; 3]) -> [InteractionMenuItem<T>; 3] {
    for (i, item) in items.iter_mut().enumerate() {
        item.area.y = i as f32 * (InteractionMenuItem::<T>::DIMENSIONS.height + 5.0) + BASE_ITEM_HEIGHT;
    }
    
    items
}
