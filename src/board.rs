use std::collections::HashMap;

use tile::{Tile, Position, TileType};

// TODO: added for randomizing the board, might be a better way
//static TYPES: [Type; 7] = [Circle, Type::Bamboo, Type::Character, Type::Wind, Type::Dragon, Type::Flower, Type::Season];
//static NONARY: [Nonary; 9] = [One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
//static WINDS: [Wind; 4] = [North, South, East, West];
//static DRAGONS: [Dragon; 3] = [Red, Green, White];
//static FLOWERS: [Flower; 4] = [Plum, Orchid, Chrysanthemum, Bamboo];
//static SEASONS: [Season; 4] = [Spring, Summer, Autumn, Winter];

// TODO: refactor tiles datastructure: make it a normal vector
//       we dont care about needing to iterate over all tiles for certain operations
pub struct Board {
    height: u8,
    width: u8,
    depth: u8,
    pub tiles: HashMap<u32, Tile>,
    pub reachable_tiles: Vec<u32>,
}

impl Board {
    // TODO: implement tile randomizing
    // NOTE(Edwin): create a generation algorithm by placing valid moves on the board one by one
    
    // TODO: take in a immutable borrowed vector instead of moving it
    // NOTE(Edwin): Refactor to smaller function ->
    //              make it more readable by creating functions that name all the different
    //              operations done for board creation
    pub fn new(mut positions: Vec<(Position, TileType)>) -> Board {
        let mut tiles = HashMap::new();
        let (mut height, mut width, mut depth) = (4, 4, 0);

        while !positions.is_empty() {
            let (position, kind) = positions.pop().expect("non empty vector should always pop!?");
            if height <= position.y { height = position.y + 1; } // Why +1?
            if width <= position.x { width = position.x + 1; }
            if depth <= position.z { depth = position.z + 1; }
            tiles.insert(position.to_key(), Tile::new(position, kind));
        }

        for tile in tiles.values() {
            let (x, y, z) = (tile.position.x, tile.position.y, tile.position.z);

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
            potential_blocking_positions.push(Position::new(x, y, z - 1));
            if x >= 1 {
                potential_blocking_positions.push(Position::new(x - 1, y, z - 1));
                if y >= 1 { potential_blocking_positions.push(Position::new(x - 1, y - 1, z - 1)); }
                if y <= height - 2 { potential_blocking_positions.push(Position::new(x - 1, y + 1, z - 1)); }
            }
            if x <= width - 2 {
                potential_blocking_positions.push(Position::new(x + 1, y, z - 1));
                if y >= 1 { potential_blocking_positions.push(Position::new(x + 1, y - 1, z - 1)); }
                if y <= height - 2 { potential_blocking_positions.push(Position::new(x + 1, y + 1, z - 1)); }
            }
            if y >= 1 { potential_blocking_positions.push(Position::new(x, y - 1, z - 1)); }
            if y <= height - 2 { potential_blocking_positions.push(Position::new(x, y + 1, z - 1)); }
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

    pub fn make_move(&self, key1: u32, key2: u32) -> bool {
        if !self.tiles.get(&key1).unwrap().matches(self.tiles.get(&key2).unwrap()) {
            return false
        }
        true
        //let mut tile1, mut tile2 = self.tiles.
    }
}
