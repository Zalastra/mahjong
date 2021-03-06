mod models;
mod position;
mod shuffle;
mod types;

use {
    std::{
        cmp::Ordering,
        collections::HashMap,
        iter::{
            Enumerate,
            FilterMap,
        },
        path::{
            Path,
            PathBuf,
        },
        slice::Iter,
    },
    sdl2::{
        image::LoadTexture,
        render::{
            Texture,
            TextureCreator,
            WindowCanvas,
        },
        video::WindowContext,
    },
    self::{
        models::Models,
        position::{
            Direction,
            Neighbour,
            Position,
        },
        shuffle::get_shuffled_types,
        types::TileType,
        PlayState::*,
    }
};

static ERROR_MESSAGE: &str = "error loading texture";

pub struct Tiles<'tc> {
    positions: Vec<Position>,
    neighbours: Vec<Vec<Neighbour>>,
    types: Vec<TileType>,
    states: Vec<PlayState>,
    models: Models,
    textures: HashMap<TextureId, Texture<'tc>>,
}

impl<'tc> Tiles<'tc> {
    pub fn new(
        raw_positions: &mut [(u8, u8, u8); 144],
        texture_creator: &'tc TextureCreator<WindowContext>,
    ) -> Self {
        // NOTE: sorting currently needed for rendering
        // NOTE: also needed now for searching for a tile based on coords
        //       maybe this should just be left in?
        raw_positions.sort_by(|&(x1, y1, z1), &(x2, y2, z2)| {
            if (z1, x2, y1) < (z2, x1, y2) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let positions = raw_positions.iter().map(Position::from).collect::<Vec<_>>();

        let neighbours = create_neighbour_list(&positions);
        let types = get_shuffled_types(&positions, &neighbours);
        let states = vec![Default::default(); 144];
        let models = Models::new(raw_positions);
        let textures = create_textures(texture_creator);

        let mut tiles = Tiles {
            positions,
            neighbours,
            types,
            states,
            models,
            textures,
        };

        for tile in 0..144 {
            tiles.update_neighbouring_tile_states(tile);
        }

        tiles
    }

    pub fn reset(&mut self) {
        self.types = get_shuffled_types(&self.positions, &self.neighbours);
        
        for tile in 0..144 {
            self.states[tile] = Blocked;
        }
        for tile in 0..144 {
            self.update_neighbouring_tile_states(tile);
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        use self::TextureId::*;

        let side_tex = &self.textures[&Side];
        let bottom_tex = &self.textures[&Bottom];

        let iter = self
            .types
            .iter()
            .zip(self.models.iter())
            .zip(self.states.iter());

        for ((tile_type, model), state) in iter {
            if *state == Played {
                continue;
            }

            let face_tex = &self.textures[&Face(*tile_type, model.is_highlighted())];

            let _ = canvas.copy(side_tex, None, Some(model.side()));
            let _ = canvas.copy(bottom_tex, None, Some(model.bottom()));
            let _ = canvas.copy(face_tex, None, Some(model.face()));
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
        self.types[tile1.0].matches(self.types[tile2.0])
    }

    pub fn playable_tiles(&self) -> PlayableTiles {
        PlayableTiles {
            iter: self
                .states
                .iter()
                .enumerate()
                .filter_map(|(index, &state)| {
                    if state == Playable {
                        Some(TileId(index))
                    } else {
                        None
                    }
                }),
        }
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
        use self::Direction::*;

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

#[derive(Clone, Debug)]
pub struct PlayableTiles<'a> {
    iter: FilterMap<
        Enumerate<Iter<'a, PlayState>>,
        for<'r> fn((usize, &'r PlayState)) -> Option<TileId>,
    >,
}

impl Iterator for PlayableTiles<'_> {
    type Item = TileId;

    fn next(&mut self) -> Option<TileId> {
        self.iter.next()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TileId(usize);

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

fn create_textures<'tc>(
    texture_creator: &'tc TextureCreator<WindowContext>,
) -> HashMap<TextureId, Texture<'tc>> {
    use self::TextureId::*;

    let mut textures = HashMap::new();

    for tile_type in TileType::iter() {
        let mut texture_path_buf = PathBuf::from("img/");
        texture_path_buf.push(tile_type.filename_texture());
        let texture_path = texture_path_buf.as_path();

        let mut texture = texture_creator
            .load_texture(texture_path)
            .expect(ERROR_MESSAGE);
        texture.set_color_mod(255, 127, 127);
        textures.insert(Face(*tile_type, true), texture);

        let texture = texture_creator
            .load_texture(texture_path)
            .expect(ERROR_MESSAGE);
        textures.insert(Face(*tile_type, false), texture);
    }

    let side_texture = texture_creator
        .load_texture(Path::new("img/TileSide.png"))
        .expect(ERROR_MESSAGE);
    let bottom_texture = texture_creator
        .load_texture(Path::new("img/TileBottom.png"))
        .expect(ERROR_MESSAGE);

    textures.insert(Side, side_texture);
    textures.insert(Bottom, bottom_texture);

    textures.shrink_to_fit();

    textures
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum TextureId {
    Face(TileType, bool),
    Bottom,
    Side,
}
