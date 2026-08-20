use macroquad::prelude::*;
use crate::food::Food;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ui::button::Button;
use crate::utils::Dimensions;

#[derive(Debug, Clone)]
pub struct FoodMenu {
    selected_food: Option<Food>,
    running: bool,
    menu_items: [FoodMenuItem; 3],
    return_btn: Button,
}
impl FoodMenu {
    const BACKDROP_DIMENSIONS: Dimensions = Dimensions { width: 150.0, height: 120.0 };
    
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
        while self.running {
            clear_background(crate::BACKGROUND_COLOR);
            
            draw_rectangle(
                (SCREEN_WIDTH as f32 - Self::BACKDROP_DIMENSIONS.width) / 2.0,
                (SCREEN_HEIGHT as f32 - Self::BACKDROP_DIMENSIONS.height) / 2.0,
                Self::BACKDROP_DIMENSIONS.width,
                Self::BACKDROP_DIMENSIONS.height,
                Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 },
            );
            
            for item in self.menu_items {
                item.render();
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
    const NAME_FONT_SIZE: f32 = 16.0;
    const DIMENSIONS: Dimensions = Dimensions { width: 100.0, height: 30.0 };
    
    pub fn new(food: Food, draw_height: f32) -> Self {
        Self {
            food,
            area: Rect::new(
                10.0,
                draw_height,
                Self::DIMENSIONS.width,
                Self::DIMENSIONS.height,
            ),
        }
    }
    
    pub fn render(&self) {
        let name_dimensions = measure_text(
            self.food.name(),
            None,
            Self::NAME_FONT_SIZE as u16,
            1.0,
        );
        
        todo!()
    }
    
    pub fn is_clicked(&self) -> bool {
        todo!()
    }
}