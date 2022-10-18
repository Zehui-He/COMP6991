//! This module implements the Player object and its behaviour.
use crate::movement::MovementTrait;


/// The Player struct is used to represent a player and its state in the game.
/// The states of the player includes his x and y position in the viewport
/// and the character which representing the player.
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub repr: char 
}

/// This function would defaultly construct a player object with coordinaation
/// of x = 3 and y = 3 and a visual representation of ♟.
/// 
/// # Example
/// 
/// ```
/// use adventurers::player::Player;
/// let player = Player::new();
/// assert_eq!(player.x, 3);
/// assert_eq!(player.y, 3);
/// assert_eq!(player.repr, '♟');
/// ```
impl Player {
    pub fn new() -> Self {
        Player { x: 3, y: 3, repr: '♟'}
    }
}


/// Implement the movement of a player via the MovementTrait.
/// 
/// # Example
/// 
/// ```
/// use adventurers::player::Player;
/// use adventurers::movement::{Movement, MovementTrait};
/// 
/// // By default, the player should begin at (3,3)
/// let mut player = Player::new();
/// assert_eq!(player.x, 3);
/// assert_eq!(player.y, 3);
/// 
/// // Ask the player to move down by 1. The new position of the
/// // player should be (3,4).
/// player.move_by(Movement::down());
/// assert_eq!(player.x, 3);
/// assert_eq!(player.y, 4);
/// ```
impl MovementTrait for Player {
    fn move_by(&mut self, movement: crate::movement::Movement) {
        self.x += movement.x;
        self.y += movement.y;
    }
}