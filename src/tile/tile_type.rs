use std::fmt;

use self::TileType::*;

iterable_enum!{
    TileType {
        #![derive(PartialEq, Clone, Copy)]

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
}

impl TileType {
    pub fn max_allowed(&self) -> u8 {
        match *self {
            FlowerPlum | FlowerOrchid |
            FlowerChrysanthemum | FlowerBamboo |
            SeasonSpring | SeasonSummer |
            SeasonAutumn | SeasonWinter => 1,
            _ => 4,
        }
    }

    pub fn matches(&self, other: &TileType) -> bool {
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
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
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
        };
        write!(f, "{}", s)
    }
}
