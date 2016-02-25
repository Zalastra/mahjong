use std::path::Path;

use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;

use sdl2_image::LoadTexture;

use super::tiles::{Tile, Tiles, TilesBuilder};

pub struct Board {
    tiles: Tiles,
    played: Vec<(usize, usize)>,
    selected_tile: Option<usize>,
    side_texture: Texture,
    bottom_texture: Texture,
}

impl Board {
    pub fn new(renderer: &Renderer) -> Board {

        let side_texture = renderer.load_texture(Path::new("img/TileSide.png")).expect("error loading side texture");
        let bottom_texture = renderer.load_texture(Path::new("img/TileBottom.png")).expect("error loading bottom texture");

        Board {
            tiles: TilesBuilder::new(&POSITIONS, renderer).build(),
            played: Vec::new(),
            selected_tile: None,
            side_texture: side_texture,
            bottom_texture: bottom_texture,
        }
    }

    // TODO: Needs better name or reflect the fact it might not work through returning an error
    pub fn try_select_tile(&mut self, mouse_x: i32, mouse_y: i32) {
        if let Some(index1) = self.find_tile_index_by_coord(mouse_x, mouse_y) {
            match self.selected_tile {
                Some(index2) => {
                    // deselect tile
                    if index1 == index2 {
                        self.deselect_tile(index2);
                        return;
                    }

                    // test tile match
                    if !Tile::matches(&self.tiles[index1], &self.tiles[index2]) {
                        return;
                    }

                    // valid match
                    self.play_tiles(index1, index2);

                    self.deselect_tile(index2);
                },
                None => {
                    self.select_tile(index1)
                }
            }
        }
    }

    pub fn undo(&mut self) {
        if let Some(index) = self.selected_tile {
            self.deselect_tile(index)
        }
        if !self.played.is_empty() {
            let (index1, index2) = self.played.pop().unwrap();
            self.tiles[index1].reset();
            self.tiles[index2].reset();
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        for tile in self.tiles.iter() {
            if tile.is_played() { continue; }

            let x = tile.x() as i32 * 23 + tile.z() as i32 * 5 + 20;
            let y = tile.y() as i32 * 29 - tile.z() as i32 * 5 + 15;

            renderer.copy(&self.side_texture, None, Rect::new(x - 5, y, 5, 62).unwrap());
            renderer.copy(&self.bottom_texture, None, Rect::new(x, y + 57, 46, 5).unwrap());

            renderer.copy(&tile.texture(), None, Rect::new(x, y, 46, 57).unwrap());
        }
    }

    fn play_tiles(&mut self, index1: usize, index2: usize) {
        self.tiles[index1].play();
        self.tiles[index2].play();
        self.played.push((index1, index2));
    }

    fn select_tile(&mut self, index: usize) {
        self.tiles[index].highlight();
        self.selected_tile= Some(index);
    }

    fn deselect_tile(&mut self, index: usize) {
        self.tiles[index].unhighlight();
        self.selected_tile = None;
    }

    fn find_tile_index_by_coord(&self, x: i32, y: i32) -> Option<usize> {
        for (index, tile) in self.tiles.iter().enumerate().rev() {
            if tile.is_played() || !tile.is_playable() { continue; }

            let tile_x = tile.x() as i32 * 23 + tile.z() as i32 * 5 + 15;
            let tile_y = tile.y() as i32 * 29 - tile.z() as i32 * 5 + 15;

            if x >= tile_x && x <= tile_x + 46 && y >= tile_y && y <= tile_y + 57 {
                return Some(index);
            }
        }
        None
    }
}

// TODO: put positions in a file and read them from disk
//       use human readable format
static POSITIONS: [u32; 144] = [
    4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26,
    72, 74, 76, 78, 80, 82, 84, 86,
    134, 136, 138, 140, 142, 144, 146, 148, 150, 152,
    196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218,
    224, 226, 252,
    260, 262, 264, 266, 268, 270, 272, 274, 276, 278, 280, 282,
    326, 328, 330, 332, 334, 336, 338, 340, 342, 344,
    392, 394, 396, 398, 400, 402, 404, 406,
    452, 454, 456, 458, 460, 462, 464, 466, 468, 470, 472, 474,

    1098, 1100, 1102, 1104, 1106, 1108,
    1162, 1164, 1166, 1168, 1170, 1172,
    1226, 1228, 1230, 1232, 1234, 1236,
    1290, 1292, 1294, 1296, 1298, 1300,
    1354, 1356, 1358, 1360, 1362, 1364,
    1418, 1420, 1422, 1424, 1426, 1428,

    2188, 2190, 2192, 2194,
    2252, 2254, 2256, 2258,
    2316, 2318, 2320, 2322,
    2380, 2382, 2384, 2386,

    3278, 3280,
    3342, 3344,

    4335,
];
