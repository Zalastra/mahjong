use self::TileType::*;

macro_rules! iterable_enum {
    (
        #[$meta_decl:meta]
        $enum_name:ident {
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

            pub fn filename_texture(&self) -> &'static str {
                match *self {
                    $( $variant => concat!(stringify!($variant), ".png"),)*
                }
            }
        }
    )
}

iterable_enum!{
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    TileType {
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
            FlowerPlum | FlowerOrchid | FlowerChrysanthemum | FlowerBamboo | SeasonSpring |
            SeasonSummer | SeasonAutumn | SeasonWinter => 1,
            _ => 4,
        }
    }

    pub fn matches(&self, other: &TileType) -> bool {
        match *self {
            FlowerPlum | FlowerOrchid | FlowerChrysanthemum | FlowerBamboo => {
                match *other {
                    FlowerPlum | FlowerOrchid | FlowerChrysanthemum | FlowerBamboo => true,
                    _ => false,
                }
            }
            SeasonSpring | SeasonSummer | SeasonAutumn | SeasonWinter => {
                match *other {
                    SeasonSpring | SeasonSummer | SeasonAutumn | SeasonWinter => true,
                    _ => false,
                }
            }
            _ => *self == *other,
        }
    }
}
