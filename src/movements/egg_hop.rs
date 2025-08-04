use crate::movements::CreatureMovement;
use crate::utils::Location;

pub struct EggHop {
    base_location: Location,
    is_grounded: bool,
    last_update_time: i64,
}

impl EggHop {
    pub fn new(base_location: Location) -> Self {
        Self {
            base_location,
            is_grounded: true,
            last_update_time: 0, // TODO: Actually get millis from now
        }
    }
}

impl CreatureMovement for EggHop {
    fn next_position(&mut self) -> Option<Location> {
        match self.is_grounded {
            true => {
                Some(self.base_location)
            },
            false => {
                Some(self.base_location.translate(0.0, 5.0))
            }
        }
    }
}