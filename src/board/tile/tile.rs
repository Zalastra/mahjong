use std::fmt;
use std::path::PathBuf;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

use board::tile::{Position, TilePosition, TileType};

pub struct Tile {
    pub position: TilePosition,
    pub texture: Texture,
    kind: TileType,
}

impl Tile {
    pub fn new(position: TilePosition, kind: TileType, renderer: &Renderer) -> Tile {
        let mut texture_path = PathBuf::from("img/");
        texture_path.push(kind.filename_texture());
        let texture = renderer.load_texture(texture_path.as_path()).expect("error loading texture");

        Tile {
            position: position,
            texture: texture,
            kind: kind,
        }
    }

    pub fn matches(&self, other: &Tile) -> bool {
        self.kind.matches(&other.kind)
    }

    pub fn is_on_position(&self, other_position: &Position) -> bool {
        self.position.is_on_position(other_position)
    }
}

// TODO: should be removed if we dont need console printing for debugging
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.position.z(), self.kind)
    }
}
