use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

/// Takes in a max value and returns a random number between 1 and max
fn get_random_value(max: u8) -> u8 {
    let mut rng = ChaCha20Rng::seed_from_u64(2);
    rng.gen_range(1..=max)
}

pub struct Coin;

//TODO - create a trait that can be used for both die and coin
//      and implement the trait for both die and coin
pub fn roll<T:Roll>(item: T) -> u8 
{
    item.roll_dice_coin()
}

pub trait Roll {
    fn roll_dice_coin(&self) -> u8;
}

impl Roll for Coin {
    fn roll_dice_coin(&self) -> u8 {
        get_random_value(2)
    }
}

impl Roll for Die {
    fn roll_dice_coin(&self) -> u8 {
        let max = match &self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
        };
        get_random_value(max)
    }
}