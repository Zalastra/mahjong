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
            .map(|&(x, y, z)| Rc::new(BoardPosition::new(x, y, z)))
            .collect::<Vec<_>>();

        // NOTE: Optimization possible by not checking every combination twice
        for position1 in positions.iter() {
            for position2 in positions.iter() {
                if let Some(direction) = neighbouring(position1, position2) {
                    let neighbour = Neighbour {
                        direction: direction,
                        position: Rc::downgrade(position2),
                    };
                    position1.neighbours.borrow_mut().push(neighbour)
                }
            }
        }

        set_random_start_positions(&positions);
        Positions(positions)
    }

    // NOTE: perhaps start positions could be cached so they wouldnt have to be calculated
    //       every time and could be conditionally reset.
    pub fn reset(&self) {
        for position in self.0.iter() {
            position.state.set(Empty);
        }
        set_random_start_positions(&self.0);
    }

    // NOTE: we need to update the state after the shuffling the types
    //       because our board building algorithm is currently flawed
    //
    pub fn update_states(&self) {
        for position in self.0.iter() {
            position.update_state();
        }
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

    fn traverse_positions(position: Rc<BoardPosition>,
                          visited: &mut HashSet<Rc<BoardPosition>>,
                          graph: &mut Vec<Rc<BoardPosition>>) {
        if visited.contains(&position) {
            return;
        }

        visited.insert(position.clone());
        for neighbour in position.neighbours.borrow().iter() {
            traverse_positions(neighbour.position().clone(), visited, graph)
        }
        if position.z() == 0 {
            graph.push(position);
        }
    }

    for position in positions.iter() {
        if visited_positions.contains(position) {
            continue;
        }

        ground_position_graphs.push(Vec::new());
        traverse_positions(position.clone(),
                           &mut visited_positions,
                           ground_position_graphs.last_mut().unwrap());
    }

    for graph in ground_position_graphs.iter() {
        let rows: HashSet<u8> = graph.iter()
            .map(|position| position.y())
            .fold(RefCell::new(HashSet::new()), |rows, y| {
                rows.borrow_mut().insert(y);
                rows
            })
            .into_inner();

        match rows.len() {
            0 => unreachable!(),
            1 => {
                let random_index = thread_rng().gen_range(0, graph.len());
                graph[random_index].state.set(Placable);
            }
            3 => {
                let add_random_position_from_row = |row| {
                    let count = graph.iter()
                        .filter(|&position| position.y() == row)
                        .count();

                    let random_index = thread_rng().gen_range(0, count);
                    let (_, position) = graph.iter()
                        .filter(|&position| position.y() == row)
                        .enumerate()
                        .filter(|&(index, _)| index == random_index)
                        .next()
                        .unwrap();

                    position.state.set(Placable);
                };

                let row = *rows.iter().min().unwrap();
                add_random_position_from_row(row);

                let row = *rows.iter().max().unwrap();
                add_random_position_from_row(row);
            }
            _ => unimplemented!(),
        }
    }
}

#[inline]
fn neighbouring(position1: &BoardPosition, position2: &BoardPosition) -> Option<Direction> {
    if position1.z() == position2.z() {
        if position1.y() <= position2.y() + 1 && position1.y() + 1 >= position2.y() {
            if position1.x() == position2.x() + 2 {
                Some(Left)
            } else if position1.x() + 2 == position2.x() {
                Some(Right)
            } else {
                None
            }
        } else {
            None
        }
    } else if position1.x() <= position2.x() + 1 && position1.x() + 1 >= position2.x() &&
       position1.y() <= position2.y() + 1 && position1.y() + 1 >= position2.y() {
        if position1.z() + 1 == position2.z() {
            Some(Up)
        } else if position1.z() == position2.z() + 1 {
            Some(Down)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct RawPosition {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug)]
pub struct BoardPosition {
    raw: RawPosition,
    state: Cell<State>,
    neighbours: RefCell<Vec<Neighbour>>,
}

impl BoardPosition {
    fn new(x: u8, y: u8, z: u8) -> Self {
        BoardPosition {
            raw: RawPosition { x: x, y: y, z: z },
            state: Cell::new(Empty),
            neighbours: RefCell::new(Vec::new()),
        }
    }

    #[inline]
    pub fn x(&self) -> u8 {
        self.raw.x
    }

    #[inline]
    pub fn y(&self) -> u8 {
        self.raw.y
    }

    #[inline]
    pub fn z(&self) -> u8 {
        self.raw.z
    }

    #[inline]
    pub fn is_occupied(&self) -> bool {
        match self.state.get() {
            Blocked | Unblocked => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_playable(&self) -> bool {
        match self.state.get() {
            Unblocked => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_placable(&self) -> bool {
        match self.state.get() {
            Placable => true,
            _ => false,
        }
    }

    #[inline]
    pub fn empty(&self) {
        self.state.set(Empty);
        self.update_neighbours()
    }

    #[inline]
    pub fn occupy(&self) {
        self.state.set(Unblocked);
        self.update_neighbours()
    }

    #[inline]
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

        match self.state.get() {
            // TODO: selecting placables needs to work better than this flawed algo
            Empty => {
                if all_occupied(Down) {
                    if all_occupied(Left) || all_occupied(Right) {
                        self.state.set(Placable);
                    }
                }
            }
            Blocked | Unblocked => {
                if any_occupied(Up) {
                    self.state.set(Blocked)
                } else if any_occupied(Left) && any_occupied(Right) {
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Empty,
    Placable,
    Blocked,
    Unblocked,
}

#[derive(Debug)]
pub struct Neighbour {
    direction: Direction,
    position: Weak<BoardPosition>,
}

impl Neighbour {
    pub fn position(&self) -> Rc<BoardPosition> {
        self.position.upgrade().unwrap()
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
