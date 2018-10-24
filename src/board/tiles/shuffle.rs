use std::collections::HashSet;

use rand::{thread_rng, Rng};

use super::position::{Direction, Neighbour, Position};
use super::types::TileType;
use super::Direction::*;

use self::CreationState::*;

/// Shuffles the tiles by setting the types of the tiles based on an algorithm that guarantees the board can be
/// played to completion.
///
/// Panics if the provided slices are not of equal length
pub fn shuffle(types: &mut [TileType], positions: &[Position], neighbours: &[Vec<Neighbour>]) {
    assert!(
        types.len() == positions.len() && types.len() == neighbours.len(),
        "Given argument slices are not of equal length"
    );
    let shuffler = TileShuffler::new(positions, neighbours);
    shuffler.shuffle(types);
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum CreationState {
    Unplaced,
    Placable,
    Placed,
}

impl Default for CreationState {
    fn default() -> Self {
        Unplaced
    }
}

struct TileShuffler<'a> {
    positions: &'a [Position],
    neighbours: &'a [Vec<Neighbour>],
    states: Vec<CreationState>,
}

impl<'a> TileShuffler<'a> {
    fn new(positions: &'a [Position], neighbours: &'a [Vec<Neighbour>]) -> Self {
        Self {
            positions,
            neighbours,
            states: vec![Default::default(); 144],
        }
    }

    fn shuffle(mut self, types: &mut [TileType]) {
        //let mut types = vec![Default::default(); 144];
        loop {
            self.set_random_starting_creation_tiles();

            if self.try_shuffle(types).is_ok() {
                break;
            } else {
                self.states = vec![Default::default(); 144]
            }
        }
        //types
    }

    fn set_random_starting_creation_tiles(&mut self) {
        for mut group in self.get_grouped_ground_tiles() {
            while !group.is_empty() {
                let random_index = thread_rng().gen_range(0, group.len());
                let tile = group[random_index];

                self.states[tile] = Placable;

                group = group
                    .into_iter()
                    .filter(|&tile| {
                        self.states[tile] == Unplaced && self.no_placable_neighbour_in_row(tile)
                    }).collect()
            }
        }
    }

    fn no_placable_neighbour_in_row(&self, tile: usize) -> bool {
        self.no_placable_neighbour_in_row_direction(tile, Left)
            && self.no_placable_neighbour_in_row_direction(tile, Right)
    }

    fn no_placable_neighbour_in_row_direction(&self, tile: usize, direction: Direction) -> bool {
        let mut neighbours = self.neighbours[tile].clone();
        while let Some(neighbour) = neighbours.pop() {
            if neighbour.direction == direction {
                if self.states[neighbour.id] == Placable {
                    return false;
                }
                neighbours.extend_from_slice(&self.neighbours[neighbour.id]);
            }
        }
        true
    }

    fn get_grouped_ground_tiles(&self) -> Vec<Vec<usize>> {
        let mut position_groups = Vec::new();
        let mut visited = HashSet::new();

        for tile in 0..144 {
            if visited.contains(&tile) {
                continue;
            }

            let mut group = Vec::new();
            let mut neighbours = self.neighbours[tile].clone();

            while let Some(neighbour) = neighbours.pop() {
                if visited.contains(&neighbour.id) {
                    continue;
                }
                visited.insert(neighbour.id);

                if self.positions[neighbour.id].z == 0 {
                    group.push(neighbour.id);
                    neighbours.extend_from_slice(&self.neighbours[neighbour.id]);
                }
            }

            position_groups.push(group);
        }

        position_groups
    }

    fn try_shuffle(&mut self, types: &mut [TileType]) -> Result<(), ()> {
        let mut available_types = get_tile_types();
        let mut tiles_placed = 0;

        loop {
            if cfg!(feature = "debug") {
                // TODO: figure out a way to be able to render the board during the shuffle
                //self.debug_render();
                //sdl::wait_for_click();
            }

            /*
            TODO:   Figure out an actual working strategy to prevent the creation of an unfinished board.
            
            IDEA:   We would need to look up the number of tiles that must be placed for an unplaced tile
                    and see if that number matches the amount of turns of placing tiles left. This could
                    still lead to problems when there's multiple of these tiles that depend on other tiles
                    being placed.

            Code below in placable_tiles_adjusted is a slight improvement reducing the number of failed boards
            */

            let mut tiles = self.placable_tiles_adjusted(tiles_placed);

            if tiles.len() < 2 {
                return Err(());
            }

            let random_index = thread_rng().gen_range(0, available_types.len() / 2) * 2;
            let tile_type1 = available_types.swap_remove(random_index + 1);
            let tile_type2 = available_types.swap_remove(random_index);

            let random_index = thread_rng().gen_range(0, tiles.len());
            let tile_id1 = tiles.swap_remove(random_index);

            let random_index = thread_rng().gen_range(0, tiles.len());
            let tile_id2 = tiles.swap_remove(random_index);

            self.set_placed(tile_id1);
            self.set_placed(tile_id2);

            types[tile_id1] = tile_type1;
            types[tile_id2] = tile_type2;

            tiles_placed += 2;

            if tiles_placed == self.positions.len() {
                return Ok(());
            }
        }
    }

