use {
    rand::{
        thread_rng,
        Rng,
    },
    super::{
        Direction,
        Neighbour,
        TileType,
    },
    self::ShuffleState::*,
};

pub fn get_shuffled_types(neighbours: &[Vec<Neighbour>]) -> Vec<TileType> {
    assert!(neighbours.len() % 2 == 0);

    let mut shuffler = TypeShuffler::from_neighbourlist(neighbours);

    for _ in 0..neighbours.len() / 2 {
        shuffler.place_random_type_pair();
    }

    shuffler.set_types.iter().filter_map(|t| *t).collect()
}

#[derive(Debug)]
struct TypeShuffler<'n> {
    neighbours: &'n [Vec<Neighbour>],
    states: Vec<ShuffleState>,
    available_types: Vec<TileType>,
    set_types: Vec<Option<TileType>>,
}

impl<'n> TypeShuffler<'n> {
    fn from_neighbourlist(neighbours: &'n [Vec<Neighbour>]) -> Self {
        let states = vec![ShuffleState::default(); neighbours.len()];
        let available_types = get_tile_types();
        let set_types = vec![None; neighbours.len()];

        let mut type_shuffler = Self {
            neighbours,
            states,
            available_types,
            set_types,
        };

        for tile_id in 0..neighbours.len() {
            type_shuffler.update_unplaced_neighbours_shuffle_states(tile_id)
        }

        type_shuffler
    }

    fn place_random_type_pair(&mut self) {
        let mut placable_tiles = self.get_placable_tiles();

        let random_index = thread_rng().gen_range(0, self.available_types.len() / 2) * 2;
        let tile_type1 = self.available_types.swap_remove(random_index + 1);
        let tile_type2 = self.available_types.swap_remove(random_index);

        let random_index = thread_rng().gen_range(0, placable_tiles.len());
        let tile_id1 = placable_tiles.swap_remove(random_index);

        let random_index = thread_rng().gen_range(0, placable_tiles.len());
        let tile_id2 = placable_tiles.swap_remove(random_index);

        self.states[tile_id1] = Placed;
        self.update_unplaced_neighbours_shuffle_states(tile_id1);
        self.states[tile_id2] = Placed;
        self.update_unplaced_neighbours_shuffle_states(tile_id2);

        self.set_types[tile_id1] = Some(tile_type1);
        self.set_types[tile_id2] = Some(tile_type2);
    }

    fn get_placable_tiles(&self) -> Vec<usize> {
        self.states
            .iter()
            .enumerate()
            .filter(|&(_, &state)| state == Placable)
            .map(|(index, _)| index)
            .collect()
    }

    fn update_unplaced_neighbours_shuffle_states(&mut self, tile_id: usize) {
        use self::Direction::*;

        for neighbour in &self.neighbours[tile_id] {
            if self.states[neighbour.id] == Blocked {
                let any_up = self.any_unplaced_neighbour_in_direction(neighbour.id, Up);
                let any_left = self.any_unplaced_neighbour_in_direction(neighbour.id, Left);
                let any_right = self.any_unplaced_neighbour_in_direction(neighbour.id, Right);

                if !(any_up || (any_left && any_right)) {
                    
                    self.states[neighbour.id] = Placable;
                }
            }
        }
    }

    fn any_unplaced_neighbour_in_direction(&self, tile: usize, direction: Direction) -> bool {
        self.neighbours[tile]
            .iter()
            .filter(|neighbour| neighbour.direction == direction)
            .any(|neighbour| self.states[neighbour.id] != Placed)
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum ShuffleState {
    Blocked,
    Placable,
    Placed,
}

impl Default for ShuffleState {
    fn default() -> Self {
        Blocked
    }
}
