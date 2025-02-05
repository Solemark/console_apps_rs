use rand::prelude::*;

#[allow(dead_code)]
pub fn roll(max_roll: u32) -> u32 {
    rand::rng().random_range(1..=max_roll)
}
