use std::fmt;
use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::Renderer;

use super::position::BoardPosition;
use super::tile_type::TileType;
use super::textures::TileTextures;
use super::textures::TextureId::*;

pub struct Tile {
    position: Rc<BoardPosition>,
    kind: TileType,
    highlighted: bool,
}

impl Tile {
    pub fn new(position: Rc<BoardPosition>, kind: TileType) -> Tile {
        Tile {
            position: position,
            kind: kind,
            highlighted: false,
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
        self.highlighted = true;
    }

    pub fn unhighlight(&mut self) {
        self.highlighted = false;
    }
    
    pub fn render(&self, renderer: &mut Renderer, textures: &TileTextures) {
        let x = self.x() as i32 * 23 + self.z() as i32 * 5 + 20;
        let y = self.y() as i32 * 29 - self.z() as i32 * 5 + 15;
        
        textures.render(renderer, Side, Rect::new(x - 5, y, 5, 62));
        textures.render(renderer, Bottom, Rect::new(x, y + 57, 46, 5));
        textures.render(renderer, Face(self.kind, self.highlighted), Rect::new(x, y, 46, 57));
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{:?}", self.position.z(), self.kind)
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
