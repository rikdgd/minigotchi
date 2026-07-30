use serde::{Serialize, Deserialize};
use crate::items::creature_color::CreatureColor;
use crate::items::{BuyableItem, ItemType};


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub coins: u32,
    pub creature_colors: Vec<CreatureColor>,
}

impl Inventory {
    pub fn try_buy_item(&mut self, item: &Box<dyn BuyableItem>) -> Result<(), String> {
        if self.contains_item(item) {
            return Err("Cannot buy an item twice".into());
        }
        
        match item.try_buy(self) {
            Ok(_) => Ok(()),
            Err(msg) => Err(String::from(msg)),
        }
    }
    
    pub fn contains_item(&self, item: &Box<dyn BuyableItem>) -> bool {
        match item.item_type() {
            ItemType::CreatureColor => {
                for color in &self.creature_colors {
                    if color.name() == item.name() { return true }
                }
                false
            }
            _ => false
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::items::inventory::Inventory;
    use crate::items::{BuyableItem, creature_color::CreatureColor};
    
    
    #[test]
    fn try_buy_affordable_item() {
        let test_item: Box<dyn BuyableItem> = Box::new(CreatureColor::Red);
        let mut inventory = Inventory::default();
        inventory.coins = test_item.price() * 2;
        
        inventory.try_buy_item(&test_item).expect("Failed to buy item, even with enough coins.");
        assert_eq!(inventory.coins, test_item.price());
        assert_eq!(inventory.creature_colors[0].name(), test_item.name());
        assert_eq!(inventory.creature_colors.len(), 1);
    }
    
    #[test]
    fn try_buy_not_affordable_item() {
        let test_item: Box<dyn BuyableItem> = Box::new(CreatureColor::Red);
        let mut inventory = Inventory::default();
        inventory.coins = test_item.price() - 1;
        
        // The 'try_buy_item' method should return an Err value:
        inventory.try_buy_item(&test_item).unwrap_err();
    }
}