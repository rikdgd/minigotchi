use macroquad::prelude::*;

const FOOD_COUNT: u8 = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Food {
    Soup,
    Cookie,
    Burger,
}

impl Food {
    pub fn points(&self) -> u8 {
        match self {
            Food::Soup => 20,
            Food::Cookie => 30,
            Food::Burger => 40,
        }
    }

    pub fn new_random() -> Self {
        match rand::gen_range(0, FOOD_COUNT) {
            0 => Food::Soup,
            1 => Food::Cookie,
            _ => Food::Burger,
        }
    }
}
