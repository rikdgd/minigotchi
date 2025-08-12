use crate::movements::CreatureMovement;
use crate::utils::Location;

pub struct ZigZag {
    base_location: Location,
    timer: f32,
}

impl CreatureMovement for ZigZag {
    fn next_position(&mut self) -> Option<Location> {
        todo!()
    }
}