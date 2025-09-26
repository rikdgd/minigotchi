use macroquad::texture::Texture2D;
use crate::food::Food;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::shapes::{CreatureShapes, egg_shape, baby_shape, kid_shape};
use crate::utils::Stat;

const MINUTE_MILLIS: i64 = 1000 * 60;
const FOOD_OFFSET_MINUTES: i64 = 16 * MINUTE_MILLIS;
const ENERGY_OFFSET_MINUTES: i64 = 3 * MINUTE_MILLIS;
const JOY_OFFSET_MINUTES: i64 = 18 * MINUTE_MILLIS;
const HEALTH_OFFSET_MINUTES: i64 = MINUTE_MILLIS;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrowthStage {
    Egg,
    Baby,
    Kid,
    Adult,
}

impl GrowthStage {
    /// Upgrades self to the next logical growth stage.
    pub fn next_stage(&mut self) {
        match self {
            GrowthStage::Egg => *self = GrowthStage::Baby,
            GrowthStage::Baby => *self = GrowthStage::Kid,
            GrowthStage::Kid => *self = GrowthStage::Adult,

            GrowthStage::Adult => (),
        }
    }
}

/// This is the struct represents the creature/pet of the player. It mainly keeps track of state-update
/// times, its status, its shape, and its growth stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
    name: String,
    food: Stat,
    joy: Stat,
    energy: Stat,
    health: Stat,
    last_time_lower_food: i64,
    last_time_lower_joy: i64,
    last_time_lower_energy: i64,
    last_time_lower_health: i64,
    health_decrease_time_left: i64,
    shape: CreatureShapes,
    growth_stage: GrowthStage,
    asleep: bool,
    asleep_since: Option<i64>,
    alive: bool,
    time_created: i64,
}

impl Creature {
    pub fn new(name: &str, shape: CreatureShapes, now: i64) -> Self {
        Self {
            name: String::from(name),
            food: Stat::new(50).unwrap(),
            joy: Stat::new(50).unwrap(),
            energy: Stat::new(50).unwrap(),
            health: Stat::new(50).unwrap(),
            last_time_lower_food: now,
            last_time_lower_joy: now,
            last_time_lower_energy: now,
            last_time_lower_health: now,
            health_decrease_time_left: 0,
            shape,
            growth_stage: GrowthStage::Egg,
            asleep: false,
            asleep_since: None,
            alive: true,
            time_created: now,
        }
    }

    /// Updates this creature's state for each minute passed since last update.
    pub fn update_state(&mut self, now: i64) {
        self.update_growth_stage(now);

        if self.growth_stage != GrowthStage::Egg {
            self.update_stats(now);
            self.update_alive_status();
        }
    }

    fn update_stats(&mut self, now: i64) {
        // Use while loops instead of if statements to account for loading from file
        // when we might have been away for more than a single minute.
        while now - self.last_time_lower_food >= FOOD_OFFSET_MINUTES {
            self.food.subtract(1);
            self.last_time_lower_food += FOOD_OFFSET_MINUTES;
        }

        while now - self.last_time_lower_energy >= ENERGY_OFFSET_MINUTES {
            if self.asleep {
                self.energy.add(1);
            }

            self.last_time_lower_energy += ENERGY_OFFSET_MINUTES;
        }

        while now - self.last_time_lower_joy >= JOY_OFFSET_MINUTES {
            self.joy.subtract(1);
            self.last_time_lower_joy += JOY_OFFSET_MINUTES;
        }

        while now - self.last_time_lower_health >= HEALTH_OFFSET_MINUTES {
            if self.health_decrease_time_left >= HEALTH_OFFSET_MINUTES {
                self.health.subtract(1);
                self.health_decrease_time_left -= HEALTH_OFFSET_MINUTES;
            }
            self.last_time_lower_health += HEALTH_OFFSET_MINUTES;
        }
    }

    fn update_alive_status(&mut self) {
        // Use u16 conversions to forecome overflows.
        let stats_sum =
            self.food.value() as u16 +
            self.joy.value() as u16 +
            self.health.value() as u16;

        if stats_sum < 15 {
            self.alive = false;
        }

        let mut counter: u8 = 0;
        for stat in [self.food, self.joy, self.energy, self.health] {
            if stat.value() == 0 {
                counter += 1;
            }
        }
        if counter >= 2 {
            self.alive = false;
        }

        if self.health.value() == 0 {
            self.alive = false;
        }
    }

