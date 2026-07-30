use macroquad::prelude::*;
use crate::items::inventory::Inventory;
use crate::items::BuyableItem;
use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};


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


/// The `ShopItem` structure can be used to render a buyable item in the shop menu.
/// It manages its own state and can be rendered using the `render()` function.
/// 
/// ## Fields:
/// * `item` - The `BuyableItem` instance that is sold via this ShopItem.
/// * `sprite` - The item's sprite that should be rendered in the shop
/// * `area` - The hitbox/size of the item when rendered on screen.
pub struct ShopItem {
    item: Box<dyn BuyableItem>,
    sprite: Texture2D,
    area: Rect,
}

impl ShopItem {
    pub fn new(item: Box<dyn BuyableItem>, sprite: Texture2D, y_pos: f32) -> Self {
        Self {
            item,
            sprite,
            area: Rect::new(
                10.0,
                y_pos,
                SCREEN_WIDTH as f32 * 0.9,
                20.0,
            ),
        }
    }

    pub fn is_clicked(&self) -> bool {
        self.area.contains(mouse_position().into()) && is_mouse_button_pressed(MouseButton::Left)
    }
    
    pub fn draw(&self) {
        draw_rectangle(
            self.area.x,
            self.area.y,
            self.area.w,
            self.area.h,
            Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }
        );
        
        draw_texture_ex(
            &self.sprite,
            self.area.x + 2.0,
            self.area.y + 2.0,
            BLACK,
            DrawTextureParams::default(),
        );
        
        draw_text(
            self.item.name(),
            self.area.x + 10.0,
            self.area.y + 2.0,
            16.0,
            BLACK,
        );
        
        draw_text(
            &format!("price: {}", self.item.price()),
            self.area.x + self.area.w - 10.0,
            self.area.y + 10.0,
            14.0,
            Color { r: 0.85, g: 0.85, b: 0.85, a: 1.0 },
        );
    }
}
