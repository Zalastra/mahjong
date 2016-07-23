use std::cell::{RefCell, Cell};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::ops::{Deref, DerefMut};

use rand::{thread_rng, Rng};

use self::Direction::*;
use self::State::*;

#[derive(Debug)]
pub struct Positions(Vec<Rc<BoardPosition>>);

impl Positions {
    pub fn new(raw_positions: &[(u8, u8, u8); 144]) -> Self {
        let positions = raw_positions.iter()
            .map(|&(x, y, z)| BoardPosition::new(x, y, z))
            .collect::<Vec<_>>();

        // NOTE: Optimization possible by not checking every combination twice
        for position1 in &positions {
            for position2 in &positions {
                if let Some(direction) = neighbouring(position1.raw, position2.raw) {
                    let neighbour = Neighbour {
                        direction: direction,
                        position: Rc::downgrade(position2),
                    };
                    position1.neighbours.borrow_mut().push(neighbour);
                }
            }
        }

        set_random_start_positions(&positions);
        Positions(positions)
    }

    // NOTE: perhaps start positions could be cached so they wouldnt have to be calculated
    //       every time and could be conditionally reset.
    pub fn reset(&self) {
        for position in &self.0 {
            position.state.set(Empty);
        }
        set_random_start_positions(&self.0);
    }
}

impl Deref for Positions {
    type Target = [Rc<BoardPosition>];

    fn deref(&self) -> &[Rc<BoardPosition>] {
        self.0.as_slice()
    }
}

impl DerefMut for Positions {
    fn deref_mut(&mut self) -> &mut [Rc<BoardPosition>] {
        self.0.as_mut_slice()
    }
}

fn set_random_start_positions(positions: &[Rc<BoardPosition>]) {
    let mut ground_position_graphs: Vec<Vec<Rc<BoardPosition>>> = Vec::new();
    let mut visited_positions: HashSet<Rc<BoardPosition>> = HashSet::default();

    for position in positions.iter() {
        if visited_positions.contains(position) {
            continue;
        }

        let mut graph = Vec::new();
        let mut neighbours: Vec<Neighbour> = position.neighbours.borrow().clone();

        while !neighbours.is_empty() {
            let position = neighbours.pop().unwrap().position();
            if !visited_positions.contains(&position) {
                if position.raw.z == 0 {
                    graph.push(position.clone());
                }
                neighbours.append(&mut position.neighbours.borrow_mut().clone());
                visited_positions.insert(position.clone());
            }
        }

        ground_position_graphs.push(graph);
    }

    for graph in &ground_position_graphs {
        let mut available_positions = graph.clone();

        while !available_positions.is_empty() {
            let random_index = thread_rng().gen_range(0, available_positions.len());
            let position = available_positions[random_index].clone();
            position.state.set(Placable);

            position.set_recursively_neighbours_unplaceble(Left);
            position.set_recursively_neighbours_unplaceble(Right);

            let mut new_available_positions = Vec::new();
            for position in &available_positions {
                if position.state.get() == Empty {
                    new_available_positions.push(position.clone());
                }
            }
            available_positions = new_available_positions;
        }
    }
}

fn neighbouring(position1: RawPosition, position2: RawPosition) -> Option<Direction> {
    if position1.z == position2.z {
        if position1.y <= position2.y + 1 && position1.y + 1 >= position2.y {
            if position1.x == position2.x + 2 {
                Some(Left)
            } else if position1.x + 2 == position2.x {
                Some(Right)
            } else {
                None
            }
        } else {
            None
        }
    } else if position1.x <= position2.x + 1 && position1.x + 1 >= position2.x &&
       position1.y <= position2.y + 1 && position1.y + 1 >= position2.y {
        if position1.z + 1 == position2.z {
            Some(Up)
        } else if position1.z == position2.z + 1 {
            Some(Down)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug)]
pub struct BoardPosition {
    raw: RawPosition,
    state: Cell<State>,
    neighbours: RefCell<Vec<Neighbour>>,
}

impl BoardPosition {
    fn new(x: u8, y: u8, z: u8) -> Rc<BoardPosition> {
        Rc::new(BoardPosition {
            raw: RawPosition { x: x, y: y, z: z },
            state: Cell::new(Empty),
            neighbours: RefCell::new(Vec::new()),
        })
    }

    pub fn is_occupied(&self) -> bool {
        match self.state.get() {
            Blocked | Unblocked => true,
            _ => false,
        }
    }

    pub fn is_playable(&self) -> bool {
        match self.state.get() {
            Unblocked => true,
            _ => false,
        }
    }

    pub fn is_placable(&self) -> bool {
        match self.state.get() {
            Placable => true,
            _ => false,
        }
    }

    pub fn empty(&self) {
        self.state.set(Empty);
        self.update_neighbours()
    }

    pub fn occupy(&self) {
        self.state.set(Unblocked);
        self.update_neighbours()
    }

    fn set_recursively_neighbours_unplaceble(&self, direction: Direction) {
        let mut neighbours = self.neighbours.borrow().clone();
        while !neighbours.is_empty() {
            let neighbour = neighbours.pop().unwrap();
            if neighbour.direction == direction {
                let position = neighbour.position();
                position.state.set(Unplacable);
                neighbours.append(&mut position.neighbours.borrow().clone());
            }
        }
    }

    fn update_neighbours(&self) {
        for neighbour in self.neighbours.borrow().iter() {
            neighbour.position().update_state();
        }
    }

    fn update_state(&self) {
        let any_occupied = |direction| {
            self.neighbours
                .borrow()
                .iter()
                .filter(|n| n.position().is_occupied())
                .any(|n| n.direction == direction)
        };

        let all_occupied = |direction| {
            self.neighbours
                .borrow()
                .iter()
                .filter(|n| n.direction == direction)
                .all(|n| n.position().is_occupied())
        };

        let all_empty = |direction| {
            let mut neighbours = self.neighbours.borrow().clone();

            while !neighbours.is_empty() {
                let neighbour = neighbours.pop().unwrap();
                if neighbour.direction == direction {
                    let position = neighbour.position();
                    if position.state.get() != Empty {
                        return false;
                    }
                    neighbours.append(&mut position.neighbours.borrow().clone());
                }
            }
            true
        };

        match self.state.get() {
            Empty | Unplacable => {
                if all_occupied(Down) &&
                   ((all_empty(Left) && all_empty(Right)) ||
                    ((any_occupied(Left) && all_occupied(Left)) ||
                     (any_occupied(Right) && all_occupied(Right)))) {

                    self.state.set(Placable);
                }
            }
            Blocked | Unblocked => {
                if any_occupied(Up) || (any_occupied(Left) && any_occupied(Right)) {
                    self.state.set(Blocked)
                } else {
                    self.state.set(Unblocked)
                }
            }
            _ => (),
        }
    }
}

impl PartialEq for BoardPosition {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for BoardPosition {}

impl Hash for BoardPosition {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.raw.hash(state);
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct RawPosition {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Clone, Debug)]
struct Neighbour {
    direction: Direction,
    position: Weak<BoardPosition>,
}

impl Neighbour {
    pub fn position(&self) -> Rc<BoardPosition> {
        self.position.upgrade().unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Empty,
    Unplacable,
    Placable,
    Blocked,
    Unblocked,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
