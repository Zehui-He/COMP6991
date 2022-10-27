//! This module implements the enum type of blocks.
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
    Sign(String),
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Blocks::Grass => write!(f, "Grass block"),
            Blocks::Sand => write!(f, "Sand block"),
            Blocks::Rock => write!(f, "Rock block"),
            Blocks::Cinderblock => write!(f, "Cinder block"),
            Blocks::Flowerbush => write!(f, "Flower bush"),
            Blocks::Water => write!(f, "Water block"),
            _ => write!(f, "Other block"),
        }
    }
}
