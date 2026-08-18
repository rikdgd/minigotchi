use macroquad::prelude::*;

use crate::utils::time::get_now_millis;
use crate::utils::Location;
use crate::items::inventory::Inventory;
use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, include_texture};
use crate::items::creature_color::CreatureColor;
use crate::items::game_background::GameBackground;
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
/// * `return_button` - This is the button component used to exit the Shop page.
pub struct ShopPage<'a> {
    inventory: &'a mut Inventory,
    items: Vec<ShopItem>,
    error_message: Option<String>,
    last_error_millis: i64,
    start_render_millis: i64,
    return_button: Button,
    current_scroll: f32,
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
            current_scroll: 0.0,
        }
    }
    
    pub async fn render(&mut self) {
        self.start_render_millis = get_now_millis();
        loop {
            clear_background(crate::BACKGROUND_COLOR);

            for item in &self.items {
                item.draw();
            }

            self.render_return_button();
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
        
        // Draw a background to keep visible while scrolling
        draw_rectangle(
            0.0,
            0.0,
            SCREEN_WIDTH as f32,
            txt_size.height + 14.0,
            crate::BACKGROUND_COLOR,
        );
        
        draw_text(
            &text,
            (SCREEN_WIDTH as f32 - txt_size.width) / 2.0,
            txt_size.height + 7.0,
            FONT_SIZE as f32,
            Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
        );
    }
    
    fn render_return_button(&self) {
        const RECT_HEIGHT: f32 = 25.0;
        
        draw_rectangle(
            0.0, 
            SCREEN_HEIGHT as f32 - RECT_HEIGHT,
            SCREEN_HEIGHT as f32,
            RECT_HEIGHT,
            crate::BACKGROUND_COLOR,
        );
        
        self.return_button.render();
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
        if now - self.start_render_millis < 100 {
            return;
        }
        
        for item in &mut self.items {
            if item.is_clicked() {
                match item.owned {
                    true => {
                        if let Err(msg) = self.inventory.try_equip_item(&(*item.item)) {
                            println!("Failed to equip item: {msg}");
                            self.error_message = Some(msg);
                            self.last_error_millis = now;
                        }
                    },
                    false => {
                        if let Err(msg) = self.inventory.try_buy_item(&(*item.item)) {
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
        
        self.update_scrolling();
    }
    
    fn update_scrolling(&mut self) {
        let (_wheel_x, wheel_y) = mouse_wheel();
        
        if wheel_y < 0.0 {
            self.scroll_shop_items(ScrollDirection::Down);
        }
        
        if wheel_y > 0.0 {
            self.scroll_shop_items(ScrollDirection::Up);
        }
    }
    
    fn scroll_shop_items(&mut self, direction: ScrollDirection) {
        let y_move = match direction {
            ScrollDirection::Down => -7.0,
            ScrollDirection::Up => 7.0,
        };
        
        if self.current_scroll + y_move > 0.0 {
            return;
        }
        
        self.current_scroll += y_move;
        for item in &mut self.items {
            item.move_y(y_move);
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
            Box::new(CreatureColor::Black),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            0,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Red),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            1,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Green),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            2,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Blue),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            3,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Pink),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            4,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Yellow),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            5,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Cyan),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            6,
        ),
        ShopItem::new(
            Box::new(CreatureColor::Rainbow),
            include_texture!("../../../resources/shop/item_sprites/creature_color_item.png"),
            7
        ),
        ShopItem::new(
            Box::new(GameBackground::Plain),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
            8,
        ),
        ShopItem::new(
            Box::new(GameBackground::Fields),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
            9,
        ),
        ShopItem::new(
            Box::new(GameBackground::Shrooms),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
            10,
        ),
        ShopItem::new(
            Box::new(GameBackground::Cave),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
            11,
        ),
        ShopItem::new(
            Box::new(GameBackground::Ocean),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
            12,
        ),
        ShopItem::new(
            Box::new(GameBackground::Space),
            include_texture!("../../../resources/shop/item_sprites/background_item.png"),
            13,
        ),
    ];
    
    for item in &mut items {
        if inv.contains_item(&(*item.item)) {
            item.owned = true;
            
            if item.item.is_equipped(inv) {
                item.equipped = true;
            }
        }
    }
    
    items
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScrollDirection {
    Up,
    Down,
}