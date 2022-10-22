//! This module define the Movement struct and the MovementTrait.
//! The movement struct implies the position of the object(such as player)
//! or the number of movement in x and y direction.
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

impl Coordinate {
    pub fn left() -> Coordinate {
        Coordinate { x: -1, y: 0 }
    }

    pub fn right() -> Coordinate {
        Coordinate { x: 1, y: 0 }
    }

    pub fn up() -> Coordinate {
        Coordinate { x: 0, y: -1 }
    }

    pub fn down() -> Coordinate {
        Coordinate { x: 0, y: 1 }
    }
}

pub trait MovementTrait {
    fn get_x_pos(&self) -> i32;
    fn get_y_pos(&self) -> i32;
    fn get_position(&self) -> &Coordinate;
    fn move_by(&mut self, movement: Coordinate);
}