use std::fmt;

use tiles::tile_type::*;

// NOTE might need to moved to a different file based on how we implement the board/tile structure
#[derive(PartialEq, Clone)]
pub struct TilePosition {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub struct Tile {
    pub position: TilePosition,
    pub kind: TileType,
}

impl Tile {
    pub fn new(position: TilePosition, kind: TileType) -> Tile {
        Tile {
            position: position,
            kind: kind,
        }
    }

    pub fn is_blocked(&self) -> bool {
        true//self.neighbours.borrow().len() == 2 || self.blocked_by.get() > 0
    }

    pub fn matches(&self, other: &Tile) -> bool {
        self.kind.matches(&other.kind)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.position.z, self.kind)
    }
}
