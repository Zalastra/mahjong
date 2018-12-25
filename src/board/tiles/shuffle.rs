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
    let mut shuffler: TypeShuffler<SmallRng> = ShufflerBuilder::new(positions, neighbours)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

    for _ in 0..neighbours.len() / 2 {
        shuffler.place_random_type_pair()
    }

    shuffler.set_types.iter().filter_map(|t| *t).collect()
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

    #[allow(dead_code)]
    pub fn types(mut self, types: Vec<TileType>) -> Self {
        self.types = Some(types);
        self
    }

    #[allow(dead_code)]
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
    /**
     * Tile shuffle strategy is to assign a random type pair to two random tiles according to the
     * same rules as they can be played. Additionally to make sure the process does not enter an
     * invalid state the Z position of tiles is checked to see if some tiles need to be prioritized
     * for assignment.
     */
    fn place_random_type_pair(&mut self) {
        let mut placable_tiles = self.get_placable_tiles();

        self.tiles_left -= 2;

        let placable_tile_index = placable_tiles
            .iter()
            .enumerate()
            .find(|(_, &tile)| self.tiles_left == usize::from(self.positions[tile].z) * 2)
            .map(|(index, _)| index)
            .unwrap_or_else(|| thread_rng().gen_range(0, placable_tiles.len()));

        let tile_id1 = placable_tiles.swap_remove(placable_tile_index);

        let placable_tile_index = placable_tiles
            .iter()
            .enumerate()
            .find(|(_, &tile)| self.tiles_left == usize::from(self.positions[tile].z) * 2)
            .map(|(index, _)| index)
            .unwrap_or_else(|| thread_rng().gen_range(0, placable_tiles.len()));

        let tile_id2 = placable_tiles.swap_remove(placable_tile_index);

        self.states[tile_id1] = Placed;
        self.states[tile_id2] = Placed;

        self.update_unplaced_neighbours_shuffle_states(tile_id1);
        self.update_unplaced_neighbours_shuffle_states(tile_id2);

        let random_index = thread_rng().gen_range(0, self.available_types.len() / 2) * 2;
        let tile_type1 = self.available_types.swap_remove(random_index + 1);
        let tile_type2 = self.available_types.swap_remove(random_index);

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
