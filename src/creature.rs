use macroquad::texture::Texture2D;
use macroquad::rand::gen_range;
use serde::{Deserialize, Serialize};
use crate::food::Food;
use crate::creature_game::CreatureGame;
use crate::creature_personality::CreaturePersonality;
use crate::shapes::{CreatureShapes, egg_shape, baby_shape, kid_shape};
use crate::utils::{time::get_now_millis, Stat};

const MINUTE_MILLIS: i64 = 1000 * 60;
const FOOD_OFFSET_MILLIS: i64 = 16 * MINUTE_MILLIS;
const ENERGY_OFFSET_MILLIS: i64 = 3 * MINUTE_MILLIS;
const JOY_OFFSET_MILLIS: i64 = 18 * MINUTE_MILLIS;
const HEALTH_OFFSET_MILLIS: i64 = 1000 * 12;   // 12 seconds, 5 times a minute triggered

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creature {
    name: String,
    food: Stat,
    joy: Stat,
    energy: Stat,
    health: Stat,
    love: Stat, // Hidden stat that determines how much the creature likes the player.
    personality: CreaturePersonality,
    
    previous_food_update: i64,
    previous_joy_update: i64,
    previous_energy_update: i64,
    previous_health_update: i64,
    
    shape: CreatureShapes,
    growth_stage: GrowthStage,
    asleep_since: Option<i64>,
    is_sick: bool,
    alive: bool,
    time_created: i64,
    time_of_death: Option<i64>,
}

impl Creature {
    pub fn new(name: &str, shape: CreatureShapes, now_millis: i64) -> Self {
        Self {
            name: String::from(name),
            food: Stat::new(50).unwrap(),
            joy: Stat::new(50).unwrap(),
            energy: Stat::new(50).unwrap(),
            health: Stat::new(50).unwrap(),
            love: Stat::new(0).unwrap(),
            personality: CreaturePersonality::new_random(),
            
            previous_food_update: now_millis,
            previous_joy_update: now_millis,
            previous_energy_update: now_millis,
            previous_health_update: now_millis,
            
            shape,
            growth_stage: GrowthStage::Egg,
            asleep_since: None,
            is_sick: false,
            alive: true,
            time_created: now_millis,
            time_of_death: None,
        }
    }

    /// Updates this creature's state for each minute passed since last update.
    ///
    /// # Parameters:
    /// `now_millis` - The current time in milliseconds based on **SystemTime**
    pub fn update_state(&mut self, now_millis: i64) {
        self.update_growth_stage(now_millis);

        if self.growth_stage != GrowthStage::Egg {
            self.update_stats(now_millis);
        }

        // If the creature is still sleeping while its energy is already full, wake it up.
        if self.is_asleep() && self.energy.value() == 100 {
            self.asleep_since = None;
        }
    }

    fn update_stats(&mut self, now_millis: i64) {
        // Use while loops instead of if statements to account for loading from file
        // when we might have been away for more than a single minute.
        while now_millis - self.previous_food_update >= FOOD_OFFSET_MILLIS && self.alive{
            self.food.subtract(1);
            self.previous_food_update += FOOD_OFFSET_MILLIS;

            self.update_alive_status(self.previous_food_update);
        }

        while now_millis - self.previous_energy_update >= ENERGY_OFFSET_MILLIS && self.alive {
            if self.is_asleep() {
                self.energy.add(1);
            }

            self.previous_energy_update += ENERGY_OFFSET_MILLIS;
            self.update_alive_status(self.previous_energy_update);
        }

        while now_millis - self.previous_joy_update >= JOY_OFFSET_MILLIS && self.alive {
            self.joy.subtract(1);
            self.previous_joy_update += JOY_OFFSET_MILLIS;

            self.update_alive_status(self.previous_joy_update);
        }

        while now_millis - self.previous_health_update >= HEALTH_OFFSET_MILLIS && self.alive {
            if self.is_sick {
                self.health.subtract(20);
            } else {
                self.health.add(1);
            }

            self.previous_health_update += HEALTH_OFFSET_MILLIS;
            self.update_alive_status(self.previous_health_update);
        }
    }

    /// Calls the creature's `die()` method when appropriate, updating its `alive` status.
    ///
    /// ## Parameters:
    /// * `update_time` - The time at which the creature's alive status should be updated. This time
    ///   is only used to calculate and display the creature's age on the death screen.
    fn update_alive_status(&mut self, update_time: i64) {
        let stats_sum = self.food.value() + self.joy.value();
        if stats_sum < 15 {
            self.die(update_time)
        }

        for stat in [self.food, self.joy, self.health] {
            if stat.value() == 0 {
                self.die(update_time)
            }
        }
    }

    /// Sets the creature's "alive" stat to `false` and records the time of death.
    ///
    /// ## Parameters:
    /// * `time_of_death` - The time at which the creature has died in millis.
    fn die(&mut self, time_of_death: i64) {
        self.alive = false;
        self.time_of_death = Some(time_of_death);
    }

