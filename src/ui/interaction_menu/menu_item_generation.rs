use crate::ui::interaction_menu::{
    CreatureInteraction,
    InteractionMenuItem,
    ITEM_CONTAINER_AREA, 
};


const BASE_ITEM_HEIGHT: f32 = ITEM_CONTAINER_AREA.y + 10.0;

/// Generates and returns all the **InteractionMenuItem**s that should be displayed in the
/// **InteractionMenu**. This function makes sure all items have the correct location on the screen,
/// and that the correct items are present.
pub fn gen_interaction_items<T: CreatureInteraction>() -> [InteractionMenuItem<T>; 3] {
    let variants = T::all_variants();

    set_item_heights([
        InteractionMenuItem::new(variants[0]),
        InteractionMenuItem::new(variants[1]),
        InteractionMenuItem::new(variants[2]),
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
