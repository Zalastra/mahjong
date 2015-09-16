extern crate rand;

use rand::distributions::{IndependentSample, Range};

use tiles::tile::*;
use tiles::tile_type::TileType;
use tiles::tile_type::TileType::*;

static TILE_TYPES: [TileType; 144] = [
    CircleOne, CircleOne, CircleOne, CircleOne,
    CircleTwo, CircleTwo, CircleTwo, CircleTwo,
    CircleThree, CircleThree, CircleThree, CircleThree,
    CircleFour, CircleFour, CircleFour, CircleFour,
    CircleFive, CircleFive, CircleFive, CircleFive,
    CircleSix, CircleSix, CircleSix, CircleSix,
    CircleSeven, CircleSeven, CircleSeven, CircleSeven,
    CircleEight, CircleEight, CircleEight, CircleEight,
    CircleNine, CircleNine, CircleNine, CircleNine,
    BambooOne, BambooOne, BambooOne, BambooOne,
    BambooTwo, BambooTwo, BambooTwo, BambooTwo,
    BambooThree, BambooThree, BambooThree, BambooThree,
    BambooFour, BambooFour, BambooFour, BambooFour,
    BambooFive, BambooFive, BambooFive, BambooFive,
    BambooSix, BambooSix, BambooSix, BambooSix,
    BambooSeven, BambooSeven, BambooSeven, BambooSeven,
    BambooEight, BambooEight, BambooEight, BambooEight,
    BambooNine, BambooNine, BambooNine, BambooNine,
    CharacterOne, CharacterOne, CharacterOne, CharacterOne,
    CharacterTwo, CharacterTwo, CharacterTwo, CharacterTwo,
    CharacterThree, CharacterThree, CharacterThree, CharacterThree,
    CharacterFour, CharacterFour, CharacterFour, CharacterFour,
    CharacterFive, CharacterFive, CharacterFive, CharacterFive,
    CharacterSix, CharacterSix, CharacterSix, CharacterSix,
    CharacterSeven, CharacterSeven, CharacterSeven, CharacterSeven,
    CharacterEight, CharacterEight, CharacterEight, CharacterEight,
    CharacterNine, CharacterNine, CharacterNine, CharacterNine,
    WindNorth, WindNorth, WindNorth, WindNorth,
    WindEast, WindEast, WindEast, WindEast,
    WindSouth, WindSouth, WindSouth, WindSouth,
    WindWest, WindWest, WindWest, WindWest,
    DragonRed, DragonRed, DragonRed, DragonRed,
    DragonGreen, DragonGreen, DragonGreen, DragonGreen,
    DragonWhite, DragonWhite, DragonWhite, DragonWhite,
    FlowerPlum, FlowerOrchid, FlowerChrysanthemum, FlowerBamboo,
    SeasonSpring, SeasonSummer, SeasonAutumn, SeasonWinter,
];

static POSITIONS: [u32; 144] = [
    4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26,
    72, 74, 76, 78, 80, 82, 84, 86,
    134, 136, 138, 140, 142, 144, 146, 148, 150, 152,
    196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218,
    224, 226, 250,
    260, 262, 264, 266, 268, 270, 272, 274, 276, 278, 280, 282,
    326, 328, 330, 332, 334, 336, 338, 340, 342, 344,
    392, 394, 396, 398, 400, 402, 404, 406,
    452, 454, 456, 458, 460, 462, 464, 466, 468, 470, 472, 474,

    1098, 1100, 1102, 1104, 1106, 1108,
    1162, 1164, 1166, 1168, 1170, 1172,
    1226, 1228, 1230, 1232, 1234, 1236,
    1290, 1292, 1294, 1296, 1298, 1300,
    1354, 1356, 1358, 1360, 1362, 1364,
    1418, 1420, 1422, 1424, 1426, 1428,

    2188, 2190, 2192, 2194,
    2252, 2254, 2256, 2258,
    2316, 2318, 2320, 2322,
    2380, 2382, 2384, 2386,

    3278, 3280,
    3342, 3344,

    4335,
];

pub struct TileFactory {
    remaining_tiles: Vec<Tile>,
}

impl TileFactory {
    pub fn new() -> Self {
        let remaining_tiles = POSITIONS.iter()
                                       .zip(TILE_TYPES.iter())
                                       .map(|(&position, &tile_type)| {
                                           let x = ((position % 1024) & 32) as u8;
                                           let y = ((position % 1024) / 32) as u8;
                                           let z = (position / 1024) as u8;
                                           Tile::new(TilePosition {x: x, y: y, z: z}, tile_type)
                                       })
                                       .collect();

        TileFactory {
            remaining_tiles: remaining_tiles,
        }
    }

    pub fn get_random_tile(&mut self) -> Option<Tile> {
        let tile_count = self.remaining_tiles.len();
        if tile_count == 0 { return None }
        let tile_range = Range::new(0, tile_count);
        let mut rng = rand::thread_rng();

        Some(self.remaining_tiles.remove(tile_range.ind_sample(&mut rng)))
    }
}
