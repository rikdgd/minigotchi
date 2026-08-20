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
    
    pub fn name(&self) -> &str {
        match self {
            Food::Soup => "Soup",
            Food::Cookie => "Cookie",
            Food::Burger => "Burger",
        }
    }
}
