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
/// * `error_message` - The error message that was received from a failed transaction. Gets set back
///   to `None` after 2 seconds have passed.
/// * `last_error_millis` - The last system time in milliseconds on which an error has occurred during
///   a transaction. Used determine when an error message should be rendered.
/// * `start_render_millis` - The system time in milliseconds when the shop page started rendering.
///   used to prevent *(accidental)* purchases in the first **0.5 seconds**.
pub struct ShopPage<'a> {
    inventory: &'a mut Inventory,
    items: Vec<ShopItem>,
    error_message: Option<String>,
    last_error_millis: i64,
    start_render_millis: i64,
    return_button: Button,
}

impl<'a> ShopPage<'a> {
    pub fn new(inventory: &'a mut Inventory) -> Self {
        Self {
            items: get_all_shop_items(inventory),
            inventory,
            error_message: None,
            last_error_millis: 0,
            start_render_millis: 0,
            return_button: Self::return_button(),
        }
    }
    
    pub async fn render(&mut self) {
        self.start_render_millis = get_now_millis();
        loop {
            clear_background(crate::BACKGROUND_COLOR);

            for item in &self.items {
                item.draw();
            }

            self.return_button.render();
            self.render_coin_count();
            self.render_error_message();

            if is_key_pressed(KeyCode::Escape) || self.return_button.is_clicked() {
                break;
            }

            self.update();
            next_frame().await;
        }
    }
    
    fn render_coin_count(&self) {
        const FONT_SIZE: u16 = 14;
        let text = format!("Coins: {}", self.inventory.coins);
        let txt_size = measure_text(&text, None, FONT_SIZE, 1.0);
        
        draw_text(
            &text,
            (SCREEN_WIDTH as f32 - txt_size.width) / 2.0,
            txt_size.height + 7.0,
            FONT_SIZE as f32,
            Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
        );
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
        
        for item in &mut self.items {
            if item.is_clicked() {
                match item.owned {
                    true => {
                        if let Err(msg) = self.inventory.try_equip_item(&item.item) {
                            println!("Failed to equip item: {msg}");
                            self.error_message = Some(msg);
                            self.last_error_millis = now;
                        }
                    },
                    false => {
                        if let Err(msg) = self.inventory.try_buy_item(&item.item) {
                            println!("Error when buying item from shop: '{msg}'");
                            self.error_message = Some(msg);
                            self.last_error_millis = now;
                        } else {
                            item.owned = true;
                        }
                    }
                }
            }
            
            item.equipped = item.item.is_equipped(self.inventory);
        }
    }
    
    /// Returns the `Button` component used to enter the shop page. Note that this Button component
    /// **does not** actually do anything on its own and is merely useful for its pre-set styling.
    pub fn shop_button() -> Button {
        Button {
            pos: Vec2::new(
                SCREEN_WIDTH as f32 - 35.0,
                (SCREEN_HEIGHT / 2) as f32 - 6.0
            ),
            size: Vec2::new(25.0, 12.0),
            text: "Shop".to_string(),
            fontsize: 11.0,
            ..Default::default()
        }
    }
    
    fn return_button() -> Button {
        Button {
            text: "return".to_string(),
            pos: Vec2::new(
                (SCREEN_WIDTH as f32 - 40.0) / 2.0,
                SCREEN_HEIGHT as f32 - 20.0,
            ),
            size: Vec2::new(40.0, 15.0),
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
            
            if item.item.is_equipped(inv) {
                item.equipped = true;
            }
        }
    }
    
    items
}