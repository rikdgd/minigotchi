pub mod creature_color;
pub mod inventory;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Background,
    CreatureColor,
    Ball,
}


pub trait BuyableItem {
    fn price(&self) -> u32;
    fn item_type(&self) -> ItemType;
}
