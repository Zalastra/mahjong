#![allow(dead_code)]

use std::cell::{Cell, RefCell};
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

use sdl2::render::Renderer;

use rand;
use rand::distributions::{IndependentSample, Range};

use board::tile::*;

use self::Direction::*;

// TODO: put positions in a file and read them from disk
//       use human readable format
static POSITIONS: [u32; 144] = [
    4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26,
    72, 74, 76, 78, 80, 82, 84, 86,
    134, 136, 138, 140, 142, 144, 146, 148, 150, 152,
    196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218,
    224, 226, 252,
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
    remaining_tile_types: Vec<TileType>,
    available_positions: Vec<TilePosition>,
    next_tile: Option<Tile>,
    tile_nodes: Vec<Rc<TileNode>>,
    available_nodes: Vec<Rc<TileNode>>,
}

impl TileFactory {
    pub fn new() -> Self {
        let mut tile_types = Vec::new();

        for tile_type in TileType::iter() {
            for _ in 0..tile_type.max_allowed() {
                tile_types.push(*tile_type);
            }
        }

        let tile_positions: Vec<TilePosition> =
            POSITIONS.iter()
                     .map(|&position| {
                         let x = ((position % 1024) % 32) as u8;
                         let y = ((position % 1024) / 32) as u8;
                         let z = (position / 1024) as u8;
                         TilePosition::new(x, y, z)
                     })
                     .collect();

        let nodes =
            POSITIONS.iter()
                     .map(|&position| {
                         let x = ((position % 1024) % 32) as u8;
                         let y = ((position % 1024) / 32) as u8;
                         let z = (position / 1024) as u8;
                         Rc::new(TileNode::new(TilePosition::new(x, y, z)))
                     })
                     .collect::<HashSet<Rc<TileNode>>>();

        for node in &nodes {
            for other_node in nodes.iter() {
                let (position, other_position) = (&node.position, &other_node.position);
                if position == other_position { continue; }
                if position.is_right_of(other_position) {
                    node.neighbours.borrow_mut().push(
                        TileNeighbour::new(other_node.clone(), Left));
                } else if position.is_left_of(other_position) {
                    node.neighbours.borrow_mut().push(
                        TileNeighbour::new(other_node.clone(), Right));
                } else if position.is_above(other_position) {
                    node.neighbours.borrow_mut().push(
                        TileNeighbour::new(other_node.clone(), Under));
                } else if position.is_under(other_position) {
                    node.neighbours.borrow_mut().push(
                        TileNeighbour::new(other_node.clone(), Above));
                }
            }
        }

        fn traverse_nodes<F>(node: &Rc<TileNode>, action: &F) where F: Fn(&Rc<TileNode>) {
            if node.visited.get() { return }
            node.visited.set(true);
            action(node);
            for neighbour in node.neighbours.borrow().iter() {
                traverse_nodes(&neighbour.tile, action);
            }
        }

        let mut chains = 0;
        let mut node_chains: Vec<RefCell<Vec<Rc<TileNode>>>> = vec![];
        for node in &nodes {
            if !node.visited.get() {
                node_chains.push(RefCell::new(vec![]));
                traverse_nodes(&node, &|node| {
                    node_chains[chains].borrow_mut().push(node.clone());
                });
                chains += 1;
            }
        }

        let tile_nodes = create_tile_nodes();
        let starting_tiles = get_starting_tiles(&tile_nodes);

        TileFactory {
            remaining_tile_types: tile_types,
            available_positions: tile_positions,
            next_tile: None,
            tile_nodes: tile_nodes,
            available_nodes: starting_tiles,
        }
    }

    pub fn get_tile(&mut self, renderer: &Renderer) -> Option<Tile> {
        let opt_tile = mem::replace(&mut self.next_tile, None);
        match opt_tile {
            Some(tile) => Some(tile),
            None => {
                if self.available_nodes.is_empty() { return None; }

                let mut rng = rand::thread_rng();

                let tile_count = self.available_nodes.len();

                let random_index = Range::new(0, tile_count).ind_sample(&mut rng);
                let node1 = self.available_nodes.swap_remove(random_index);

                let random_index = Range::new(0, tile_count - 1).ind_sample(&mut rng);
                let node2 = self.available_nodes.swap_remove(random_index);

                let random_index = Range::new(0, tile_count / 2).ind_sample(&mut rng) * 2;
                let tile_type1 = self.remaining_tile_types.remove(random_index);
                let tile_type2 = self.remaining_tile_types.remove(random_index);

                self.next_tile = Some(Tile::new(node1.position.clone(), tile_type1, renderer));

                Some(Tile::new(node2.position.clone(), tile_type2, renderer))
                /*
                let tile_count = self.remaining_tile_types.len();
                if self.remaining_tile_types.is_empty() { return None }

                let mut rng = rand::thread_rng();
                let random_index = Range::new(0, tile_count / 2).ind_sample(&mut rng) * 2;
                let tile_type1 = self.remaining_tile_types.remove(random_index);
                let tile_type2 = self.remaining_tile_types.remove(random_index);

                let random_index = Range::new(0, tile_count).ind_sample(&mut rng);
                let tile_position1 = self.available_positions.remove(random_index);
                let random_index = Range::new(0, tile_count-1).ind_sample(&mut rng);
                let tile_position2 = self.available_positions.remove(random_index);

                self.next_tile = Some(Tile::new(tile_position2, tile_type2, renderer));
                Some(Tile::new(tile_position1, tile_type1, renderer))
                */
            }
        }
    }
}

