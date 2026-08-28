use serde::{Deserialize, Serialize};

/// A managed `u8` value that always remains in range `0..=100`
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat(u8);
impl Stat {
    pub const MAX_VALUE: u8 = 100;
    
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn new(value: u8) -> Result<Self, std::io::Error> {
        if value <= Self::MAX_VALUE {
            return Ok(Stat(value));
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("value not between 0 and {}", Self::MAX_VALUE),
        ))
    }

    /// Adds the provided value, never exceeding 100.
    pub fn add(&mut self, value: u8) {
        let new_value = self.0 + value;

        if new_value <= Self::MAX_VALUE {
            self.0 = new_value;
        } else {
            self.0 = Self::MAX_VALUE;
        }
    }

    /// subtracts the provided value, never wrapping around.
    pub fn subtract(&mut self, value: u8) {
        if self.0 >= value {
            self.0 -= value;
        } else {
            self.0 = 0;
        }
    }

    /// Sets the Stat to the provided value <br>
    /// returns [std::io::ErrorKind::InvalidInput] when value does not fit range: <br>
    /// _0 <= value <= 100_
    #[allow(unused)]
    pub fn set(&mut self, value: u8) -> Result<(), std::io::Error> {
        if value <= Self::MAX_VALUE {
            self.0 = value;
            return Ok(());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("value not between 0 and {}", Self::MAX_VALUE),
        ))
    }
}
