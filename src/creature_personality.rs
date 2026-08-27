use macroquad::rand::gen_range;
use crate::food::Food;
use crate::creature_game::CreatureGame;
use crate::ui::interaction_menu::CreatureInteraction;


#[derive(Debug, Clone, Copy)]
pub struct CreaturePersonality {
    pub liked_food: Food,
    pub hated_food: Food,
    
    pub liked_game: CreatureGame,
    pub hated_game: CreatureGame,
}

impl CreaturePersonality {
    /// Generates a new `CreaturePersonality` instance with all its liked and hated interactions set
    /// randomly. Duplicates are prevented.
    pub fn new_random() -> Self {
        let mut all_food = Food::all_variants().to_vec();
        let mut all_games = CreatureGame::all_variants().to_vec();
        
        let liked_food = all_food.remove(gen_range(0, all_food.len()));
        let hated_food = all_food.remove(gen_range(0, all_food.len()));

        let liked_game = all_games.remove(gen_range(0, all_games.len()));
        let hated_game = all_games.remove(gen_range(0, all_games.len()));
        
        Self {
            liked_food,
            hated_food,
            liked_game,
            hated_game,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::creature_personality::CreaturePersonality;
    
    #[test]
    fn no_duplicate_personality_interactions() {
        for _ in 0..100 {
            let personality = CreaturePersonality::new_random();
            
            assert_ne!(personality.liked_food, personality.hated_food);
            assert_ne!(personality.liked_game, personality.hated_game);
        }
    }
}
