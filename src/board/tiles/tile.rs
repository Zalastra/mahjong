use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

use super::position::BoardPosition;
use super::tile_type::TileType;

pub struct Tile {
    position: Rc<BoardPosition>,
    texture: Texture,
    kind: TileType,
}

impl Tile {
    pub fn new(position: Rc<BoardPosition>, kind: TileType, renderer: &Renderer) -> Tile {
        let mut texture_path = PathBuf::from("img/");
        texture_path.push(kind.filename_texture());
        let texture = renderer.load_texture(texture_path.as_path()).expect("error loading texture");

        Tile {
            position: position,
            texture: texture,
            kind: kind,
        }
    }

    pub fn x(&self) -> u8 {
        self.position.x()
    }

    pub fn y(&self) -> u8 {
        self.position.y()
    }

    pub fn z(&self) -> u8 {
        self.position.z()
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
    pub fn is_played(&self) -> bool {
        self.position.is_empty()
    }

    pub fn is_playable(&self) -> bool {
        self.position.is_reachable() && !self.position.is_empty()
    }

    pub fn matches(&self, other: &Tile) -> bool {
        self.kind.matches(&other.kind)
    }

    pub fn play(&self) {
        self.position.empty(true);
    }

    pub fn reset(&self) {
        self.position.empty(false);
    }

    pub fn highlight(&mut self) {
        self.texture.set_color_mod(255, 127, 127);
    }

    pub fn unhighlight(&mut self) {
        self.texture.set_color_mod(255, 255, 255);
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.position.z(), self.kind)
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
