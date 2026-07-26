use crate::items::inventory::Inventory;

pub mod creature_color;
pub mod inventory;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Background,
    CreatureColor,
    Ball,
}


pub trait BuyableItem {
    fn name(&self) -> &str;
    fn price(&self) -> u32;
    fn item_type(&self) -> ItemType;
    fn add_to_inventory(&self, inventory: &mut Inventory);

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
        if !self.can_buy(inventory) {
            return Err("Not enough coins");
        }

        self.add_to_inventory(inventory);
        inventory.coins -= self.price();

        Ok(())
    }

    /// Returns `True` when the inventory has enough coins to buy this `BuyableItem`.
    fn can_buy(&self, inventory: &Inventory) -> bool {
        inventory.coins >= self.price()
    }
}
