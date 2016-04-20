use std::collections::HashMap;
use std::path::{Path, PathBuf};

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

use super::tile_type::TileType;

use self::TextureId::*;

static ERROR_MESSAGE: &'static str = "error loading texture";

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TextureId {
    Face(TileType, bool),
    Bottom,
    Side,
}

pub struct TileTextures {
    textures: HashMap<TextureId, Texture>,
}

impl TileTextures {
    pub fn new() -> TileTextures {
        TileTextures { textures: HashMap::new() }
    }

    pub fn load_textures(&mut self, renderer: &Renderer) {
        for tile_type in TileType::iter() {
            let mut texture_path_buf = PathBuf::from("img/");
            texture_path_buf.push(tile_type.filename_texture());
            let texture_path = texture_path_buf.as_path();

            let mut texture = renderer.load_texture(texture_path).expect(ERROR_MESSAGE);
            texture.set_color_mod(255, 127, 127);
            self.textures.insert(Face(*tile_type, true), texture);

            let texture = renderer.load_texture(texture_path).expect(ERROR_MESSAGE);
            self.textures.insert(Face(*tile_type, false), texture);
        }

        let side_texture = renderer.load_texture(Path::new("img/TileSide.png"))
                                   .expect(ERROR_MESSAGE);
        let bottom_texture = renderer.load_texture(Path::new("img/TileBottom.png"))
                                     .expect(ERROR_MESSAGE);

        self.textures.insert(Side, side_texture);
        self.textures.insert(Bottom, bottom_texture);
    }

    pub fn render(&self, renderer: &mut Renderer, id: TextureId, placement: Rect) {
        if let Some(texture) = self.textures.get(&id) {
            renderer.copy(texture, None, Some(placement));
        }
    }
}
