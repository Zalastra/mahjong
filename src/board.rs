use std::collections::HashMap;

use tile::{Tile, Position, Type, Nonary, Wind, Dragon, Flower, Season};
use tile::Nonary::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine};
use tile::Wind::{North, South, East, West};
use tile::Dragon::{Red, Green, White};
use tile::Flower::{Plum, Orchid, Chrysanthemum, Bamboo};
use tile::Season::{Spring, Summer, Autumn, Winter};

// TODO: added for randomizing the board, might be a better way
//static TYPES: [Type; 7] = [Circle, Type::Bamboo, Type::Character, Type::Wind, Type::Dragon, Type::Flower, Type::Season];
static NONARY: [Nonary; 9] = [One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
static WINDS: [Wind; 4] = [North, South, East, West];
static DRAGONS: [Dragon; 3] = [Red, Green, White];
static FLOWERS: [Flower; 4] = [Plum, Orchid, Chrysanthemum, Bamboo];
static SEASONS: [Season; 4] = [Spring, Summer, Autumn, Winter];

pub struct Board {
    height: u8,
    width: u8,
    depth: u8,
    pub tiles: HashMap<u32, Tile>,
    reachable_tiles: Vec<u32>,
}

impl Board {
    // TODO: implement tile randomizing
    // TODO: take in a immutable borrowed vector instead of moving it
    pub fn new(mut positions: Vec<(Position, Type)>) -> Board {
        let mut tiles = HashMap::new();
        let (mut height, mut width, mut depth) = (4, 4, 0);

        while !positions.is_empty() {
            let (pos, kind) = positions.pop().expect("non empty vector should always pop!?");
            if height <= pos.y { height = pos.y + 1; }
            if width <= pos.x { width = pos.x + 1; }
            if depth <= pos.z { depth = pos.z + 1; }
            tiles.insert(pos.to_key(), Tile::new(pos, kind));
        }

        for tile in tiles.values() {
            let (x, y, z) = (tile.pos.x, tile.pos.y, tile.pos.z);

            let mut potential_neighbours = Vec::new();
            if x >= 2 {
                potential_neighbours.push(Position::new(x - 2, y, z).to_key());
            }
            if x <= width - 3 {
                potential_neighbours.push(Position::new(x + 2, y, z).to_key());
            }
            for key in potential_neighbours {
                if tiles.contains_key(&key) {
                    tile.neighbours.borrow_mut().push(key);
                }
            }

            if z == 0 { continue; }
            let mut potential_blocking_positions = Vec::new();
            potential_blocking_positions.push(Position::new(x, y, z));
            if x >= 1 {
                potential_blocking_positions.push(Position::new(x - 1, y, z));
                if y >= 1 { potential_blocking_positions.push(Position::new(x - 1, y - 1, z)); }
                if y <= height - 2 { potential_blocking_positions.push(Position::new(x - 1, y + 1, z)); }
            }
            if x <= width - 2 {
                potential_blocking_positions.push(Position::new(x + 1, y, z));
                if y >= 1 { potential_blocking_positions.push(Position::new(x + 1, y - 1, z)); }
                if y <= height - 2 { potential_blocking_positions.push(Position::new(x + 1, y + 1, z)); }
            }
            if y >= 1 { potential_blocking_positions.push(Position::new(x, y - 1, z)); }
            if y <= height - 2 { potential_blocking_positions.push(Position::new(x, y + 1, z)); }
            for position in potential_blocking_positions {
                if let Some(other_tile) = tiles.get(&position.to_key()) {
                    other_tile.blocked_by.set(other_tile.blocked_by.get() + 1);
                    tile.blocking.borrow_mut().push(position.to_key());
                }
            }
        }

        let mut reachable_tiles = Vec::new();
        for (key, val) in tiles.iter() {
            if !val.is_blocked() { reachable_tiles.push(*key); }
        }

        Board {
            height: height,
            width: width,
            depth: depth,
            tiles: tiles,
            reachable_tiles: reachable_tiles,
        }
    }
}
