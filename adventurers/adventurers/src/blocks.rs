//! This module implements the Blocks object and its behaviour.
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
/// The Block emun type lists all the possible type of blocks that would
/// appear in the game.
pub enum Blocks {
    Grass,
    Sand,
    Rock,
    Cinderblock,
    Flowerbush,
    Barrier,
    Water,
    Object(char),
    Sign(String)
}
