use {
    std::default::Default,
    rand::{
        thread_rng,
        Rng,
        SeedableRng,
        FromEntropy,
        rngs::SmallRng,
    },
    super::{
        Direction,
        Neighbour,
        Position,
        TileType,
    },
    self::ShuffleState::*,
};

pub fn get_shuffled_types(positions: &[Position], neighbours: &[Vec<Neighbour>]) -> Vec<TileType> {
    // TODO: Fix the shuffling for real this time.
    //       Placing tiles as if playing them still leaves the possibility of being left
    //       with two tiles on top of eachother that both need to be placed but obviously can't.
    loop {
        let mut shuffler: TypeShuffler<SmallRng> = ShufflerBuilder::new(positions, neighbours)
            .seed_rng(127)
            .build()
            .unwrap_or_else(|err| panic!("{}", err));

        for _ in 0..neighbours.len() / 2 {
            if !shuffler.place_random_type_pair() {
                continue;
            }
        }

        return shuffler.set_types.iter().filter_map(|t| *t).collect();
    }
}

struct ShufflerBuilder<'td, R: Rng> {
    positions: &'td [Position],
    neighbours: &'td [Vec<Neighbour>],
    types: Option<Vec<TileType>>,
    rng: Option<R>,
}

impl<'td, R> ShufflerBuilder<'td, R> where R: Rng + SeedableRng + FromEntropy {
    pub fn new(positions:  &'td [Position], neighbours: &'td [Vec<Neighbour>]) -> Self {
        Self {
            positions,
            neighbours,
            types: None,
            rng: None,
        }
    }

    pub fn types(mut self, types: Vec<TileType>) -> Self {
        self.types = Some(types);
        self
    }

    pub fn seed_rng(mut self, seed: u64) -> Self {
        let rng = R::seed_from_u64(seed);
        self.rng = Some(rng);
        self
    }

    pub fn build(self) -> Result<TypeShuffler<'td, R>, &'static str> {
        let num_tiles = self.positions.len();

        if self.neighbours.len() != num_tiles {
            return Err("neighbours length does not match positions length");
        }

        let available_types = self.types.unwrap_or_else(get_tile_types);

        if available_types.len() != num_tiles {
            return Err("types length does not match positions length");
        }

        let rng = self.rng.unwrap_or_else(R::from_entropy);

        let mut type_shuffler = TypeShuffler {
            tiles_left: num_tiles,
            positions: self.positions,
            neighbours: self.neighbours,
            states: vec![Default::default(); num_tiles],
            available_types,
            set_types: vec![None; num_tiles],
            rng,
        };

        for tile_id in 0..num_tiles {
            type_shuffler.update_unplaced_neighbours_shuffle_states(tile_id)
        }

        Ok(type_shuffler)
    }
}

#[derive(Debug)]
struct TypeShuffler<'td, R: Rng> {
    tiles_left: usize,
    positions: &'td [Position],
    neighbours: &'td [Vec<Neighbour>],
    states: Vec<ShuffleState>,
    available_types: Vec<TileType>,
    set_types: Vec<Option<TileType>>,
    rng: R,
}

impl<R> TypeShuffler<'_, R> where R: Rng {
    fn place_random_type_pair(&mut self) -> bool {
        let mut placable_tiles = self.get_placable_tiles();

        /*let unplaced_elevated_tiles = self
            .states
            .iter()
            .enumerate()
            .filter(|&(_, &state)| state != Placed)
            .filter_map(|(tile, _)| {
                if self.any_unplaced_neighbour_in_direction(tile, Direction::Down) {
                    Some(tile)
                } else {
                    None
                }
            })
            .count();

        if (unplaced_elevated_tiles / 2 + unplaced_elevated_tiles % 2) == pairs_left {
            placable_tiles = placable_tiles.into_iter().filter(|&tile| {
                self.neighbours[tile].iter().any(|neighbour| neighbour.direction == Direction::Down)
            }).collect();
        }*/

        if placable_tiles.len() < 2 {
            return false;
        }

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

        true
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