    fn set_placed(&mut self, tile: usize) {
        self.states[tile] = Placed;
        self.update_neighbour_creation_states(tile);
    }

    fn update_neighbour_creation_states(&mut self, tile: usize) {
        for neighbour in &self.neighbours[tile] {
            /*
            all down neighbours must be placed
            
            if updated by side neighbour:
                same y: always place
                different y: all neighbours on reverse direction must be placed
            if update from bottom neighbour:
                if entire row Unplaced: place
                if side neighbour with same y is placed: place
                |TODO| if direct neighbour is Placable, rest of row is Unplaced: place
            if update from top neighbour:
                tile should already be placed
            */
            match (self.states[neighbour.id], neighbour.direction) {
                (Unplaced, direction @ Left) | (Unplaced, direction @ Right) => {
                    let all_down = self.all_neighbours_in_direction_placed(neighbour.id, Down);
                    let same_y = self.positions[tile].y == self.positions[neighbour.id].y;
                    let all_placed_in_source_direction =
                        self.all_neighbours_in_direction_placed(neighbour.id, direction.rev());

                    if all_down && (same_y || all_placed_in_source_direction) {
                        self.states[neighbour.id] = Placable;
                    }
                }
                (Unplaced, Up) => {
                    let all_down = self.all_neighbours_in_direction_placed(neighbour.id, Down);
                    let unplaced = self.row_unplaced(neighbour.id);
                    let same_y_placed = self.same_y_neighbour_placed(neighbour.id);

                    if all_down && (unplaced || same_y_placed) {
                        self.states[neighbour.id] = Placable;
                    }
                }
                (_, _) => (),
            }
        }
    }

    fn row_unplaced(&self, tile: usize) -> bool {
        self.all_recursive_neighbours_unplaced(tile, Left)
            && self.all_recursive_neighbours_unplaced(tile, Right)
    }

    fn same_y_neighbour_placed(&self, tile: usize) -> bool {
        self.neighbours[tile].iter().any(|neighbour| {
            let direction = neighbour.direction;
            let same_y = self.positions[tile].y == self.positions[neighbour.id].y;
            let placed = self.states[neighbour.id] == Placed;

            (direction == Left || direction == Right) && same_y && placed
        })
    }

    fn all_neighbours_in_direction_placed(&self, tile: usize, direction: Direction) -> bool {
        self.neighbours[tile]
            .iter()
            .filter(|neighbour| neighbour.direction == direction)
            .all(|neighbour| self.states[neighbour.id] == Placed)
    }

    fn all_recursive_neighbours_unplaced(&self, tile: usize, direction: Direction) -> bool {
        let mut neighbours = self.neighbours[tile].clone();

        while let Some(neighbour) = neighbours.pop() {
            if neighbour.direction == direction {
                if self.states[neighbour.id] != Unplaced {
                    return false;
                }
                neighbours.extend_from_slice(&self.neighbours[neighbour.id]);
            }
        }
        true
    }

    fn placable_tiles_adjusted(&self, amount_placed: usize) -> Vec<usize> {
        let mut tiles = self.placable_tiles();

        match (amount_placed, tiles.len()) {
            (142, 2) => (),
            (_, 3) => {
                let mut new_tiles = Vec::new();
                let mut ignored = false;
                for &tile in &tiles {
                    let mut ignore = !ignored;
                    for neighbour in &self.neighbours[tile] {
                        if self.states[neighbour.id] != Placed {
                            ignore = false;
                            break;
                        }
                    }
                    if ignore {
                        ignored = true;
                    } else {
                        new_tiles.push(tile);
                    }
                }
                tiles = new_tiles;
            }
            _ => (),
        }

        tiles
    }

    fn placable_tiles(&self) -> Vec<usize> {
        self.states
            .iter()
            .enumerate()
            .filter(|&(_, state)| *state == Placable)
            .map(|(idx, _)| idx)
            .collect()
    }
}
