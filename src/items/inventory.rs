use serde::{Serialize, Deserialize};
use crate::items::creature_color::CreatureColor;
use crate::items::{BuyableItem, ItemType};


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub coins: u32,
    pub creature_colors: Vec<CreatureColor>,
}

impl Inventory {
    /// Attempts to buy a `BuyableItem` instance, accounting for its price and if it is already
    /// owned or not.
    /// 
    /// ## Parameters:
    /// * `item` - The item that should be bought.
    /// 
    /// ## Returns:
    /// The function returns an Err value when the given `Inventory` already contains this item,
    /// or when not enough coins are present.
    pub fn try_buy_item(&mut self, item: &Box<dyn BuyableItem>) -> Result<(), String> {
        if self.contains_item(item) {
            return Err("Already owned".into());
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
    
    #[test]
    fn contains_item() {
        let green = CreatureColor::Green;
        let red = CreatureColor::Red;
        let green_boxed: Box<dyn BuyableItem> = Box::new(green);
        let red_boxed: Box<dyn BuyableItem> = Box::new(red);
        
        let mut inventory = Inventory::default();
        
        
        inventory.creature_colors.push(green);
        
        assert!(inventory.contains_item(&green_boxed));
        assert!(!inventory.contains_item(&red_boxed));
    }
    
    #[test]
    fn try_buy_owned_item() {
        let test_item: Box<dyn BuyableItem> = Box::new(CreatureColor::Green);
        let mut inventory = Inventory::default();
        inventory.coins = 100;
        
        inventory.try_buy_item(&test_item).expect("Failed to buy initial item");
        
        // Attempting to buy it again should fail:
        inventory.try_buy_item(&test_item).unwrap_err();
        
        assert_eq!(100 - test_item.price(), inventory.coins);
        assert_eq!(1, inventory.creature_colors.len());
    }
}