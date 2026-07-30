use macroquad::prelude::*;

use crate::items::inventory::Inventory;
use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::ui::shop::ShopItem;


/// The ShopPage struct is used to render the in-game shop menu. It manages the state of the ui
/// and uses a mutable reference the GameState's inventory to process purchases.
///
/// ## Fields:
/// * `inventory` - A mutable reference the GameState's *inventory* field.
/// * `items` - A vector containing all items that are for sale in the shop.
pub struct ShopPage<'a> {
    inventory: &'a mut Inventory,
    items: Vec<ShopItem>,
}

impl<'a> ShopPage<'a> {
    pub fn new(inventory: &'a mut Inventory) -> Self {
        Self {
            inventory,
            items: vec![],
        }
    }
    
    pub async fn render(&mut self) {
        loop {
            clear_background(crate::BACKGROUND_COLOR);

            for item in &self.items {
                item.draw();
            }

            // TODO: Render toggle button to return to main screen

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            self.update();
            next_frame().await;
        }
    }
    
    fn update(&mut self) {
        for item in &self.items {
            if item.is_clicked() {
                if let Err(msg) = item.item.try_buy(self.inventory) {
                    // TODO: Notify the user that he doesn't have enough coins for this purchase
                    println!("Error when buying item from shop: {msg}");
                }
            }
        }
    }
    
    /// Returns the `Button` component used to toggle the shop page.
    pub fn shop_button() -> Button {
        const BTN_DIMENSION: f32 = 10.0;
        const X_POS: f32 = SCREEN_WIDTH as f32 - BTN_DIMENSION * 2.0;
        const Y_POS: f32 = (SCREEN_HEIGHT / 2) as f32 - 6.0;
        
        Button {
            pos: (X_POS, Y_POS).into(),
            size: (BTN_DIMENSION, BTN_DIMENSION).into(),
            text: "$".to_string(),
            fontsize: 16.0,
            ..Default::default()
        }
    }
}
