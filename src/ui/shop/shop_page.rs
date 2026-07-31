use macroquad::prelude::*;

use crate::utils::time::get_now_millis;
use crate::items::inventory::Inventory;
use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::ui::shop::ShopItem;
use crate::ui::shop::shop_item::get_unowned_shop_items;


/// The ShopPage struct is used to render the in-game shop menu. It manages the state of the ui
/// and uses a mutable reference the GameState's inventory to process purchases.
///
/// ## Fields:
/// * `inventory` - A mutable reference the GameState's *inventory* field.
/// * `items` - A vector containing all items that are for sale in the shop.
pub struct ShopPage<'a> {
    inventory: &'a mut Inventory,
    items: Vec<ShopItem>,
    start_render_millis: i64,
}

impl<'a> ShopPage<'a> {
    pub fn new(inventory: &'a mut Inventory) -> Self {
        Self {
            items: get_unowned_shop_items(&(*inventory)),
            inventory,
            start_render_millis: 0,
        }
    }
    
    pub async fn render(&mut self) {
        self.start_render_millis = get_now_millis();
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
        if get_now_millis() - self.start_render_millis < 500 {
            return;
        }
        
        for item in &self.items {
            if item.is_clicked() {
                if let Err(msg) = self.inventory.try_buy_item(&item.item) {
                    // TODO: Notify the user about the occurred error
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
