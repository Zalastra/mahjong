use std::fmt;

use tile::tile_type::*;
use sdl2::render::Texture;

// NOTE might need to moved to a different file based on how we implement the board/tile structure
#[derive(PartialEq, Clone)]
pub struct TilePosition {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub struct Tile {
    pub position: TilePosition,
    pub texture: Texture,
    kind: TileType,
    positions: Vec<TilePosition>,
}

impl Tile {
    pub fn new(position: TilePosition, texture: Texture, kind: TileType) -> Tile {
        let positions = vec![position.clone(),
                             TilePosition {x: position.x + 1, y: position.y, z: position.z },
                             TilePosition {x: position.x, y: position.y + 1, z: position.z },
                             TilePosition {x: position.x + 1, y: position.y + 1, z: position.z }];

        Tile {
            position: position,
            texture: texture,
            positions: positions,
            kind: kind,
        }
    }

    pub fn matches(&self, other: &Tile) -> bool {
        self.kind.matches(&other.kind)
    }

    pub fn is_on_position(&self, other_position: TilePosition) -> bool {
        for position in self.positions.iter() {
            if *position == other_position { return true }
        }
        false
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.position.z, self.kind)
    }
}
