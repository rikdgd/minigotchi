use macroquad::prelude::*;
use crate::ui::button::Button;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::utils::Location;


#[derive(Debug, Clone)]
pub struct ShopPage {
    items: Vec<ShopItem>,
}

impl ShopPage {
    pub fn new() -> Self {
        Self {
            items: vec![],
        }
    }
    
    pub async fn render(&mut self) {
        clear_background(crate::BACKGROUND_COLOR);
        
        for item in &self.items {
            item.render();
        }
        
        next_frame().await;
        // self.update();
    }
    
    fn update(&mut self) {
        todo!()
    }
    
    /// Returns the `Button` component used to toggle the shop page.
    pub fn shop_button() -> Button {
        const BTN_DIMENSION: f32 = 10.0;
        let x_pos = SCREEN_WIDTH as f32 - BTN_DIMENSION * 2.0;
        let y_pos = (SCREEN_HEIGHT / 2) as f32 - 6.0;
        
        Button {
            pos: (x_pos, y_pos).into(),
            size: (BTN_DIMENSION, BTN_DIMENSION).into(),
            text: "$".to_string(),
            fontsize: 16.0,
            ..Default::default()
        }
    }
}


/// The `ShopItem` structure can be used to render the shop menu in the minigothci game.
/// It manages the state of the ui and is able to render itself to the screen.
/// 
/// ## Fields:
/// * `name` - The name of the item that appears in the shop.
/// * `sprite` - The item's sprite that should be rendered in the shop
/// * `price` - The amount of coins it costs to buy the item.
/// * `y_pos` - The Y location on the screen where this ShopItem should be drawn.
#[derive(Debug, Clone)]
pub struct ShopItem {
    name: String,
    sprite: Texture2D,
    price: u32,
    area: Rect,
}

impl ShopItem {
    pub fn new(name: &str, sprite: Texture2D, price: u32, y_pos: f32) -> Self {
        Self {
            name: name.to_string(),
            sprite,
            price,
            area: Rect::new(
                10.0,
                y_pos,
                SCREEN_WIDTH as f32 * 0.9,
                20.0,
            ),
        }
    }
    
    pub fn render(&self) {
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
            &self.name,
            self.area.x + 10.0,
            self.area.y + 2.0,
            16.0,
            BLACK,
        );
        
        draw_text(
            &format!("price: {}", self.price),
            self.area.x + self.area.w - 10.0,
            self.area.y + 10.0,
            14.0,
            Color { r: 0.85, g: 0.85, b: 0.85, a: 1.0 },
        );
    }
}
