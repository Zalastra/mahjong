extern crate rand;

use rand::distributions::{IndependentSample, Range};

use tile::{Tile, TilePosition, TileType};
use tile::TileType::*;

// TODO: added for randomizing the board, might be a better way
static TILE_TYPES: [TileType; 42] = [
    CircleOne, CircleTwo, CircleThree, CircleFour, CircleFive,
    CircleSix, CircleSeven, CircleEight, CircleNine,
    BambooOne, BambooTwo, BambooThree, BambooFour, BambooFive,
    BambooSix, BambooSeven, BambooEight, BambooNine,
    CharacterOne, CharacterTwo, CharacterThree, CharacterFour, CharacterFive,
    CharacterSix, CharacterSeven, CharacterEight, CharacterNine,
    WindNorth, WindEast, WindSouth, WindWest,
    DragonRed, DragonGreen, DragonWhite,
    FlowerPlum, FlowerOrchid, FlowerChrysanthemum, FlowerBamboo,
    SeasonSpring, SeasonSummer, SeasonAutumn, SeasonWinter,
];

// TODO: refactor tiles datastructure: make it a normal vector
//       we dont care about needing to iterate over all tiles for certain operations
pub struct Board {
    pub tiles: Vec<Tile>,
    //pub reachable_tiles: Vec<u32>,
}

impl Board {
    // TODO: implement tile randomizing
    // NOTE(Edwin): create a generation algorithm by placing valid moves on the board one by one

    pub fn new(positions: &Vec<TilePosition>) -> Board {
        let mut tiles = Vec::new();
        let mut rng = rand::thread_rng();
        let tile_types_range = Range::new(0, 42);

        for position in positions {
            let tile_type = TILE_TYPES[tile_types_range.ind_sample(&mut rng)];
            tiles.push(Tile::new(position.clone(), tile_type));
        }


        Board {
            tiles: tiles,
            //reachable_tiles: reachable_tiles,
        }
    }
}
