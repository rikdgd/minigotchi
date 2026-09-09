use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::include_texture;
use crate::creature::Creature;
use crate::shapes::CreatureShape;
use crate::utils::{Location, Dimensions};
use crate::ui::play_area::{PLAY_AREA_RECT, play_area_background_color};


#[derive(Debug, Clone, Copy, PartialEq)]
struct DroppedCoin(Rect);
impl DroppedCoin {
    const COIN_DIMENSIONS: Dimensions = Dimensions { width: 9., height: 12. };
    
    /// Creates a new `DroppedCoin` instance with a random location within the *'playing area'* on the
    /// screen.
    pub fn new_random() -> Self {
        let x = gen_range(
            PLAY_AREA_RECT.x,
            PLAY_AREA_RECT.right() - Self::COIN_DIMENSIONS.width,
        ).round();

        let y = gen_range(
            PLAY_AREA_RECT.y,
            PLAY_AREA_RECT.bottom() - Self::COIN_DIMENSIONS.height,
        ).round();
        
        Self(Rect::new(
            x,
            y,
            Self::COIN_DIMENSIONS.width,
            Self::COIN_DIMENSIONS.height,
        ))
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct CoinDropManager {
    drop_timer: f32,
    dropped_coin: Option<DroppedCoin>,
}

impl CoinDropManager {
    /// This is the delay used for coin drops. Every `COIN_DROP_DELAY` seconds an attempt is made to
    /// spawn a new `DroppedCoin`.
    pub const COIN_DROP_DELAY: f32 = 300.;
    
    /// Updates the state of the **CoinDropManager**, this includes:
    /// * Try spawning a coin drop.
    /// * Update creature-coin collisions.
    /// 
    /// ## Parameters:
    /// * `creature_loc` - The current location of the creature, this is needed to check if the
    ///   creature has picked up a coin.
    /// * `creature` - A reference to the creature, used to get the value of its *love* stat.
    /// 
    /// ## Returns:
    /// This function returns `true` when the creature has picked up a coin, and `false` otherwise.
    pub fn update(&mut self, creature_loc: Location, creature: &Creature) -> bool {
        self.try_spawn_coin(creature);
        self.update_collisions(creature_loc)
    }
    
    /// Draws the dropped coin on the screen, if one is present.
    pub fn draw_coin(&self, creature_asleep: bool) {
        if let Some(coin) = self.dropped_coin {
            let coin_texture = include_texture!("../resources/dropped_items/coin.png");
            let backdrop_texture = include_texture!("../resources/dropped_items/coin_backdrop.png");

            draw_texture_ex(
                &backdrop_texture,
                coin.0.x,
                coin.0.y,
                play_area_background_color(creature_asleep),
                DrawTextureParams::default(),
            );
            
            draw_texture_ex(
                &coin_texture,
                coin.0.x,
                coin.0.y,
                BLACK,
                DrawTextureParams::default(),
            );
        }
    }
    
    /// Checks if the creature collides with the dropped coin when present. If it does, this function
    /// sets `self.dropped_coin` to `None` and returns `true`.
    fn update_collisions(&mut self, creature_loc: Location) -> bool {
        let creature_center = Vec2::new(
            creature_loc.x + CreatureShape::TEXTURE_DIMENSIONS.width / 2.,
            creature_loc.y + CreatureShape::TEXTURE_DIMENSIONS.height / 2.,
        );
        
        if let Some(coin) = self.dropped_coin {
            if coin.0.contains(creature_center) {
                self.dropped_coin = None;
                return true;
            }
        }
        
        false
    }
    
    fn try_spawn_coin(&mut self, creature: &Creature) {
        if creature.love().value() < 80 || self.dropped_coin.is_some() {
            return;
        }
        
        self.drop_timer += get_frame_time();
        if self.drop_timer > Self::COIN_DROP_DELAY {
            self.drop_timer = 0.;
            
            if gen_range(0, 100) < 5 {
                self.dropped_coin = Some(DroppedCoin::new_random());
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::coin_drops::*;
    use crate::creature::Creature;
    use crate::shapes::CreatureShape;
    use crate::Location;
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct CoinCreatureLocationSet {
        pub creature_location: Location,
        pub coin_location: Location,
        pub expected_result: bool,
    }

    #[test]
    fn coin_pickup_radius() {
        let mut creature = Creature::new("test", CreatureShape::Bunny, 0);
        creature.set_love(100).unwrap();
        
        let test_sets = [
            CoinCreatureLocationSet {
                creature_location: (50., 20.).into(),
                coin_location: (
                    50. + CreatureShape::TEXTURE_DIMENSIONS.width / 2. - 1.,
                    20. + CreatureShape::TEXTURE_DIMENSIONS.height / 2. - 1.,
                    ).into(),
                expected_result: true,
            },
            CoinCreatureLocationSet {
                creature_location: (10., 10.).into(),
                coin_location: (100., 20.).into(),
                expected_result: false,
            },
            CoinCreatureLocationSet {
                creature_location: (50., 20.).into(),
                coin_location: (
                    50. + CreatureShape::TEXTURE_DIMENSIONS.width,
                    20.,
                ).into(),
                expected_result: false,
            },
            CoinCreatureLocationSet {
                creature_location: (50., 20.).into(),
                coin_location: (
                    50.,
                    20. + CreatureShape::TEXTURE_DIMENSIONS.height,
                ).into(),
                expected_result: false,
            },
        ];
        
        for test_set in test_sets {
            let mut coin_manager = CoinDropManager::default();
            coin_manager.dropped_coin = Some(DroppedCoin(Rect::new(
                test_set.coin_location.x,
                test_set.coin_location.y,
                DroppedCoin::COIN_DIMENSIONS.width,
                DroppedCoin::COIN_DIMENSIONS.height,
            )));
            
            let result = coin_manager.update(test_set.creature_location, &creature);
            assert_eq!(test_set.expected_result, result);
        }
    }
    
    #[macroquad::test]
    async fn coin_spawn_timer() {
        let mut creature = Creature::new("test", CreatureShape::Bunny, 0);
        let mut coin_manager = CoinDropManager::default();
        
        
        // Too low friendship AND to little time passed:
        coin_manager.drop_timer = CoinDropManager::COIN_DROP_DELAY / 2.;
        creature.set_love(10).unwrap();
        coin_manager.try_spawn_coin(&creature);
        
        assert_eq!(coin_manager.drop_timer.round(), CoinDropManager::COIN_DROP_DELAY / 2.);

        
        // Enough friendship AND time passed:
        coin_manager.drop_timer = CoinDropManager::COIN_DROP_DELAY * 2.;
        creature.set_love(100).unwrap();

        coin_manager.try_spawn_coin(&creature);
        assert_eq!(coin_manager.drop_timer.round(), 0.);
        
        
        // Enough friendship BUT NOT time passed:
        coin_manager.drop_timer = CoinDropManager::COIN_DROP_DELAY / 2.;
        creature.set_love(90).unwrap();
        
        coin_manager.try_spawn_coin(&creature);
        assert_eq!(coin_manager.drop_timer.round(), CoinDropManager::COIN_DROP_DELAY / 2.);
        
        
        // Too low friendship BUT enough time passed:
        coin_manager.drop_timer = CoinDropManager::COIN_DROP_DELAY * 1.1;
        creature.set_love(50).unwrap();
        
        coin_manager.try_spawn_coin(&creature);
        assert_eq!(coin_manager.drop_timer.round(), CoinDropManager::COIN_DROP_DELAY * 1.1);
    }
}
