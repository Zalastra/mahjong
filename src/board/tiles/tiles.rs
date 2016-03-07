use std::cell::RefCell;
use std::collections::hash_set::HashSet;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::slice::Iter;
use std::time::UNIX_EPOCH;

use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

use sdl2::render::{Renderer};

use super::position::{BoardPosition, Positions};
use super::tile::Tile;
use super::tile_type::TileType;

type Test = Fn(&Tile) -> bool;

pub struct Tiles {
    tiles: Vec<Tile>
}

impl Tiles {
    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }

    pub fn iter_playable<'a>(&'a self) -> Box<Iterator<Item=(usize, &'a Tile)> + 'a> {
        Box::new(self.tiles.iter().enumerate().filter(|&(_, tile)| tile.is_playable()))
    }
}

impl Index<usize> for Tiles {
    type Output = Tile;

    fn index(&self, _index: usize) -> &Tile {
        &self.tiles[_index]
    }
}

impl IndexMut<usize> for Tiles {
    fn index_mut(&mut self, _index: usize) -> &mut Tile {
        &mut self.tiles[_index]
    }
}

pub struct TilesBuilder<'a> {
    all_positions: Positions,
    remaining_tile_types: Vec<TileType>,
    available_positions: Vec<Rc<BoardPosition>>,
    used_positions: Vec<Rc<BoardPosition>>,
    renderer: &'a Renderer<'a>,
}

impl<'a> TilesBuilder<'a> {
    pub fn new(positions: &'a [u32; 144], renderer: &'a Renderer) -> TilesBuilder<'a> {
        let all_positions = Positions::new(positions);
        let starting_positions = get_starting_positions(&all_positions);
        TilesBuilder {
            all_positions: all_positions,
            remaining_tile_types: get_tile_types(),
            available_positions: starting_positions,
            used_positions: Vec::new(),
            renderer: renderer,
        }
    }

    pub fn build(mut self) -> Tiles {
        let mut tiles = Vec::new();
        let mut valid = false;
        while !valid {
            valid = true;

            while let Some(tiles_result) = self.get_tile_set() {
                if let Ok((tile1, tile2)) = tiles_result {
                    tiles.push(tile1);
                    tiles.push(tile2);
                } else {
                    valid = false;
                    break;
                }
            }

            // if board isn't valid we clear the vec and try again
            if !valid {
                tiles.clear();
                self.reset();
            }
        }

        tiles.sort_by(|tile1, tile2| {
            use std::cmp::Ordering::*;
            if tile1.z() < tile2.z() { Less }
            else if tile1.z() > tile2.z() { Greater }
            else if tile1.x() > tile2.x() { Less }
            else if tile1.x() < tile2.x() { Greater }
            else if tile1.y() < tile2.y() { Less }
            else { Greater }
        });

        Tiles {
            tiles: tiles
        }
    }

    fn reset(&mut self) {
        self.remaining_tile_types = get_tile_types();
        self.available_positions = get_starting_positions(&self.all_positions);
        self.used_positions = Vec::new();
    }

    fn get_tile_set(&mut self) -> Option<Result<(Tile, Tile), ()>> {
        if self.available_positions.is_empty() { return None; }

        let mut rng = StdRng::from_seed(&[UNIX_EPOCH.elapsed().unwrap().as_secs() as usize]);

        let random_index = Range::new(0, self.remaining_tile_types.len() / 2).ind_sample(&mut rng) * 2;
        let tile_type1 = self.remaining_tile_types.remove(random_index);
        let tile_type2 = self.remaining_tile_types.remove(random_index);

        let random_index = Range::new(0, self.available_positions.len()).ind_sample(&mut rng);
        let node1 = self.available_positions.swap_remove(random_index);
        self.used_positions.push(node1.clone());

        if self.available_positions.is_empty() { return Some(Err(())); };
        let random_index = Range::new(0, self.available_positions.len()).ind_sample(&mut rng);
        let node2 = self.available_positions.swap_remove(random_index);
        self.used_positions.push(node2.clone());

        for neighbour in node1.neighbours().iter() {
            let node = &neighbour.position();
            if !self.available_positions.contains(node) && !self.used_positions.contains(node) {
                self.available_positions.push(node.clone());
            }
        }
        for neighbour in node2.neighbours().iter() {
            let node = &neighbour.position();
            if !self.available_positions.contains(node) && !self.used_positions.contains(node) {
                self.available_positions.push(node.clone());
            }
        }

        let tile1 = Tile::new(node1, tile_type1, self.renderer);
        let tile2 = Tile::new(node2, tile_type2, self.renderer);
        Some(Ok((tile1, tile2)))
    }
}

fn get_tile_types() -> Vec<TileType> {
    let mut tile_types = Vec::new();
    for tile_type in TileType::iter() {
        for _ in 0..tile_type.max_allowed() {
            tile_types.push(*tile_type);
        }
    }
    tile_types
}

fn get_starting_positions(positions: &Positions) -> Vec<Rc<BoardPosition>> {
    let ground_position_graphs = get_ground_positions(positions);

    let mut rng = StdRng::from_seed(&[UNIX_EPOCH.elapsed().unwrap().as_secs() as usize]);
    let mut starting_positions = vec![];
    for graph in &ground_position_graphs {
        let rows: HashSet<u8> = graph
            .iter()
            .map(|node| {
                node.y()
            })
            .fold(RefCell::new(HashSet::new()), |rows, y| {
                rows.borrow_mut().insert(y);
                rows
            })
            .into_inner();

        match rows.len() {
            0 => unreachable!(),
            1 => {
                let random_index = Range::new(0, graph.len()).ind_sample(&mut rng);
                starting_positions.push(graph[random_index].clone());
            },
            3 => {
                let mut add_random_node_from_row = |row| {
                    let count = graph.iter()
                        .filter(|&node| node.y() == row)
                        .count();
                    let random_index = Range::new(0, count).ind_sample(&mut rng);
                    let (_, node) = graph.iter()
                        .filter(|&node| node.y() == row)
                        .enumerate()
                        .filter(|&(index, _)| index == random_index)
                        .next()
                        .unwrap();

                    starting_positions.push(node.clone());
                };

                let row = *rows.iter().min().unwrap();
                add_random_node_from_row(row);

                let row = *rows.iter().max().unwrap();
                add_random_node_from_row(row);

            },
            _ => unimplemented!(), // TODO: implement the possibility to have an arbitrary row count
        }
    }

    starting_positions
}

fn get_ground_positions(positions: &Positions) -> Vec<Vec<Rc<BoardPosition>>> {
    let mut ground_node_graphs: Vec<Vec<Rc<BoardPosition>>> = vec![];
    let mut visited_nodes: HashSet<Rc<BoardPosition>> = HashSet::new();
    for position in positions.iter() {
        if !visited_nodes.contains(position) {
            ground_node_graphs.push(vec![]);

            fn traverse_nodes(position: Rc<BoardPosition>,
                              visited: &mut HashSet<Rc<BoardPosition>>,
                              graph: &mut Vec<Rc<BoardPosition>>) {
                if visited.contains(&position) { return; }
                visited.insert(position.clone());
                if position.z() == 0 { graph.push(position.clone()); }
                for neighbour in position.neighbours().iter() {
                    traverse_nodes(neighbour.position().clone(), visited, graph);
                }
            }

            traverse_nodes(position.clone(), &mut visited_nodes, ground_node_graphs.last_mut().unwrap());
        }
    }
    ground_node_graphs
}
