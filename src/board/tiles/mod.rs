use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::cmp::Ordering::*;

use rand::{thread_rng, Rng};
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;

use sdl;

mod models;
mod types;

use self::models::Models;
use self::types::TileType;

use self::Direction::*;
use self::CreationState::*;
use self::PlayState::*;

static ERROR_MESSAGE: &'static str = "error loading texture";

pub struct Tiles {
    positions: Vec<Position>,
    neighbours: Vec<Vec<Neighbour>>,
    types: Vec<TileType>,
    states: Vec<PlayState>,
    models: Models,
    textures: HashMap<TextureId, Texture>,
}

impl Tiles {
    pub fn new(raw_positions: &mut [(u8, u8, u8); 144]) -> Self {
        // NOTE: sorting currently needed for rendering
        // NOTE: also needed now for searching for a tile based on coords
        //       maybe this should just be left in?
        raw_positions.sort_by(|&(x1, y1, z1), &(x2, y2, z2)| {
            if (z1, x2, y1) < (z2, x1, y2) {
                Less
            } else {
                Greater
            }
        });

        let positions = raw_positions.iter()
            .map(Position::from)
            .collect::<Vec<_>>();

        let neighbours = create_neighbour_list(&positions);
        let types = TileShuffler::new(&positions, &neighbours).shuffle();
        let states = vec![Default::default(); 144];
        let models = Models::new(raw_positions);
        let textures = create_textures();

        let mut tiles = Tiles {
            positions: positions,
            neighbours: neighbours,
            types: types,
            states: states,
            models: models,
            textures: textures,
        };

        for tile in 0..144 {
            tiles.update_neighbouring_tile_states(tile);
        }

        tiles
    }

