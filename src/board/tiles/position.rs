use self::Direction::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn neighbours(self, other: Position) -> Option<Direction> {
        if self.is_up_neighbour(other) { Some(Up) }
        else if self.is_down_neighbour(other) { Some(Down) }
        else if self.is_left_neighbour(other) { Some(Left) }
        else if self.is_right_neighbour(other) { Some(Right) }
        else { None }
    }

    fn is_up_neighbour(self, other: Position) -> bool {
        self.z + 1 == other.z && self.is_potential_vertical_neighbour(other)
    }

    fn is_down_neighbour(self, other: Position) -> bool {
        self.z == other.z + 1 && self.is_potential_vertical_neighbour(other)
    }

    fn is_potential_vertical_neighbour(self, other: Position) -> bool {
        self.x <= other.x + 1 && self.x + 1 >= other.x
            && self.y <= other.y + 1 && self.y + 1 >= other.y
    }

    fn is_left_neighbour(self, other: Position) -> bool {
        self.x == other.x + 2 && self.is_potential_horizontal_neighbour(other)
    }

    fn is_right_neighbour(self, other: Position) -> bool {
        self.x + 2 == other.x && self.is_potential_horizontal_neighbour(other)
    }

    fn is_potential_horizontal_neighbour(self, other: Position) -> bool {
        self.z == other.z && self.y <= other.y + 1 && self.y + 1 >= other.y
    }
}

impl<'a> From<&'a (u8, u8, u8)> for Position {
    fn from(&(x, y, z): &(u8, u8, u8)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn rev(self) -> Direction {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Neighbour {
    pub id: usize,
    pub direction: Direction,
}

impl Neighbour {
    pub fn new(id: usize, direction: Direction) -> Self {
        Self { id, direction }
    }
}
