//! This module implements the enum type of blocks.
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone, PartialEq)]
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
