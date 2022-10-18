pub struct Movement {
    pub x: i32,
    pub y: i32
}

impl Movement {
    pub fn left() -> Movement {
        Movement { x: -1, y: 0 }
    }

    pub fn right() -> Movement {
        Movement { x: 1, y: 0 }
    }

    pub fn up() -> Movement {
        Movement { x: 0, y: -1 }
    }

    pub fn down() -> Movement {
        Movement { x: 0, y: 1 }
    }
}

pub trait MovementTrait {
    fn move_by(&mut self, movement: Movement);
}