use board::Board;

use std::cell::{Cell, RefCell};

use self::TileType::*;

#[derive(PartialEq)]
pub enum TileType {
    // Are these different circles or amount of circles?
    // Like a tile with five circles or the fifth type of circle?
    // If the former, rename to something like FiveCircles or CircleTimesFive
    // If the latter, improve naming, think up something else than five

    // Consider a hierarchy: Circle/Bamboo/Character/Wind/Dragon/Flower/Season, each
    // with 4 or 9 subtypes. After looking up these are called suits, get this
    // concept in your code!
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
}

impl TileType {
    fn matches(&self, other: &TileType) -> bool {
        match *self {
            FlowerPlum | FlowerOrchid |
            FlowerChrysanthemum | FlowerBamboo => {
                match *other {
                    FlowerPlum | FlowerOrchid |
                    FlowerChrysanthemum | FlowerBamboo => true,
                    _ => false,
                }
            },
            SeasonSpring | SeasonSummer |
            SeasonAutumn | SeasonWinter => {
                match *other {
                    SeasonSpring | SeasonSummer |
                    SeasonAutumn | SeasonWinter => true,
                    _ => false,
                }
            },
            _ => return *self == *other,
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            CircleOne => "o1",
            CircleTwo => "o2",
            CircleThree => "o3",
            CircleFour => "o4",
            CircleFive => "o5",
            CircleSix => "o6",
            CircleSeven => "o7",
            CircleEight => "o8",
            CircleNine => "o9",
            BambooOne => "b1",
            BambooTwo => "b2",
            BambooThree => "b3",
            BambooFour => "b4",
            BambooFive => "b5",
            BambooSix => "b6",
            BambooSeven => "b7",
            BambooEight => "b8",
            BambooNine => "b9",
            CharacterOne => "c1",
            CharacterTwo => "c2",
            CharacterThree => "c3",
            CharacterFour => "c4",
            CharacterFive => "c5",
            CharacterSix => "c6",
            CharacterSeven => "c7",
            CharacterEight => "c8",
            CharacterNine => "c9",
            WindNorth => "wN",
            WindEast => "wE",
            WindSouth => "wS",
            WindWest => "wW",
            DragonRed => "dR",
            DragonGreen => "dG",
            DragonWhite => "dW",
            FlowerPlum => "fP",
            FlowerOrchid => "fO",
            FlowerChrysanthemum => "fC",
            FlowerBamboo => "fB",
            SeasonSpring => "sL",
            SeasonSummer => "sS",
            SeasonAutumn => "sA",
            SeasonWinter => "sW",
        }
    }
}

// NOTE might need to moved to a different file based on how we implement the board/tile structure
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn new(x: u8, y: u8, z: u8) -> Position {
        Position {x: x, y: y, z: z, }
    }

    pub fn to_key(&self) -> u32 {
        (self.x as u32) << 16 | (self.y as u32) << 8 | self.z as u32
    }
}

pub struct Tile {
    pub position: Position,
    pub kind: TileType,
    pub blocking: RefCell<Vec<u32>>,
    pub neighbours: RefCell<Vec<u32>>,
    pub blocked_by: Cell<u8>,
}

impl Tile {
    pub fn new(position: Position, kind: TileType) -> Tile {
        Tile {
            position: position,
            kind: kind,
            blocking: RefCell::new(Vec::new()),
            neighbours: RefCell::new(Vec::new()),
            blocked_by: Cell::new(0),
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.neighbours.borrow().len() == 2 || self.blocked_by.get() > 0
    }

    pub fn matches(&self, other: &Tile) -> bool {
        self.kind.matches(&other.kind)
    }

    pub fn print(&self) {
        print!("{}|{}", self.position.z, self.kind.to_str())
    }
}
