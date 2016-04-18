use std::fmt::Debug;
use std::time::Instant;

use sdl2::render::Renderer;

use super::tiles::{Tile, Tiles, TilesBuilder, TileTextures};

pub struct Board {
    tiles: Tiles,
    played: Vec<(usize, usize)>,
    selected_tile: Option<usize>,
    hints: Option<Hints>,
    textures: TileTextures,
}

impl Board {
    pub fn new(renderer: &Renderer) -> Board {

        let mut textures = TileTextures::new();
        textures.load_textures(renderer);
        
        let tiles = TilesBuilder::new(&POSITIONS).build();

        Board {
            tiles: tiles,
            played: Vec::new(),
            selected_tile: None,
            hints: None,
            textures: textures
        }
    }

    pub fn try_select_tile(&mut self, mouse_x: i32, mouse_y: i32) -> Result<(), GameOver> {
        if let Some(index1) = self.find_tile_index_by_coord(mouse_x, mouse_y) {
            self.stop_hints();

            match self.selected_tile {
                Some(index2) => {
                    // deselect tile
                    if index1 == index2 {
                        self.deselect_tile();
                        return Ok(());
                    }

                    // test tile match
                    if !Tile::matches(&self.tiles[index1], &self.tiles[index2]) {
                        return Ok(());
                    }

                    // valid match
                    self.tiles[index1].play();
                    self.tiles[index2].play();
                    self.played.push((index1, index2));

                    self.deselect_tile();

                    if self.get_available_matches().is_err() {
                        return Err(GameOver);
                    }
                },
                None => {
                    self.select_tile(index1)
                }
            }
        }
        Ok(())
    }

    pub fn undo(&mut self) {
        self.deselect_tile();
        self.stop_hints();

        if !self.played.is_empty() {
            let (index1, index2) = self.played.pop().unwrap();
            self.tiles[index1].reset();
            self.tiles[index2].reset();
        }
    }

    pub fn highlight_possible_matches(&mut self) {
        self.deselect_tile();
        self.stop_hints();

        if let Ok(sets) = self.get_available_matches() {
            sets[0].highlight(&mut self.tiles);

            self.hints = Some(Hints {
                sets: sets,
                start_time: Instant::now(),
                current_index: 0,
            });
        }
    }

    pub fn update(&mut self) {
        let mut done = false;
        if let Some(hints) = self.hints.as_mut() {
            let index = (hints.start_time.elapsed().as_secs() / 2) as usize;

            if index > hints.current_index {
                hints.sets[hints.current_index].unhighlight(&mut self.tiles);
                if index >= hints.sets.len() {
                    done = true;
                } else {
                    hints.sets[index].highlight(&mut self.tiles);
                    hints.current_index = index;
                }
            }
        }
        if done {
            self.hints = None;
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        for tile in self.tiles.iter() {
            if tile.is_played() { continue; }

            tile.render(renderer, &self.textures)
        }
    }

    fn get_available_matches(&self) -> Result<Vec<HintSet>, NoMatch> {
        let mut sets = Vec::new();
        let mut used_indices = Vec::new();

        for (index, tile) in self.tiles.iter_playable() {
            if used_indices.contains(&index) { continue; }

            let mut set = HintSet::new(index);
            for (index2, tile2) in self.tiles.iter_playable() {
                if !tile.matches(tile2) || index == index2 { continue; }

                used_indices.push(index2);
                set.add(index2);
            }

            if set.0[1] != None {
                used_indices.push(index);
                sets.push(set);
            }
        }

        if sets.is_empty() {
            Err(NoMatch)
        } else {
            Ok(sets)
        }
    }

    fn select_tile(&mut self, index: usize) {
        self.tiles[index].highlight();
        self.selected_tile = Some(index);
    }

    fn deselect_tile(&mut self) {
        if let Some(index) = self.selected_tile {
            self.tiles[index].unhighlight();
        }
        self.selected_tile = None;
    }

    fn stop_hints(&mut self) {
        if let Some(hints) = self.hints.as_mut() {
            hints.sets[hints.current_index].unhighlight(&mut self.tiles);
        }

        self.hints = None;
    }

    fn find_tile_index_by_coord(&self, x: i32, y: i32) -> Option<usize> {
        for (index, tile) in self.tiles.iter().enumerate().rev() {
            if !tile.is_playable() { continue; }

            let tile_x = tile.x() as i32 * 23 + tile.z() as i32 * 5 + 15;
            let tile_y = tile.y() as i32 * 29 - tile.z() as i32 * 5 + 15;

            if x >= tile_x && x <= tile_x + 46 && y >= tile_y && y <= tile_y + 57 {
                return Some(index);
            }
        }
        None
    }
}

pub struct GameOver;

struct NoMatch;

struct Hints {
    sets: Vec<HintSet>,
    start_time: Instant,
    current_index: usize,
}

struct HintSet ([Option<usize>; 4]);
use std::fmt;
impl Debug for HintSet {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl HintSet {
    fn new(index: usize) -> HintSet {
        HintSet([Some(index), None, None, None])
    }

    fn add(&mut self, index: usize) {
        for opt_index in self.0.iter_mut() {
            if *opt_index == None {
                *opt_index = Some(index);
                break;
            }
        }
    }

    fn highlight(&self, tiles: &mut Tiles) {
        for opt_index in self.0.iter() {
            if let Some(index) = *opt_index {
                tiles[index].highlight();
            }
        }
    }

    fn unhighlight(&self, tiles: &mut Tiles) {
        for opt_index in self.0.iter() {
            if let Some(index) = *opt_index {
                tiles[index].unhighlight();
            }
        }
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
