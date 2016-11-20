use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rand::{thread_rng, Rng};
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

mod positions;
mod models;
mod types;

use self::positions::Positions;
use self::models::Models;
use self::types::TileType;

static ERROR_MESSAGE: &'static str = "error loading texture";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TileId(usize);

pub struct Tiles {
    positions: Positions,
    types: Vec<TileType>,
    models: Models,
    textures: HashMap<TextureId, Texture>,
}

impl Tiles {
    pub fn new(raw_positions: &mut [(u8, u8, u8); 144], renderer: &Renderer) -> Self {
        // NOTE: sorting currently needed for rendering
        // NOTE: also needed now for searching for a tile based on coords
        //       maybe this should just be left in?
        raw_positions.sort_by(|&(x1, y1, z1), &(x2, y2, z2)| {
            use std::cmp::Ordering::*;
            if (z1, x2, y1) < (z2, x1, y2) {
                Less
            } else {
                Greater
            }
        });

        let mut rng = thread_rng();

        let positions = Positions::new(raw_positions, &mut rng);
        let models = Models::new(raw_positions);
        let textures = create_textures(renderer);

        let mut tiles = Tiles {
            positions: positions,
            types: Vec::new(),
            models: models,
            textures: textures,
        };

        tiles.shuffle_types(&mut rng);
        tiles
    }

    pub fn reset(&mut self) {
        let mut rng = thread_rng();
        self.positions.reset(&mut rng);
        self.shuffle_types(&mut rng);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        use self::TextureId::*;

        let side_tex = self.textures.get(&Side).unwrap();
        let bottom_tex = self.textures.get(&Bottom).unwrap();

        let iter = self.types
            .iter()
            .zip(self.models.iter())
            .zip(self.positions.iter());

        for ((tile_type, model), position) in iter {
            if !position.is_occupied() {
                continue;
            }

            let face_tex = self.textures.get(&Face(*tile_type, model.is_highlighted())).unwrap();

            let _ = renderer.copy(side_tex, None, Some(model.side()));
            let _ = renderer.copy(bottom_tex, None, Some(model.bottom()));
            let _ = renderer.copy(face_tex, None, Some(model.face()));
        }
    }

    pub fn play_tile(&mut self, tile: TileId) {
        self.positions[tile.0].empty();
    }

    pub fn reset_tile(&mut self, tile: TileId) {
        self.positions[tile.0].occupy();
    }

    pub fn highlight_tile(&mut self, tile: TileId) {
        self.models[tile.0].highlight();
    }

    pub fn dehighlight_tile(&mut self, tile: TileId) {
        self.models[tile.0].dehighlight()
    }

    pub fn are_matching(&self, tile1: &TileId, tile2: &TileId) -> bool {
        self.types[tile1.0].matches(&self.types[tile2.0])
    }

    pub fn playable_tiles(&self) -> Vec<TileId> {
        self.positions
            .iter()
            .enumerate()
            .filter(|&(_, position)| position.is_playable())
            .map(|(index, _)| TileId(index))
            .collect()
    }

    pub fn find_playable_tile_by_coord(&self, x: i32, y: i32) -> Option<TileId> {
        for (index, model) in self.models.iter().enumerate().rev() {
            if self.positions[index].is_playable() && model.hit_test(x, y) {
                return Some(TileId(index));
            }
        }
        None
    }

    fn shuffle_types<R: Rng>(&mut self, rng: &mut R) {
        loop {
            match self.try_shuffle_types(rng) {
                Ok(_) => break,
                Err(_) => self.positions.reset(rng),
            }
        }
    }

    fn try_shuffle_types<R: Rng>(&mut self, rng: &mut R) -> Result<(), ()> {
        let mut available_types = get_tile_types();
        let mut used_positions = 0;
        let mut types = [None; 144];

        loop {
            let mut positions = self.placable_positions();

            if positions.len() < 2 {
                return Err(());
            }

            let random_index = rng.gen_range(0, available_types.len() / 2) * 2;
            let tile_type1 = available_types.swap_remove(random_index + 1);
            let tile_type2 = available_types.swap_remove(random_index);

            let random_index = rng.gen_range(0, positions.len());
            let tile_id1 = positions.swap_remove(random_index);

            let random_index = rng.gen_range(0, positions.len());
            let tile_id2 = positions.swap_remove(random_index);

            self.positions[tile_id1.0].occupy();
            self.positions[tile_id2.0].occupy();

            types[tile_id1.0] = Some(tile_type1);
            types[tile_id2.0] = Some(tile_type2);

            used_positions += 2;

            if used_positions == self.positions.len() {
                self.types = types.iter().map(|opt_type| opt_type.unwrap()).collect();
                return Ok(());
            }
        }
    }

    fn placable_positions(&self) -> Vec<TileId> {
        self.positions
            .iter()
            .enumerate()
            .filter(|&(_, position)| position.is_placable())
            .map(|(index, _)| TileId(index))
            .collect()
    }
}

fn create_textures(renderer: &Renderer) -> HashMap<TextureId, Texture> {
    use self::TextureId::*;

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
