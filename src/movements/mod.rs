mod egg_hop;
mod zig_zag;

pub use egg_hop::EggHop;
pub use zig_zag::ZigZag;
use crate::friend::{Friend, GrowthStage};
use crate::utils::Location;

/// Represents a movement the creature can make on the screen, for example as an idle animation.
pub trait CreatureMovement {
    /// Returns the next `Location` of the animation. If the animation has ended, it should return `None`.
    fn next_position(&mut self) -> Option<Location>;
}

/// Returns the movement/animation the creature should have on screen when idle, depending on its growth stage.
/// 
/// # Parameters:
/// * `creature` - The creature from the save file that is being played.
/// * `base_location` - The place where the movement starts, most movements stay around this location.
/// 
/// # Returns:
/// Returns a `Box<dyn CreatureMovement>` so it has a known size and can be stored in a mutable variable.
pub fn get_creature_movement(creature: &Friend, base_location: Location) -> Box<dyn CreatureMovement> {
    match creature.growth_stage() {
        GrowthStage::Egg => Box::new(EggHop::new(base_location)),
        _ => Box::new(ZigZag::default().base_location(base_location)),
    }
}
