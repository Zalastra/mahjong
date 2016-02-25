use std::cell::{Ref, RefCell, Cell};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::slice::Iter;

use self::Direction::*;

pub struct Positions {
    positions: Vec<Rc<BoardPosition>>
}

impl Positions {
    pub fn new(positions: &[u32; 144]) -> Self {
        let positions_vec =
            positions.iter()
                     .map(|&position| {
                         let x = ((position % 1024) % 32) as u8;
                         let y = ((position % 1024) / 32) as u8;
                         let z = (position / 1024) as u8;
                         BoardPosition::new(x, y, z)
                     })
                     .collect::<Vec<_>>();

        for position1 in positions_vec.iter() {
            for position2 in positions_vec.iter() {
                if position1 == position2 { continue; }
                BoardPosition::test_for_neighbouring(position1.clone(), position2.clone())
            }
        }
        Positions {
            positions: positions_vec
        }
    }

    pub fn iter(&self) -> Iter<Rc<BoardPosition>> {
        self.positions.iter()
    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: [u8; 2],
    pub y: [u8; 2],
    pub z: u8,
}

impl Position {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Position {
            x: [x, x + 1],
            y: [y, y + 1],
            z: z,
        }
    }

    fn from(position: &Position, direction: Direction) -> Self {
        match direction {
            Up => Position {
                x: [position.x[0], position.x[1]],
                y: [position.y[0], position.y[1]],
                z: position.z + 1,
            },
            Right => Position {
                x: [position.x[0] + 1, position.x[1] + 1],
                y: [position.y[0], position.y[1]],
                z: position.z,
            },
            _ => panic!("dont call from with Down or Left")
        }
    }
}

pub struct NeighbourPosition {
    direction: Direction,
    neighbour: Weak<BoardPosition>,
}

impl NeighbourPosition {
    fn new(position: Rc<BoardPosition>, direction: Direction) -> Self {
        NeighbourPosition {
            direction: direction,
            neighbour: Rc::downgrade(&position),
        }
    }

    pub fn position(&self) -> Rc<BoardPosition> {
        self.neighbour.upgrade().expect("all board positions should exist at the same time")
    }
}

pub struct BoardPosition {
    position: Position,
    neighbours: RefCell<Vec<NeighbourPosition>>,
    empty: Cell<bool>,
}

impl BoardPosition {
    fn new(x: u8, y: u8, z: u8) -> Rc<Self> {
        Rc::new(BoardPosition {
            position: Position::new(x, y, z),
            neighbours: RefCell::new(vec![]),
            empty: Cell::new(false),
        })
    }

    pub fn x(&self) -> u8 {
        self.position.x[0]
    }

    pub fn y(&self) -> u8 {
        self.position.y[0]
    }

    pub fn z(&self) -> u8 {
        self.position.z
    }

    pub fn is_empty(&self) -> bool {
        self.empty.get()
    }

    pub fn empty(&self, empty: bool) {
        self.empty.set(empty);
    }

    pub fn is_reachable(&self) -> bool {
        macro_rules! neighbours {
            () => {
                self.neighbours().iter().filter(|neighbour| !neighbour.position().is_empty())
            }
        }
        if neighbours!().any(|neighbour| neighbour.direction == Up) {
            false
        }
        else if neighbours!().any(|neighbour| neighbour.direction == Left)
            && neighbours!().any(|neighbour| neighbour.direction == Right) {
            false
        }
        else {
            true
        }
    }

    pub fn neighbours(&self) -> Ref<Vec<NeighbourPosition>> {
        self.neighbours.borrow()
    }

    pub fn test_for_neighbouring(position1: Rc<BoardPosition>, position2: Rc<BoardPosition>) {
        if position1.is_neighbour_with(position2.clone()) { return }

        if position1.overlaps_with(Position::from(&position2.position, Up)) {
            position1.add_neighbour(NeighbourPosition::new(position2.clone(), Down));
            position2.add_neighbour(NeighbourPosition::new(position1.clone(), Up));
        } else if position1.overlaps_with(Position::from(&position2.position, Right)) {
            position1.add_neighbour(NeighbourPosition::new(position2.clone(), Left));
            position2.add_neighbour(NeighbourPosition::new(position1.clone(), Right));
        } else if position2.overlaps_with(Position::from(&position1.position, Up)) {
            position1.add_neighbour(NeighbourPosition::new(position2.clone(), Up));
            position2.add_neighbour(NeighbourPosition::new(position1.clone(), Down));
        } else if position2.overlaps_with(Position::from(&position1.position, Right)) {
            position1.add_neighbour(NeighbourPosition::new(position2.clone(), Right));
            position2.add_neighbour(NeighbourPosition::new(position1.clone(), Left));
        }
    }

    fn add_neighbour(&self, neighbour: NeighbourPosition) {
        self.neighbours.borrow_mut().push(neighbour);
    }

    fn is_neighbour_with(&self, position: Rc<BoardPosition>) -> bool {
        self.neighbours
            .borrow()
            .iter()
            .map(|neighbour| neighbour.position())
            .any(|neighbour| neighbour == position)
    }

    fn overlaps_with(&self, position: Position) -> bool {
        if !(self.position.x[0] == position.x[0] || self.position.x[0] == position.x[1] ||
            self.position.x[1] == position.x[0] || self.position.x[1] == position.x[1]) {
            false
        } else if !(self.position.y[0] == position.y[0] || self.position.y[0] == position.y[1] ||
            self.position.y[1] == position.y[0] || self.position.y[1] == position.y[1]) {
            false
        } else if self.position.z == position.z {
            true
        } else {
            false
        }
    }
}

impl PartialEq for BoardPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for BoardPosition { }

impl Hash for BoardPosition {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut hash = (self.position.x[0] as u32) << 16;
        hash |= (self.position.y[0] as u32) << 8;
        hash |= self.position.z as u32;
        state.write_u32(hash);
    }
}
