//! This module implements the Player object and its behaviour.
use termgame::{Game, Message};
use crate::movement::{MovementTrait, Coordinate};


/// The Player struct is used to represent a player and its state in the game.
/// The states of the player includes his x and y position in the viewport
/// and the character which representing the player.
pub struct Player {
    position: Coordinate,
    pub repr: char,
    pub in_water: bool,
    pub steps_in_water: u8
}

/// This function would defaultly construct a player object with coordinaation
/// of x = 3 and y = 3 and a visual representation of ♟.
/// 
/// # Example
/// 
/// ```
/// use adventurers::player::Player;
/// use crate::adventurers::movement::MovementTrait;
/// 
/// let player = Player::new();
/// let player_pos = player.get_position();
/// assert_eq!(player_pos.x, 3);
/// assert_eq!(player_pos.y, 3);
/// assert_eq!(player.repr, '♟');
/// ```
impl Player {
    pub fn new() -> Self {
        Player { 
            position: Coordinate { x: 3, y: 3 },
            repr: '♟',
            in_water: false,
            steps_in_water: 0
        }
    }
    
    pub fn walk_in_water(&mut self, game: &mut Game) {
        if !self.in_water {
            self.in_water = true;
        }
        self.steps_in_water += 1;
        
        // If the player walked 10 steps in water, show dead message
        if self.steps_in_water == 10 {
            game.set_message(Some(Message{ title: Some("Message".to_string()), text:"You are drowned.".to_string() }));
        }
    }

    pub fn walk_out_water(&mut self) {
        if self.in_water {
            self.in_water = false;
        }
        self.steps_in_water = 0;
    }
}

impl std::default::Default for Player {
    fn default() -> Self {
        Player::new()
    }
}


/// Implement the movement of a player via the MovementTrait.
/// 
/// # Example
/// 
/// ```
/// use adventurers::player::Player;
/// use adventurers::movement::{Coordinate, MovementTrait};
/// 
/// // By default, the player should begin at (3,3)
/// let mut player = Player::new();
/// assert_eq!(player.get_position().x, 3);
/// assert_eq!(player.get_position().y, 3);
/// 
/// // Ask the player to move down by 1. The new position of the
/// // player should be (3,4).
/// player.move_by(Coordinate::down());
/// assert_eq!(player.get_position().x, 3);
/// assert_eq!(player.get_position().y, 4);
/// ```
impl MovementTrait for Player {
    fn move_by(&mut self, movement: &Coordinate) {
        self.position.x += movement.x;
        self.position.y += movement.y;
    }

    fn get_position(&self) -> &Coordinate {
        &self.position
    }

    fn get_x_pos(&self) -> i32 {
        self.position.x
    }

    fn get_y_pos(&self) -> i32 {
        self.position.y
    }
}