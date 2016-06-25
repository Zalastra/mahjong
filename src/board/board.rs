use std::fmt::Debug;
use std::time::Instant;

use sdl2::render::Renderer;

use super::tiles::{TileId, Tiles};

pub struct Board {
    tiles: Tiles,
    played: Vec<(TileId, TileId)>,
    selected_tile: Option<TileId>,
    hints: Option<Hints>,
}

impl Board {
    pub fn new(renderer: &Renderer) -> Board {
        let tiles = Tiles::new(&POSITIONS, renderer);

        Board {
            tiles: tiles,
            played: Vec::new(),
            selected_tile: None,
            hints: None,
        }
    }

    pub fn reset(&mut self) {
        self.tiles.reset();
        self.played = Vec::new();
        self.selected_tile = None;
        self.hints = None;
    }

    pub fn try_select_tile(&mut self, mouse_x: i32, mouse_y: i32) -> Result<(), GameOver> {
        if let Some(tile1) = self.tiles.find_playable_tile_by_coord(mouse_x, mouse_y) {
            self.stop_hints();

            match self.selected_tile {
                Some(tile2) => {
                    // deselect tile
                    if tile1 == tile2 {
                        self.deselect_tile();
                        return Ok(());
                    }

                    // test tile match
                    if !self.tiles.are_matching(&tile1, &tile2) {
                        return Ok(());
                    }

                    // valid match
                    self.tiles.play_tile(tile1);
                    self.tiles.play_tile(tile2);
                    self.played.push((tile1, tile2));

                    self.deselect_tile();

                    if self.get_available_matches().is_err() {
                        return Err(GameOver);
                    }
                }
                None => self.select_tile(tile1),
            }
        }
        Ok(())
    }

    pub fn undo(&mut self) {
        self.deselect_tile();
        self.stop_hints();

        if let Some((tile1, tile2)) = self.played.pop() {
            self.tiles.reset_tile(tile1);
            self.tiles.reset_tile(tile2);
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
                hints.sets[hints.current_index].dehighlight(&mut self.tiles);
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
        self.tiles.render(renderer);
    }

    fn get_available_matches(&self) -> Result<Vec<HintSet>, NoMatch> {
        let mut sets = Vec::new();
        let mut used_tiles = Vec::new();

        for tile in self.tiles.playable_tiles().iter() {
            if used_tiles.contains(tile) {
                continue;
            }

            let mut set = HintSet::new(*tile);
            for tile2 in self.tiles.playable_tiles().iter() {
                if !self.tiles.are_matching(tile, tile2) || tile == tile2 {
                    continue;
                }

                used_tiles.push(*tile2);
                set.add(*tile2);
            }

            if set.0[1] != None {
                used_tiles.push(*tile);
                sets.push(set);
            }
        }

        if sets.is_empty() {
            Err(NoMatch)
        } else {
            Ok(sets)
        }
    }

    fn select_tile(&mut self, tile: TileId) {
        self.tiles.highlight_tile(tile);
        self.selected_tile = Some(tile);
    }

    fn deselect_tile(&mut self) {
        if let Some(tile) = self.selected_tile {
            self.tiles.dehighlight_tile(tile)
        }
        self.selected_tile = None;
    }

    fn stop_hints(&mut self) {
        if let Some(hints) = self.hints.as_mut() {
            hints.sets[hints.current_index].dehighlight(&mut self.tiles);
        }

        self.hints = None;
    }
}

pub struct GameOver;

struct NoMatch;

struct Hints {
    sets: Vec<HintSet>,
    start_time: Instant,
    current_index: usize,
}

struct HintSet([Option<TileId>; 4]);
use std::fmt;
impl Debug for HintSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl HintSet {
    fn new(tile: TileId) -> HintSet {
        HintSet([Some(tile), None, None, None])
    }

    fn add(&mut self, tile: TileId) {
        for opt_tile in self.0.iter_mut() {
            if *opt_tile == None {
                *opt_tile = Some(tile);
                break;
            }
        }
    }

