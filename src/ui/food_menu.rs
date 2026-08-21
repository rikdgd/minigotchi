use macroquad::prelude::*;
use crate::food::Food;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ui::button::Button;
use crate::utils::Dimensions;

const ITEM_CONTAINER_AREA: Rect = Rect { 
    x: (SCREEN_WIDTH as f32 - 150.0) / 2.0,
    y: (SCREEN_HEIGHT as f32 - 120.0) / 2.0,
    w: 150.0,
    h: 120.0,
};

#[derive(Debug, Clone)]
pub struct FoodMenu {
    selected_food: Option<Food>,
    running: bool,
    menu_items: [FoodMenuItem; 3],
    return_btn: Button,
}
impl FoodMenu {
    const MESSAGE: &str = "Select food";
    
    pub fn new() -> Self {
        Self {
            selected_food: None,
            running: true,
            menu_items: [
                FoodMenuItem::new(Food::Soup, 10.0),
                FoodMenuItem::new(Food::Cookie, 20.0),
                FoodMenuItem::new(Food::Burger, 30.0),
            ],
            return_btn: Button {
                text: "return".to_string(),
                pos: Vec2::new(
                    (SCREEN_WIDTH as f32 - 50.0) / 2.0,
                    SCREEN_HEIGHT as f32 - 30.0,
                ),
                ..Default::default()
            },
        }
    }
    
    /// Renders the food selection menu and returns the food selected by the user for feeding.
    /// The selected food can also be `None` when the user exits the food menu via the *'return
    /// button'*.
    pub async fn render(&mut self) -> Option<Food> {
        let msg_dimensions = measure_text(Self::MESSAGE, None, 20, 1.0);
        
        while self.running {
            clear_background(crate::BACKGROUND_COLOR);
            
            draw_text(
                Self::MESSAGE,
                (SCREEN_WIDTH as f32 - msg_dimensions.width) / 2.0,
                msg_dimensions.height + 15.0,
                20.0,
                BLACK,
            );
            
            // Draw the item container
            draw_rectangle(
                ITEM_CONTAINER_AREA.x,
                ITEM_CONTAINER_AREA.y,
                ITEM_CONTAINER_AREA.w,
                ITEM_CONTAINER_AREA.h,
                Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 },
            );
            
            for item in self.menu_items {
                item.draw();
            }
            
            self.return_btn.render();
            self.update();
            
            next_frame().await;
        }
        
        self.selected_food
    }
    
    fn update(&mut self) {
        for item in self.menu_items {
            if item.is_clicked() {
                self.selected_food = Some(item.food);
            }
        }
        
        if self.selected_food.is_some() || self.return_btn.is_clicked() {
            self.running = false;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FoodMenuItem {
    pub food: Food,
    area: Rect,
}

impl FoodMenuItem {
    const DIMENSIONS: Dimensions = Dimensions { width: 130.0, height: 50.0 };
    
    pub fn new(food: Food, draw_height: f32) -> Self {
        Self {
            food,
            area: Rect::new(
                (ITEM_CONTAINER_AREA.w - Self::DIMENSIONS.width) / 2.0 + ITEM_CONTAINER_AREA.x,
                draw_height,
                Self::DIMENSIONS.width,
                Self::DIMENSIONS.height,
            ),
        }
    }
    
    pub fn draw(&self) {
        let name_dimensions = measure_text(
            self.food.name(),
            None,
            16,
            1.0,
        );
        
        draw_rectangle(
            self.area.x,
            self.area.y,
            self.area.w,
            self.area.h,
            Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 },
        );
    }
    
    pub fn is_clicked(&self) -> bool {
        // todo!()
        false
    }
}