use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::include_texture;
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
    
    /// Updates the state of the **CoinDropManager**, this includes:
    /// * Try spawning a coin drop.
    /// * Update creature-coin collisions.
    /// 
    /// ## Parameters:
    /// * `creature_loc` - The current location of the creature, this is needed to check if the
    ///   creature has picked up a coin.
    /// 
    /// ## Returns:
    /// This function returns `true` when the creature has picked up a coin, and false otherwise.
    pub fn update(&mut self, creature_loc: Location) -> bool {
        self.try_spawn_coin();
        
        self.update_collisions(creature_loc)
    }
    
    /// Draws the dropped coin on the screen, if one is present.
    pub fn draw_coin(&self) {
        if let Some(coin) = self.dropped_coin {
            let texture = include_texture!("../resources/coin.png");
            draw_texture_ex(
                &texture,
                coin.0.x,
                coin.0.y,
                YELLOW,
                DrawTextureParams::default(),
            );
        }
    }
    
    /// Checks if the creature collides with the dropped coin when present. If it does, this function
    /// sets `self.dropped_coin` to `None` and returns `true`.
    fn update_collisions(&mut self, creature_loc: Location) -> bool {
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
        
        false
    }
    
    fn try_spawn_coin(&mut self) {
        if self.creature_love.value() < 80 || self.dropped_coin.is_some() {
            return;
        }
        
        self.drop_timer += get_frame_time();
        if self.drop_timer > 300. {
            self.drop_timer = 0.;
            
            if gen_range(0, 100) < 5 {
                self.dropped_coin = Some(DroppedCoin::new_random());
            }
        }
    }
}
