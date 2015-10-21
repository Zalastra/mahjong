use std::fmt;

use self::TileType::*;

macro_rules! iterable_enum {
    (
        $enum_name:ident {
            #![$meta_decl:meta]
            $( $variant:ident, )+
        }
    ) => (
        #[$meta_decl]
        pub enum $enum_name { $( $variant, )* }

        impl $enum_name {
            pub fn iter() -> ::std::slice::Iter<'static, $enum_name> {
                const ENUM_VARIANTS: &'static [$enum_name] = &[ $( $variant, )* ];
                ENUM_VARIANTS.iter()
            }

            pub fn filename_texture(&self) -> String {
                match *self {
                    $( $variant => String::from(stringify!($variant)) + ".png",)*
                }
            }
        }
    )
}

iterable_enum!{
    TileType {
        #![derive(PartialEq, Clone, Copy, Eq, Hash)]

        BallOne, BallTwo, BallThree, BallFour, BallFive,
        BallSix, BallSeven, BallEight, BallNine,
        BambooOne, BambooTwo, BambooThree, BambooFour, BambooFive,
        BambooSix, BambooSeven, BambooEight, BambooNine,
        CharacterOne, CharacterTwo, CharacterThree, CharacterFour, CharacterFive,
        CharacterSix, CharacterSeven, CharacterEight, CharacterNine,
        WindNorth, WindEast, WindSouth, WindWest,
        DragonRed, DragonGreen, DragonWhite,
        FlowerPlum, FlowerOrchid, FlowerBamboo, FlowerChrysanthemum,
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

// TODO: should be removed if we dont need console printing for debugging
impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            BallOne => "o1",
            BallTwo => "o2",
            BallThree => "o3",
            BallFour => "o4",
            BallFive => "o5",
            BallSix => "o6",
            BallSeven => "o7",
            BallEight => "o8",
            BallNine => "o9",
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
