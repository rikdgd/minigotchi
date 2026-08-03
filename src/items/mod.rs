use crate::items::inventory::Inventory;

pub mod creature_color;
pub mod inventory;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    CreatureColor,
}


pub trait BuyableItem {
    fn name(&self) -> &str;
    fn price(&self) -> u32;
    fn item_type(&self) -> ItemType;
    fn add_to_inventory(&self, inventory: &mut Inventory);
    fn try_equip(&self, inventory: &mut Inventory) -> Result<(), &str>;
    fn is_equipped(&self, inventory: &Inventory) -> bool;
    

    /// Attempt to buy this item using the given inventory.
    /// 
    /// ## Parameters:
    /// * `inventory` - A mutable reference to the current GameState's inventory. Used to complete
    ///   the transaction.
    ///
    /// ## Returns:
    /// This method returns an error when the given inventory does not contain enough **coins** to
    /// buy this `BuyableItem` instance.
    fn try_buy(&self, inventory: &mut Inventory) -> Result<(), &str> {
        if self.price() > inventory.coins {
            return Err("Not enough coins");
        }

        self.add_to_inventory(inventory);
        inventory.coins -= self.price();

        Ok(())
    }
}
