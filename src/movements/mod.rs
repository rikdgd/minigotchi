mod egg_hop;
mod zig_zag;

pub use egg_hop::EggHop;

use crate::utils::Location;

/// Represents a movement the creature can make on the screen, for example as an idle animation.
pub trait CreatureMovement {
    /// Returns the next `Location` of the animation. If the animation has ended, it should return `None`.
    fn next_position(&mut self) -> Option<Location>;
}