    fn update_growth_stage(&mut self, now_millis: i64) {
        let growth_delay = match self.growth_stage {
            GrowthStage::Egg => Some(MINUTE_MILLIS),
            GrowthStage::Baby => Some(60 * MINUTE_MILLIS),
            GrowthStage::Kid => Some(5 * 60 * MINUTE_MILLIS),
            GrowthStage::Adult => None,
        };

        if let Some(growth_delay) = growth_delay &&
            now_millis - self.time_created > growth_delay {
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
    ///   creature receives, is indicated by the `Food::points()` method.
    pub fn eat(&mut self, food: Food) {
        if self.growth_stage == GrowthStage::Egg || self.is_sick {
            return;
        }
        let now = get_now_millis();

        self.food.add(food.points());
        self.previous_food_update = now;
        
        if self.personality.liked_food() == food {
            self.love.add(5);
        } else if self.personality.hated_food() == food {
            self.love.subtract(5);
        }

        // The creature has a 1/3 chance of getting sick when eating
        if gen_range(0, 3) == 0 {
            self.is_sick = true;
            self.health.subtract(20);
            self.previous_health_update = now;
        }
    }

    /// Interaction used to make the creature sleep or wake up, when it sleeps, its `asleep_since` state
    /// will be set to now.
    pub fn toggle_sleep(&mut self) {
        if self.growth_stage == GrowthStage::Egg {
            return;
        }

        if self.asleep_since.is_none() {
            self.asleep_since = Some(get_now_millis());
        } else {
            self.asleep_since = None;
        }
    }

    /// Interaction used to *"play"* with the creature in order to increase its `joy` stat. This also
    /// adds more time to its `health_decrease_time_left` field, and decreases its `energy` by 20.
    pub fn play(&mut self, game: CreatureGame) {
        if self.growth_stage == GrowthStage::Egg || 
            self.energy.value() < game.energy_cost() ||
            self.joy.value() == 100
        {
            return;
        }

        self.joy.add(game.points());
        self.previous_joy_update = get_now_millis();
        self.energy.subtract(game.energy_cost());
        
        if self.personality.liked_game() == game {
            self.love.add(5);
        } else if self.personality.hated_game() == game {
            self.love.subtract(5);
        }
    }

    /// Interaction used to give the creature some medicine in order to increase its `health` stat.
    pub fn heal(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.is_sick = false;
            self.previous_health_update = get_now_millis();
            
            self.love.subtract(5);
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
    
    pub fn love(&self) -> Stat {
        self.love
    }

    pub fn is_asleep(&self) -> bool {
        self.asleep_since.is_some()
    }

    pub fn growth_stage(&self) -> GrowthStage {
        self.growth_stage
    }

    pub fn alive(&self) -> &bool {
        &self.alive
    }
    
    pub fn personality(&self) -> CreaturePersonality {
        self.personality
    }
    
    pub fn is_sick(&self) -> bool {
        self.is_sick
    }

    pub fn time_created(&self) -> i64 { 
        self.time_created
    }
    
    pub fn time_of_death(&self) -> Option<i64> {
        self.time_of_death
    }
    
    pub fn shape(&self) -> CreatureShapes {
        self.shape
    }
    
    pub fn texture(&self) -> Texture2D {
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
    use crate::creature::{Creature, GrowthStage};
    use crate::shapes::CreatureShapes;
    use crate::utils::Stat;

    #[test]
    fn update_alive_status() {
        // Arrange
        let example_time = 1000;

        // Should be death
        let mut creature_a = Creature::new("A", CreatureShapes::Snail, 0);
        creature_a.food = Stat::new(0).unwrap();
        creature_a.joy = Stat::new(0).unwrap();

        // Should be death
        let mut creature_b = Creature::new("B", CreatureShapes::Mouse, 0);
        creature_b.food = Stat::new(6).unwrap();
        creature_b.joy = Stat::new(8).unwrap();
        creature_b.energy = Stat::new(100).unwrap();
        creature_b.health = Stat::new(100).unwrap();

        // Should be alive
        let mut creature_c = Creature::new("C", CreatureShapes::Squid, 0);
        creature_c.food = Stat::new(7).unwrap();
        creature_c.joy = Stat::new(8).unwrap();
        creature_c.energy = Stat::new(0).unwrap();
        creature_c.health = Stat::new(1).unwrap();


        // Act
        creature_a.update_alive_status(example_time);
        creature_b.update_alive_status(example_time);
        creature_c.update_alive_status(example_time);


        // Assert
        assert!(!creature_a.alive);  // Should be dead
        assert!(!creature_b.alive);  // Should be dead
        assert!(creature_c.alive);   // Should be alive
    }
    
    #[test]
    fn update_love_stat() {
        let mut creature = Creature::new("test", CreatureShapes::Sheep, 0);
        creature.growth_stage = GrowthStage::Adult;
        
        let liked_food = creature.personality().liked_food();
        let hated_food = creature.personality().hated_food();

        let liked_game = creature.personality().liked_game();
        let hated_game = creature.personality().hated_game();
        
        
        let love = creature.love().value();
        creature.eat(liked_food);
        assert!(love < creature.love().value());
        
        let love = creature.love().value();
        creature.eat(hated_food);
        assert!(love > creature.love().value());
        
        let love = creature.love().value();
        creature.play(liked_game);
        assert!(love < creature.love().value());

        let love = creature.love().value();
        creature.play(hated_game);
        assert!(love > creature.love().value());
    }
}
