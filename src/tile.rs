use std::cell::{Cell, RefCell};

#[derive(PartialEq)]
pub enum TileType {
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
            TileType::FlowerPlum | TileType::FlowerOrchid |
            TileType::FlowerChrysanthemum | TileType::FlowerBamboo => match *other {
                TileType::FlowerPlum | TileType::FlowerOrchid |
                TileType::FlowerChrysanthemum | TileType::FlowerBamboo => true,
                _ => false,
            },
            TileType::SeasonSpring | TileType::SeasonSummer |
            TileType::SeasonAutumn | TileType::SeasonWinter => match *other {
                TileType::SeasonSpring | TileType::SeasonSummer |
                TileType::SeasonAutumn | TileType::SeasonWinter => true,
                _ => false,
            },
            _ => return *self == *other,
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            TileType::CircleOne => "o1",
            TileType::CircleTwo => "o2",
            TileType::CircleThree => "o3",
            TileType::CircleFour => "o4",
            TileType::CircleFive => "o5",
            TileType::CircleSix => "o6",
            TileType::CircleSeven => "o7",
            TileType::CircleEight => "o8",
            TileType::CircleNine => "o9",
            TileType::BambooOne => "b1",
            TileType::BambooTwo => "b2",
            TileType::BambooThree => "b3",
            TileType::BambooFour => "b4",
            TileType::BambooFive => "b5",
            TileType::BambooSix => "b6",
            TileType::BambooSeven => "b7",
            TileType::BambooEight => "b8",
            TileType::BambooNine => "b9",
            TileType::CharacterOne => "c1",
            TileType::CharacterTwo => "c2",
            TileType::CharacterThree => "c3",
            TileType::CharacterFour => "c4",
            TileType::CharacterFive => "c5",
            TileType::CharacterSix => "c6",
            TileType::CharacterSeven => "c7",
            TileType::CharacterEight => "c8",
            TileType::CharacterNine => "c9",
            TileType::WindNorth => "wN",
            TileType::WindEast => "wE",
            TileType::WindSouth => "wS",
            TileType::WindWest => "wW",
            TileType::DragonRed => "dR",
            TileType::DragonGreen => "dG",
            TileType::DragonWhite => "dW",
            TileType::FlowerPlum => "fP",
            TileType::FlowerOrchid => "fO",
            TileType::FlowerChrysanthemum => "fC",
            TileType::FlowerBamboo => "fB",
            TileType::SeasonSpring => "sL",
            TileType::SeasonSummer => "sS",
            TileType::SeasonAutumn => "sA",
            TileType::SeasonWinter => "sW",
        }
    }
}

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
    pub pos: Position,
    pub kind: TileType,
    pub blocking: RefCell<Vec<u32>>,
    pub neighbours: RefCell<Vec<u32>>,
    pub blocked_by: Cell<u8>,
}

impl Tile {
    pub fn new(pos: Position, kind: TileType) -> Tile {
        Tile {
            pos: pos,
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
        print!("{}|{}", self.pos.z, self.kind.to_str())
    }
}
