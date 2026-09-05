use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::utils::{Stat, Location, Dimensions};
use crate::ui::play_area::PLAY_AREA_RECT;




#[derive(Debug, Clone, Copy, PartialEq)]
struct DroppedCoin(Rect);
impl DroppedCoin {
    const COIN_DIMENSIONS: Dimensions = Dimensions { width: 5., height: 10. };
    
    /// Creates a new `DroppedCoin` instance with a random location within the *'playing area'* on the
    /// screen.
    pub fn new_random() -> Self {
        let x = gen_range(
            PLAY_AREA_RECT.x,
            PLAY_AREA_RECT.right() - Self::COIN_DIMENSIONS.width,
        );

        let y = gen_range(
            PLAY_AREA_RECT.y,
            PLAY_AREA_RECT.bottom() - Self::COIN_DIMENSIONS.height,
        );
        
        Self(Rect::new(
            x,
            y,
            Self::COIN_DIMENSIONS.width,
            Self::COIN_DIMENSIONS.height,
        ))
    }
}


#[derive(Debug, Clone)]
pub struct CoinDropManager<'a> {
    drop_timer: f32,
    creature_love: &'a Stat,
    dropped_coin: Option<DroppedCoin>,
}

impl<'a> CoinDropManager<'a> {
    pub fn new(creature_love: &'a Stat) -> Self {
        Self {
            creature_love,
            
            drop_timer: 0.,
            dropped_coin: None,
        }
    }
    
    /// Updates the state of dropped coins and their collisions with the creature. This functions
    /// returns `true` when the creature collides with a coin, and `false` otherwise.
    pub fn update_collisions(&mut self, creature_loc: Location) -> bool {
        let creature_rect = Rect::new(
            creature_loc.x,
            creature_loc.y,
            25.,
            25.,
        );
        
        if let Some(coin) = self.dropped_coin {
            if coin.0.overlaps(&creature_rect) {
                self.dropped_coin = None;
                return true;
            }
        }
        
        self.try_spawn_coins();
        false
    }
    
    fn try_spawn_coins(&mut self) {
        if self.creature_love.value() < 80 || self.dropped_coin.is_some() {
            return;
        }
        
        self.drop_timer += get_frame_time();
        if self.drop_timer > 300. {
            self.drop_timer = 0.;
            
            // TODO: Only create new coin drop depending on random chance based of love stat.
            self.dropped_coin = Some(DroppedCoin::new_random());
        }
    }
}
