use std::cell::{Cell, RefCell};

#[derive(PartialEq)]
pub enum Nonary {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Nonary {
    fn to_str(&self) -> &str {
        match *self {
            Nonary::One => "1",
            Nonary::Two => "2",
            Nonary::Three => "3",
            Nonary::Four => "4",
            Nonary::Five => "5",
            Nonary::Six => "6",
            Nonary::Seven => "7",
            Nonary::Eight => "8",
            Nonary::Nine => "9",
        }
    }
}

#[derive(PartialEq)]
pub enum Wind {
    North,
    South,
    East,
    West,
}

impl Wind {
    fn to_str(&self) -> &str {
        match *self {
            Wind::North => "N",
            Wind::South => "S",
            Wind::East => "E",
            Wind::West => "W",
        }
    }
}

#[derive(PartialEq)]
pub enum Dragon {
    Red,
    Green,
    White,
}

impl Dragon {
    fn to_str(&self) -> &str {
        match *self {
            Dragon::Red => "R",
            Dragon::Green => "G",
            Dragon::White => "W",
        }
    }
}

pub enum Flower {
    Plum,
    Orchid,
    Chrysanthemum,
    Bamboo,
}

impl Flower {
    fn to_str(&self) -> &str {
        match *self {
            Flower::Plum => "P",
            Flower::Orchid => "O",
            Flower::Chrysanthemum => "C",
            Flower::Bamboo => "B",
        }
    }
}

impl PartialEq for Flower {
    fn eq(&self, other: &Flower) -> bool {
        true
    }
}

pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    fn to_str(&self) -> &str {
        match *self {
            Season::Spring => "L",
            Season::Summer => "S",
            Season::Autumn => "A",
            Season::Winter => "W",
        }
    }
}

impl PartialEq for Season {
    fn eq(&self, other: &Season) -> bool {
        true
    }
}

#[derive(PartialEq)]
pub enum Type {
    Circle(Nonary),
    Bamboo(Nonary),
    Character(Nonary),
    Wind(Wind),
    Dragon(Dragon),
    Flower(Flower),
    Season(Season),
}

impl Type {
    fn to_str(&self) -> (&str, &str) {
        match *self {
            Type::Circle(ref n) => ("O", n.to_str()),
            Type::Bamboo(ref n) => ("B", n.to_str()),
            Type::Character(ref n) => ("C", n.to_str()),
            Type::Wind(ref w) => ("W", w.to_str()),
            Type::Dragon(ref d) => ("D", d.to_str()),
            Type::Flower(ref f) => ("F", f.to_str()),
            Type::Season(ref s) => ("S", s.to_str()),
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
    pub kind: Type,
    pub blocking: RefCell<Vec<u32>>,
    pub neighbours: RefCell<Vec<u32>>,
    pub blocked_by: Cell<u8>,
}

impl Tile {
    pub fn new(pos: Position, kind: Type) -> Tile {
        Tile {
            pos: pos,
            kind: kind,
            blocking: RefCell::new(Vec::new()),
            neighbours: RefCell::new(Vec::new()),
            blocked_by: Cell::new(0),
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.neighbours.borrow().len() <= 1 && self.blocked_by.get() == 0
    }

    pub fn print(&self) {
        let (str_a, str_b) = self.kind.to_str();
        print!("{}{}", str_a, str_b)
    }
}
