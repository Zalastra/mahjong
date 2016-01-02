#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Position {
            x: x,
            y: y,
            z: z,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TilePosition {
    positions: [Position; 4],
}

impl TilePosition {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        TilePosition {
            positions: [Position::new(x, y, z),
                        Position::new(x + 1, y, z),
                        Position::new(x, y + 1, z),
                        Position::new(x + 1, y + 1, z)]
        }
    }

    pub fn x(&self) -> u8 {
        self.positions[0].x
    }

    pub fn y(&self) -> u8 {
        self.positions[0].y
    }

    pub fn z(&self) -> u8 {
        self.positions[0].z
    }

    pub fn is_on_position(&self, other: &Position) -> bool {
        for position in self.positions.iter() {
            if *position == *other { return true }
        }
        false
    }

    pub fn is_neighbour_of(&self, other: &TilePosition) -> bool {
        self.is_left_of(other) || self.is_right_of(other) ||
            self.is_above(other) || self.is_under(other)
    }

    pub fn is_left_of(&self, other: &TilePosition) -> bool {
        let position1 = Position::new(self.x() + 2, self.y(), self.z());
        let position2 = Position::new(self.x() + 2, self.y() + 1, self.z());
        other.is_on_position(&position1) || other.is_on_position(&position2)
    }

    pub fn is_right_of(&self, other: &TilePosition) -> bool {
        let position1 = Position::new(other.x() + 2, other.y(), other.z());
        let position2 = Position::new(other.x() + 2, other.y() + 1, other.z());
        self.is_on_position(&position1) || self.is_on_position(&position2)
    }

    pub fn is_above(&self, other: &TilePosition) -> bool {
        let position1 = Position::new(other.x(), other.y(), other.z() + 1);
        let position2 = Position::new(other.x() + 1, other.y(), other.z() + 1);
        let position3 = Position::new(other.x(), other.y() + 1, other.z() + 1);
        let position4 = Position::new(other.x() + 1, other.y() + 1, other.z() + 1);

        self.is_on_position(&position1) || self.is_on_position(&position2) ||
        self.is_on_position(&position3) || self.is_on_position(&position4)
    }

    pub fn is_under(&self, other: &TilePosition) -> bool {
        let position1 = Position::new(self.x(), self.y(), self.z() + 1);
        let position2 = Position::new(self.x() + 1, self.y(), self.z() + 1);
        let position3 = Position::new(self.x(), self.y() + 1, self.z() + 1);
        let position4 = Position::new(self.x() + 1, self.y() + 1, self.z() + 1);

        other.is_on_position(&position1) || other.is_on_position(&position2) ||
        other.is_on_position(&position3) || other.is_on_position(&position4)
    }
}
