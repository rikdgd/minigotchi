use macroquad::prelude::*;

use crate::utils::time::get_now_millis;
use crate::utils::Location;
use crate::items::inventory::Inventory;
use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, include_texture};
use crate::items::creature_color::CreatureColor;
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
    error_message: Option<String>,
    last_error_millis: i64,
    start_render_millis: i64,
}

impl<'a> ShopPage<'a> {
    pub fn new(inventory: &'a mut Inventory) -> Self {
        Self {
            items: get_all_shop_items(inventory),
            inventory,
            error_message: None,
            last_error_millis: 0,
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
            
            self.render_error_message();

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            self.update();
            next_frame().await;
        }
    }
    
    /// Renders an error message when one occurred in the last 2 seconds. When 2 seconds have passed,
    /// this method automatically sets `self.error_message` to `None`.
    fn render_error_message(&mut self) {
        if get_now_millis() - self.last_error_millis > 2000 {
            self.error_message = None;
            return;
        }
        
        if let Some(msg) = &self.error_message {
            let dimensions = measure_text(msg, None, 18, 1.0);
            let text_location = Location {
                x: (SCREEN_WIDTH / 2) as f32 - dimensions.width / 2.0,
                y: (SCREEN_HEIGHT / 2) as f32 - dimensions.height / 2.0,
            };
            draw_rectangle(
                text_location.x - 10.0,
                text_location.y - dimensions.height - 8.0,
                dimensions.width + 20.0,
                dimensions.height + 20.0,
                Color { r: 0.75, g: 0.7, b: 0.7, a: 1.0},
            );
            draw_text(
                msg,
                text_location.x,
                text_location.y,
                18.0,
                RED,
            );
        }
    }
    
    fn update(&mut self) {
        let now = get_now_millis();
        if now - self.start_render_millis < 500 {
            return;
        }
        
        for item in &self.items {
            if item.is_clicked() {
                if let Err(msg) = self.inventory.try_buy_item(&item.item) {
                    println!("Error when buying item from shop: '{msg}'");
                    self.error_message = Some(msg);
                    self.last_error_millis = now;
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

fn get_all_shop_items(inv: &Inventory) -> Vec<ShopItem> {
    let mut items = vec![
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
    ];
    
    for item in &mut items {
        if inv.contains_item(&item.item) {
            item.owned = true;
        }
    }
    
    items
}