    fn update_growth_stage(&mut self, now: i64) {
        let growth_delay = match self.growth_stage {
            GrowthStage::Egg => Some(MINUTE_MILLIS),
            GrowthStage::Baby => Some(60 * MINUTE_MILLIS),
            GrowthStage::Kid => Some(5 * 60 * MINUTE_MILLIS),
            GrowthStage::Adult => None,
        };

        if let Some(growth_delay) = growth_delay &&
            now - self.time_created > growth_delay {
                self.growth_stage.next_stage();
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Interaction used to increase the creature's `food` stat by feeding it something.
    ///
    /// # Parameters:
    /// * `food` - The food item that should be fed to the creature, the amount of food points the
    /// creature receives, is indicated by the `Food::points()` method.
    pub fn eat(&mut self, food: Food) {
        if self.growth_stage == GrowthStage::Egg {
            return;
        }

        self.food.add(food.points());
        self.health_decrease_time_left += (food.points() / 3) as i64 * MINUTE_MILLIS;
    }

    /// Interaction used to make the creature sleep or wake up, when it sleeps, its `asleep_since` state
    /// will be set to now.
    pub fn toggle_sleep(&mut self) {
        if self.growth_stage == GrowthStage::Egg {
            return;
        }

        self.asleep = !self.asleep;

        if self.asleep {
            let now = Utc::now().timestamp_millis();
            self.asleep_since = Some(now);
        } else {
            self.asleep_since = None;
        }
    }

    /// Interaction used to *"play"* with the creature in order to increase its `joy` stat.
    pub fn play(&mut self) {
        if self.growth_stage == GrowthStage::Egg || self.energy.value() < 5 {
            return;
        }

        self.joy.add(30);
        self.health_decrease_time_left += 10 * MINUTE_MILLIS;

        self.energy.subtract(20);
    }

    /// Interaction used to give the creature some medicine in order to increase its `health` stat.
    pub fn take_medicine(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.health.add(40);
        }
    }
    
    pub fn food(&self) -> Stat {
        self.food
    }

    pub fn joy(&self) -> Stat {
        self.joy
    }

    pub fn energy(&self) -> Stat {
        self.energy
    }

    pub fn health(&self) -> Stat {
        self.health
    }

    pub fn is_asleep(&self) -> bool {
        self.asleep
    }

    pub fn growth_stage(&self) -> GrowthStage {
        self.growth_stage
    }

    pub fn alive(&self) -> &bool {
        &self.alive
    }

    pub fn time_created(&self) -> i64 { self.time_created }
    
    pub fn shape(&self) -> Texture2D {
        match self.growth_stage {
            GrowthStage::Egg => egg_shape(),
            GrowthStage::Baby => baby_shape(),
            GrowthStage::Kid => kid_shape(),
            GrowthStage::Adult => self.shape.get_texture(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::creature::{Creature, MINUTE_MILLIS};
    use crate::shapes::CreatureShapes;
    use crate::utils::Stat;

    #[test]
    fn update_alive_status() {
        // Arrange
        let mut creature_a = Creature::new("A", CreatureShapes::Snail, 0);
        creature_a.food = Stat::new(0).unwrap();
        creature_a.joy = Stat::new(0).unwrap();

        let mut creature_b = Creature::new("B", CreatureShapes::Mouse, 0);
        creature_b.food = Stat::new(5).unwrap();
        creature_b.joy = Stat::new(5).unwrap();
        creature_b.energy = Stat::new(5).unwrap();
        creature_b.health = Stat::new(4).unwrap();

        let mut creature_c = Creature::new("C", CreatureShapes::Squid, 0);
        creature_c.energy = Stat::new(0).unwrap();
        creature_c.food = Stat::new(5).unwrap();
        creature_c.joy = Stat::new(5).unwrap();
        creature_c.health = Stat::new(5).unwrap();


        // Act
        creature_a.update_alive_status();
        creature_b.update_alive_status();
        creature_c.update_alive_status();


        // Assert
        assert!(!creature_a.alive);  // Should be dead
        assert!(!creature_b.alive);  // Should be dead
        assert!(creature_c.alive);   // Should be alive
    }
}