    fn highlight(&self, tiles: &mut Tiles) {
        for opt_tile in self.0.iter() {
            if let Some(tile) = *opt_tile {
                tiles.highlight_tile(tile);
            }
        }
    }

    fn dehighlight(&self, tiles: &mut Tiles) {
        for opt_tile in self.0.iter() {
            if let Some(tile) = *opt_tile {
                tiles.dehighlight_tile(tile)
            }
        }
    }
}

// TODO: add fmt exception
static POSITIONS: [(u8, u8, u8); 144] = [(4, 0, 0), (6, 0, 0), (8, 0, 0), (10, 0, 0), (12, 0, 0),
    (14, 0, 0), (16, 0, 0), (18, 0, 0), (20, 0, 0), (22, 0, 0), (24, 0, 0), (26, 0, 0), (8, 2, 0),
    (10, 2, 0), (12, 2, 0), (14, 2, 0), (16, 2, 0), (18, 2, 0), (20, 2, 0), (22, 2, 0), (6, 4, 0),
    (8, 4, 0), (10, 4, 0), (12, 4, 0), (14, 4, 0), (16, 4, 0), (18, 4, 0), (20, 4, 0), (22, 4, 0),
    (24, 4, 0), (4, 6, 0), (6, 6, 0), (8, 6, 0), (10, 6, 0), (12, 6, 0), (14, 6, 0), (16, 6, 0),
    (18, 6, 0), (20, 6, 0), (22, 6, 0), (24, 6, 0), (26, 6, 0), (0, 7, 0), (2, 7, 0), (28, 7, 0),
    (4, 8, 0), (6, 8, 0), (8, 8, 0), (10, 8, 0), (12, 8, 0), (14, 8, 0), (16, 8, 0), (18, 8, 0),
    (20, 8, 0), (22, 8, 0), (24, 8, 0), (26, 8, 0), (6, 10, 0), (8, 10, 0), (10, 10, 0),
    (12, 10, 0), (14, 10, 0), (16, 10, 0), (18, 10, 0), (20, 10, 0), (22, 10, 0), (24, 10, 0),
    (8, 12, 0), (10, 12, 0), (12, 12, 0), (14, 12, 0), (16, 12, 0), (18, 12, 0), (20, 12, 0),
    (22, 12, 0), (4, 14, 0), (6, 14, 0), (8, 14, 0), (10, 14, 0), (12, 14, 0), (14, 14, 0),
    (16, 14, 0), (18, 14, 0), (20, 14, 0), (22, 14, 0), (24, 14, 0), (26, 14, 0), (10, 2, 1),
    (12, 2, 1), (14, 2, 1), (16, 2, 1), (18, 2, 1), (20, 2, 1), (10, 4, 1), (12, 4, 1), (14, 4, 1),
    (16, 4, 1), (18, 4, 1), (20, 4, 1), (10, 6, 1), (12, 6, 1), (14, 6, 1), (16, 6, 1), (18, 6, 1),
    (20, 6, 1), (10, 8, 1), (12, 8, 1), (14, 8, 1), (16, 8, 1), (18, 8, 1), (20, 8, 1), (10, 10, 1),
    (12, 10, 1), (14, 10, 1), (16, 10, 1), (18, 10, 1), (20, 10, 1), (10, 12, 1), (12, 12, 1),
    (14, 12, 1), (16, 12, 1), (18, 12, 1), (20, 12, 1), (12, 4, 2), (14, 4, 2), (16, 4, 2),
    (18, 4, 2), (12, 6, 2), (14, 6, 2), (16, 6, 2), (18, 6, 2), (12, 8, 2), (14, 8, 2), (16, 8, 2),
    (18, 8, 2), (12, 10, 2), (14, 10, 2), (16, 10, 2), (18, 10, 2), (14, 6, 3), (16, 6, 3),
    (14, 8, 3), (16, 8, 3), (15, 7, 4)];
