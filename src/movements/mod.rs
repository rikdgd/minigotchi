mod egg_hop;
mod zig_zag;
mod dvd_bounce;
mod cursor_stalk;

pub use egg_hop::EggHop;
pub use zig_zag::ZigZag;
pub use dvd_bounce::DvdBounce;
pub use cursor_stalk::CursorStalk;
use crate::creature::{Creature, GrowthStage};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::utils::Location;

/// Represents a movement the creature can make on the screen, for example as an idle animation.
pub trait CreatureMovement {
    /// Returns the current `Location` and **does not** advance the movements state.
    fn current_position(&self) -> Location;
    /// Returns the next `Location` of the movement and updates it's state.
    fn next_position(&mut self) -> Location;
    /// Returns `true` when the creature's sprite should be horizontally mirrored in the current
    /// state of the movement.
    fn mirror_sprite(&self) -> bool;
}

/// Returns the movement/animation the creature should have on screen when idle, depending on its growth stage.
/// 
/// # Parameters:
/// * `creature` - The creature from the save file that is being played.
/// * `base_location` - The place where the movement starts, most movements stay around this location.
/// 
/// # Returns:
/// Returns a `Box<dyn CreatureMovement>` so it has a known size and can be stored in a mutable variable.
pub fn get_creature_movement(creature: &Creature, base_location: Location) -> Box<dyn CreatureMovement> {
    match creature.growth_stage() {
        GrowthStage::Egg => Box::new(EggHop::new(base_location)),
        GrowthStage::Adult => Box::new(DvdBounce::new(creature).start_location(base_location)),
        _ => Box::new(ZigZag::default().base_location(base_location)),
    }
}

/// Returns the `Location` where the given creature should be drawn when it is asleep.
///
/// # Parameters:
/// * `creature` - The creature that should be sleeping, used to get its sprite size.
///
/// # Returns:
/// The `Location` to draw the creature at, accounting for the sprite's width/height.
pub fn get_sleeping_location(creature: &Creature) -> Location {
    let base_location = Location { x: SCREEN_WIDTH as f32 / 2.0, y: SCREEN_HEIGHT as f32 / 4.0 };
    let sprite_dimension: f32 = match creature.growth_stage() {
        GrowthStage::Egg => 10.0,
        GrowthStage::Baby => 15.0,
        GrowthStage::Kid => 20.0,
        _ => 25.0,
    };

    base_location.translate(
        (-sprite_dimension / 2.0).round(),
        (-sprite_dimension / 2.0).round(),
    )
}
