use serde::{Serialize, Deserialize};
use crate::items::creature_color::CreatureColor;
use crate::items::{BuyableItem, ItemType};
use crate::items::game_background::GameBackground;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub coins: u32,
    pub creature_colors: Vec<CreatureColor>,
    pub equipped_color: CreatureColor,
    pub backgrounds: Vec<GameBackground>,
    pub equipped_background: GameBackground,
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
    pub fn try_buy_item(&mut self, item: &dyn BuyableItem) -> Result<(), String> {
        if self.contains_item(item) {
            return Err("Already owned".into());
        }
        
        match item.try_buy(self) {
            Ok(_) => Ok(()),
            Err(msg) => Err(String::from(msg)),
        }
    }
    
    pub fn contains_item(&self, item: &dyn BuyableItem) -> bool {
        match item.item_type() {
            ItemType::CreatureColor => {
                for color in &self.creature_colors {
                    if color.name() == item.name() { return true }
                }
                false
            },
            ItemType::Background => {
                for bg in &self.backgrounds {
                    if bg.name() == item.name() { return true }
                }
                false
            }
        }
    }
    
    pub fn try_equip_item(&mut self, item: &dyn BuyableItem) -> Result<(), String> {
        match item.try_equip(self) {
            Ok(_) => Ok(()),
            Err(msg) => Err(msg.to_string()),
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            coins: 0,
            creature_colors: vec![CreatureColor::Black],
            equipped_color: CreatureColor::Black,
            backgrounds: vec![GameBackground::Plain],
            equipped_background: GameBackground::Plain,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::items::inventory::Inventory;
    use crate::items::{BuyableItem, creature_color::CreatureColor};
    
    
    #[test]
    fn try_buy_affordable_item() {
        let test_item = CreatureColor::Red;
        let mut inventory = Inventory::default();
        inventory.creature_colors = vec![];
        inventory.coins = test_item.price() * 2;
        
        inventory.try_buy_item(&test_item).expect("Failed to buy item, even with enough coins.");
        assert_eq!(inventory.coins, test_item.price());
        assert_eq!(inventory.creature_colors[0].name(), test_item.name());
        assert_eq!(inventory.creature_colors.len(), 1);
    }
    
    #[test]
    fn try_buy_not_affordable_item() {
        let test_item = CreatureColor::Red;
        let mut inventory = Inventory::default();
        inventory.coins = test_item.price() - 1;
        
        // The 'try_buy_item' method should return an Err value:
        inventory.try_buy_item(&test_item).unwrap_err();
    }
    
    #[test]
    fn contains_item() {
        let green = CreatureColor::Green;
        let red = CreatureColor::Red;
        
        let mut inventory = Inventory::default();
        
        
        inventory.creature_colors.push(green);
        
        assert!(inventory.contains_item(&green));
        assert!(!inventory.contains_item(&red));
    }
    
    #[test]
    fn try_buy_owned_item() {
        let test_item = CreatureColor::Green;
        let mut inventory = Inventory::default();
        inventory.creature_colors = vec![];
        inventory.coins = 100;
        
        inventory.try_buy_item(&test_item).expect("Failed to buy initial item");
        
        // Attempting to buy it again should fail:
        inventory.try_buy_item(&test_item).unwrap_err();
        
        assert_eq!(100 - test_item.price(), inventory.coins);
        assert_eq!(1, inventory.creature_colors.len());
    }
    
    #[test]
    fn try_equip_item() {
        let test_item = CreatureColor::Blue;
        let mut inventory = Inventory::default();
        inventory.coins = 1000;

        // Item not owned so should return error:
        inventory.try_equip_item(&test_item).unwrap_err();
        
        inventory.creature_colors.push(CreatureColor::Green);
        // Item still not owned, so expect an error:
        inventory.try_equip_item(&test_item).unwrap_err();
        
        inventory.try_buy_item(&test_item).unwrap();
        // Now we own the blue color, so expect an Ok
        inventory.try_equip_item(&test_item).unwrap();
    }
}