fn get_starting_tiles(nodes: &Vec<Rc<TileNode>>) -> Vec<Rc<TileNode>> {
    let mut ground_node_graphs: Vec<Vec<Rc<TileNode>>> = vec![];
    let mut visited_nodes: HashSet<Rc<TileNode>> = HashSet::new();
    for node in nodes {
        if !visited_nodes.contains(node) {
            ground_node_graphs.push(vec![]);

            fn traverse_nodes(node: &Rc<TileNode>, visited: &mut HashSet<Rc<TileNode>>, graph: &mut Vec<Rc<TileNode>>) {
                if visited.contains(node) { return }
                visited.insert(node.clone());
                if node.position.z() > 0 { return }
                graph.push(node.clone());
                for neighbour in node.neighbours.borrow().iter() {
                    traverse_nodes(&neighbour.tile, visited, graph);
                }
            }
            traverse_nodes(&node, &mut visited_nodes, &mut ground_node_graphs.last_mut().unwrap());
        }
    }

    let mut rng = rand::thread_rng();
    let mut starting_positions = Vec::<Rc<TileNode>>::new();
    for graph in ground_node_graphs.iter() {
        let rows: HashSet<u8> = graph
            .iter()
            .map(|node| {
                node.position.y()
            })
            .fold(RefCell::new(HashSet::new()), |rows, y| {
                rows.borrow_mut().insert(y);
                rows
            })
            .into_inner();

        if rows.len() == 1 {
            let random_index = Range::new(0, graph.len()).ind_sample(&mut rng);
            starting_positions.push(graph[random_index].clone());
        } else {
            // TODO: currently assumes row count to be 3 if not 1, make the code not depend on this to support different board setups

            let mut top_row_iter = graph.iter()
                .filter(|&node| node.position.y() == *rows.iter().min().unwrap());
            let random_index = Range::new(0, top_row_iter.by_ref().count()).ind_sample(&mut rng);
            let (_, node) = top_row_iter.fold((0, None), |(index, _), node| {
                let node = if index == random_index {
                    Some(node.clone())
                } else {
                    None
                };
                (index + 1, node)
            });
            starting_positions.push(node.unwrap());

            let mut bottom_row_iter = graph.iter()
                .filter(|&node| node.position.y() == *rows.iter().max().unwrap());
            let random_index = Range::new(0, bottom_row_iter.by_ref().count()).ind_sample(&mut rng);
            let (_, node) = bottom_row_iter.fold((0, None), |(index, _), node| {
                let node = if index == random_index {
                    Some(node.clone())
                } else {
                    None
                };
                (index + 1, node)
            });
            starting_positions.push(node.unwrap());
        }
    }

    starting_positions
}

fn create_tile_nodes() -> Vec<Rc<TileNode>> {
    let nodes =
        POSITIONS.iter()
                 .map(|&position| {
                     let x = ((position % 1024) % 32) as u8;
                     let y = ((position % 1024) / 32) as u8;
                     let z = (position / 1024) as u8;
                     Rc::new(TileNode::new(TilePosition::new(x, y, z)))
                 })
                 .collect::<Vec<Rc<TileNode>>>();

    for node in &nodes {
        for other_node in &nodes {
            let (position, other_position) = (&node.position, &other_node.position);
            if position.is_right_of(other_position) {
                node.neighbours.borrow_mut().push(
                    TileNeighbour::new(other_node.clone(), Left));
            } else if position.is_left_of(other_position) {
                node.neighbours.borrow_mut().push(
                    TileNeighbour::new(other_node.clone(), Right));
            } else if position.is_above(other_position) {
                node.neighbours.borrow_mut().push(
                    TileNeighbour::new(other_node.clone(), Under));
            } else if position.is_under(other_position) {
                node.neighbours.borrow_mut().push(
                    TileNeighbour::new(other_node.clone(), Above));
            }
        }
    }
    nodes
}

enum Direction {
    Left, Right, Under, Above,
}

struct TileNeighbour {
    pub tile: Rc<TileNode>,
    pub side: Direction,
}

impl TileNeighbour {
    fn new(tile: Rc<TileNode>, side: Direction) -> Self {
        TileNeighbour {
            tile: tile,
            side: side,
        }
    }
}

/*impl Deref for TileNeighbour {
    type Target = Rc<TileNode>;

    fn deref<'a>(&'a self) -> &'a Rc<TileNode> {
        &self.tile
    }
}*/

struct TileNode {
    pub position: TilePosition,
    pub tile_type: Cell<Option<TileType>>,
    pub neighbours: RefCell<Vec<TileNeighbour>>,
    pub visited: Cell<bool>,
}

impl TileNode {
    fn new(position: TilePosition) -> Self {
        TileNode {
            position: position,
            tile_type: Cell::new(None),
            neighbours: RefCell::new(Vec::new()),
            visited: Cell::new(false),
        }
    }
}

impl PartialEq for TileNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for TileNode { }

impl Hash for TileNode {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut hash = (self.position.x() as u32) << 16;
        hash |= (self.position.y() as u32) << 8;
        hash |= self.position.z() as u32;
        state.write_u32(hash);
    }
}
