use macroquad::rand::gen_range;

pub fn get_random_bool() -> bool {
    matches!(
        gen_range(0, i32::MAX) % 2,
        0
    )
}