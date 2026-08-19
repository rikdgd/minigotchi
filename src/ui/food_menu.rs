use macroquad::prelude::*;
use crate::food::Food;
use crate::ui::button::Button;

#[derive(Debug, Clone)]
pub struct FoodMenu {
    selected_food: Option<Food>,
    running: bool,
    menu_items: [FoodMenuItem; 3],
    return_btn: Button,
}
impl FoodMenu {
    pub fn new() -> Self {
        Self {
            selected_food: None,
            running: true,
            menu_items: [
                FoodMenuItem { food: Food::Soup },
                FoodMenuItem { food: Food::Cookie },
                FoodMenuItem { food: Food::Burger },
            ],
            return_btn: Button {
                text: "return".to_string(),
                ..Default::default()
            },
        }
    }
    
    /// Renders the food selection menu and returns the food selected by the user for feeding.
    /// The selected food can also be `None` when the user exits the food menu via the *'return
    /// button'*.
    pub async fn render(&mut self) -> Option<Food> {
        while self.running {
            // TODO: Render a background

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
}

impl FoodMenuItem {
    pub fn render(&self) {
        todo!()
    }
    
    pub fn is_clicked(&self) -> bool {
        todo!()
    }
}