    pub fn reset(&mut self) {
        self.types = TileShuffler::new(&self.positions, &self.neighbours).shuffle();
        for tile in 0..144 {
            self.states[tile] = Blocked;
        }
        for tile in 0..144 {
            self.update_neighbouring_tile_states(tile);
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        use self::TextureId::*;

        let side_tex = self.textures.get(&Side).unwrap();
        let bottom_tex = self.textures.get(&Bottom).unwrap();

        let iter = self.types
            .iter()
            .zip(self.models.iter())
            .zip(self.states.iter());

        for ((tile_type, model), state) in iter {
            if *state == Played {
                continue;
            }

            let face_tex = self.textures.get(&Face(*tile_type, model.is_highlighted())).unwrap();

            let _ = renderer.copy(side_tex, None, Some(model.side()));
            let _ = renderer.copy(bottom_tex, None, Some(model.bottom()));
            let _ = renderer.copy(face_tex, None, Some(model.face()));
        }
    }

    pub fn play_tile(&mut self, tile: TileId) {
        self.states[tile.0] = Played;
        self.update_neighbouring_tile_states(tile.0);
    }

    pub fn reset_tile(&mut self, tile: TileId) {
        self.states[tile.0] = Playable;
        self.update_neighbouring_tile_states(tile.0);
    }

    pub fn highlight_tile(&mut self, tile: TileId) {
        self.models[tile.0].highlight();
    }

    pub fn dehighlight_tile(&mut self, tile: TileId) {
        self.models[tile.0].dehighlight()
    }

    pub fn are_matching(&self, tile1: TileId, tile2: TileId) -> bool {
        self.types[tile1.0].matches(&self.types[tile2.0])
    }

    // TODO return impl Iterator
    pub fn playable_tiles(&self) -> Vec<TileId> {
        self.states
            .iter()
            .enumerate()
            .filter(|&(_, state)| *state == Playable)
            .map(|(index, _)| TileId(index))
            .collect()
    }

    pub fn find_playable_tile_by_coord(&self, x: i32, y: i32) -> Option<TileId> {
        for (index, model) in self.models.iter().enumerate().rev() {
            if self.states[index] == Playable && model.hit_test(x, y) {
                return Some(TileId(index));
            }
        }
        None
    }

    fn update_neighbouring_tile_states(&mut self, tile: usize) {
        for neighbour in &self.neighbours[tile] {
            match self.states[neighbour.id] {
                Blocked | Playable => {
                    let any_up = self.any_unplayed_neighbour_in_direction(neighbour.id, Up);
                    let any_left = self.any_unplayed_neighbour_in_direction(neighbour.id, Left);
                    let any_right = self.any_unplayed_neighbour_in_direction(neighbour.id, Right);
                    
                    if any_up || (any_left && any_right) {
                        self.states[neighbour.id] = Blocked;
                    } else {
                        self.states[neighbour.id] = Playable;
                    }
                }
                Played => (),
            }
        }
    }

    fn any_unplayed_neighbour_in_direction(&self, tile: usize, direction: Direction) -> bool {
        self.neighbours[tile]
            .iter()
            .filter(|neighbour| neighbour.direction == direction)
            .any(|neighbour| self.states[neighbour.id] != Played)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TileId(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Position {
    x: u8,
    y: u8,
    z: u8,
}

impl Position {
    fn neighbours(&self, other: Position) -> Option<Direction> {
        if self.is_up_neighbour(other) { Some(Up) }
        else if self.is_down_neighbour(other) { Some(Down) }
        else if self.is_left_neighbour(other) { Some(Left) }
        else if self.is_right_neighbour(other) { Some(Right) }
        else { None }
    }

    fn is_up_neighbour(&self, other: Position) -> bool {
        self.z + 1 == other.z && self.is_potential_vertical_neighbour(other)
    }

    fn is_down_neighbour(&self, other: Position) -> bool {
        self.z == other.z + 1 && self.is_potential_vertical_neighbour(other)
    }

    fn is_potential_vertical_neighbour(&self, other: Position) -> bool {
        self.x <= other.x + 1 && self.x + 1 >= other.x &&
        self.y <= other.y + 1 && self.y + 1 >= other.y
    }

    fn is_left_neighbour(&self, other: Position) -> bool {
        self.x == other.x + 2 && self.is_potential_horizontal_neighbour(other)
    }

    fn is_right_neighbour(&self, other: Position) -> bool {
        self.x + 2 == other.x && self.is_potential_horizontal_neighbour(other)
    }

    fn is_potential_horizontal_neighbour(&self, other: Position) -> bool {
        self.z == other.z &&
        self.y <= other.y + 1 && self.y + 1 >= other.y
    }
}

impl<'a> From<&'a (u8, u8, u8)> for Position {
    fn from(&(x, y, z): &(u8, u8, u8)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn rev(self) -> Direction {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Neighbour {
    id: usize,
    direction: Direction,
}

impl Neighbour {
    fn new(id: usize, direction: Direction) -> Self {
        Self { id, direction }
    }
}

fn create_neighbour_list(positions: &[Position]) -> Vec<Vec<Neighbour>> {
    let mut neighbour_list = vec![Vec::new(); 144];

    for tile1 in 0..144 {
        for tile2 in 0..144 {
            if let Some(direction) = positions[tile1].neighbours(positions[tile2]) {
                neighbour_list[tile1].push(Neighbour::new(tile2, direction));
            }
        }
    }

    neighbour_list
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayState {
    Blocked,
    Playable,
    Played,
}

impl Default for PlayState {
    fn default() -> PlayState {
        Blocked
    }
}

fn create_textures() -> HashMap<TextureId, Texture> {
    use self::TextureId::*;

    let sdl_systems = sdl::get_systems();
    let renderer = sdl_systems.0.borrow();
    let mut textures = HashMap::new();

    for tile_type in TileType::iter() {
        let mut texture_path_buf = PathBuf::from("img/");
        texture_path_buf.push(tile_type.filename_texture());
        let texture_path = texture_path_buf.as_path();

        let mut texture = renderer.load_texture(texture_path).expect(ERROR_MESSAGE);
        texture.set_color_mod(255, 127, 127);
        textures.insert(Face(*tile_type, true), texture);

        let texture = renderer.load_texture(texture_path).expect(ERROR_MESSAGE);
        textures.insert(Face(*tile_type, false), texture);
    }

    let side_texture = renderer.load_texture(Path::new("img/TileSide.png"))
        .expect(ERROR_MESSAGE);
    let bottom_texture = renderer.load_texture(Path::new("img/TileBottom.png"))
        .expect(ERROR_MESSAGE);

    textures.insert(Side, side_texture);
    textures.insert(Bottom, bottom_texture);

    textures.shrink_to_fit();

    textures
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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum TextureId {
    Face(TileType, bool),
    Bottom,
    Side,
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
        Self { positions, neighbours, states: vec![Default::default(); 144] }
    }

    fn shuffle(mut self) -> Vec<TileType> {
        let mut types = vec![Default::default(); 144];
        loop {
            self.set_random_starting_creation_tiles();
            
            if let Ok(_) = self.try_shuffle(&mut types) {
                break;
            } else {
                self.states = vec![Default::default(); 144]
            }
        }
        types
    }

    fn set_random_starting_creation_tiles(&mut self) {
        for mut group in self.get_grouped_ground_tiles() {
            while !group.is_empty() {
                let random_index = thread_rng().gen_range(0, group.len());
                let tile = group[random_index];

                self.states[tile] = Placable;

                group = group.into_iter().filter(|&tile| {
                    self.states[tile] == Unplaced && self.no_placable_neighbour_in_row(tile)
                }).collect()
            }
        }
    }

    fn no_placable_neighbour_in_row(&self, tile: usize) -> bool {
        self.no_placable_neighbour_in_row_direction(tile, Left) &&
        self.no_placable_neighbour_in_row_direction(tile, Right)
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
            if visited.contains(&tile) { continue }

            let mut group = Vec::new();
            let mut neighbours = self.neighbours[tile].clone();

            while let Some(neighbour) = neighbours.pop() {
                if visited.contains(&neighbour.id) { continue }
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
                },
                (Unplaced, Up) => {
                    let all_down = self.all_neighbours_in_direction_placed(neighbour.id, Down);
                    let unplaced = self.row_unplaced(neighbour.id);
                    let same_y_placed = self.same_y_neighbour_placed(neighbour.id);

                    if all_down && (unplaced || same_y_placed) {
                        self.states[neighbour.id] = Placable;
                    }
                },
                (_, _) => (),
            }
        }
    }

    fn row_unplaced(&self, tile: usize) -> bool {
        self.all_recursive_neighbours_unplaced(tile, Left) &&
        self.all_recursive_neighbours_unplaced(tile, Right)
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
            },
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