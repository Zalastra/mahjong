use std::cell::Cell;
use std::fmt;
use std::fmt::Write;

use sdl2::render::{Renderer};
use sdl2::rect::Rect;

use tile::*;

pub struct Board {
    height: u8,
    width: u8,
    tiles: Vec<Tile>,
    played: Vec<usize>,
    blocking_data: Vec<TileBlockingData>,
    reachable_tiles: Vec<usize>,
    selected_tile: Option<usize>,
}

struct TileBlockingData {
    pub tile_index: usize,
    pub blocking_verticaly: Vec<usize>,
    pub blocking_right: Vec<usize>,
    pub blocking_left: Vec<usize>,
    pub blocked_by_verticaly: Cell<u8>,
    pub blocked_by_left: Cell<u8>,
    pub blocked_by_right: Cell<u8>,
}

enum Blocking {
    Verticaly, Right, Left,
}

use self::Blocking::{Left, Right, Verticaly};

impl TileBlockingData {
    pub fn get_blocking(&self, blocking: Blocking) -> &Vec<usize> {
        match blocking {
            Left => &self.blocking_left,
            Right => &self.blocking_right,
            Verticaly => &self.blocking_verticaly,
        }
    }
}

fn generate_blocking_data(tiles: &Vec<Tile>) -> Vec<TileBlockingData> {
    macro_rules! blocking_tiles_on {
        ( $( { $x:expr, $y:expr, $z:expr } ),+ ) => {
            tiles.iter()
                 .enumerate()
                 .filter(|&(_, other_tile)| {
                     let positions = [ $( TilePosition { x: $x, y: $y, z: $z }, )+ ];
                     positions.iter().any(|position| other_tile.is_on_position(position))
                 })
                 .map(|(index, _)| index)
                 .collect()
        }
    }

    let mut blocking_data = Vec::new();
    for (index, tile) in tiles.iter().enumerate() {
        let position = &tile.position;
        let blocking_verticaly = match position.z {
            0 => Vec::new(),
            _ => blocking_tiles_on![{position.x,     position.y,     position.z - 1},
                                    {position.x + 1, position.y,     position.z - 1},
                                    {position.x,     position.y + 1, position.z - 1},
                                    {position.x + 1, position.y + 1, position.z - 1}]
        };

        let blocking_left = match position.x {
            0 => Vec::new(),
            _ => blocking_tiles_on![{position.x - 1, position.y,     position.z},
                                    {position.x - 1, position.y + 1, position.z}]
        };

        let blocking_right =
            blocking_tiles_on![{position.x + 2, position.y,     position.z},
                               {position.x + 2, position.y + 1, position.z}];

        blocking_data.push(TileBlockingData {
            tile_index: index,
            blocking_verticaly: blocking_verticaly,
            blocking_left: blocking_left,
            blocking_right: blocking_right,
            blocked_by_verticaly: Cell::new(0),
            blocked_by_left: Cell::new(0),
            blocked_by_right: Cell::new(0),
        })
    }


    blocking_data
}

impl Board {
    pub fn new(renderer: &Renderer) -> Board {
        let mut tiles = Vec::new();
        let (mut height, mut width) = (0, 0);

        let mut tile_factory = TileFactory::new(renderer);

        while let Some(tile) = tile_factory.get_random_tile() {
            if tile.position.x >= width { width = tile.position.x + 1; }
            if tile.position.y >= height { height = tile.position.y + 1; }

            tiles.push(tile);
        }

        let blocking_data = generate_blocking_data(&tiles);

        let mut board = Board {
            height: height,
            width: width,
            tiles: tiles,
            played: Vec::new(),
            blocking_data: blocking_data,
            reachable_tiles: Vec::new(),
            selected_tile: None,
        };

        board.update_blocked_by_data();
        board.set_reachable_tiles();

        board
    }

    pub fn select_tile(&mut self, mouse_x: i32, mouse_y: i32) {
        let mut tile_index = None;

        for (index, tile) in self.tiles.iter().enumerate() {
            if self.played.contains(&index) || !self.reachable_tiles.contains(&index) { continue; }

            let x = tile.position.x as i32 * 23 + tile.position.z as i32 * 5 + 15;
            let y = tile.position.y as i32 * 29 - tile.position.z as i32 * 5 + 15;

            if mouse_x >= x && mouse_x <= x + 46 && mouse_y >= y && mouse_y <= y + 57 {
                tile_index = Some(index);
                break;
            }
        }

        if let Some(tile_index) = tile_index {
            match self.selected_tile {
                Some(tile_index2) => {
                    if tile_index == tile_index2 {
                        self.tiles[tile_index2].texture.set_color_mod(255, 255, 255);
                        self.selected_tile = None;
                        return;
                    }
                    {
                        let tile1 = &self.tiles[tile_index];
                        let tile2 = &self.tiles[tile_index2];

                        if !Tile::matches(&tile1, &tile2) {
                            return;
                        }
                    }
                    self.played.push(tile_index);
                    self.played.push(tile_index2);
                    self.update_blocked_by_data();
                    self.set_reachable_tiles();

                    self.tiles[tile_index2].texture.set_color_mod(255, 255, 255);
                    self.selected_tile = None;
                },
                None => {
                    self.tiles[tile_index].texture.set_color_mod(255, 127, 127);
                    self.selected_tile = Some(tile_index);
                }
            }
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        for (index, tile) in self.tiles.iter().enumerate() {
            if self.played.contains(&index) { continue; }

            let x = tile.position.x as i32 * 23 + tile.position.z as i32 * 5 + 15;
            let y = tile.position.y as i32 * 29 - tile.position.z as i32 * 5 + 15;

            renderer.copy(&tile.texture, None, Rect::new(x, y, 46, 57).unwrap())
        }
    }

    fn update_blocked_by_data(&self) {
        for data in self.blocking_data.iter() {
            let index = data.tile_index;

            macro_rules! amount_blocked_by {
                ( $b:expr ) => {
                    self.blocking_data
                        .iter()
                        .filter(|&data| !self.played.contains(&data.tile_index))
                        .fold(0, |count, data| {
                            if data.get_blocking($b).contains(&index) {
                                count + 1
                            } else {
                                count
                            }
                        })
                }
            }

            data.blocked_by_verticaly.set(amount_blocked_by!(Verticaly));
            data.blocked_by_left.set(amount_blocked_by!(Left));
            data.blocked_by_right.set(amount_blocked_by!(Right));
        }
    }

    fn set_reachable_tiles(&mut self) {
        self.reachable_tiles = self.blocking_data
            .iter()
            .filter(|&data| {
                !self.played.contains(&data.tile_index) &&
                data.blocked_by_verticaly.get() == 0 &&
                (data.blocked_by_left.get() == 0 ||
                data.blocked_by_right.get() == 0)
            })
            .map(|data| data.tile_index)
            .collect();
    }

    // TODO: remove once we remove console printing
    fn get_top_tile_index_at_position(&self, x: u8, y: u8) -> Option<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|&(index, tile)| {
                tile.position.x == x &&
                tile.position.y == y &&
                !self.played.contains(&index)
            })
            .fold(None, |top_tile_index, (index, tile)| {
                if let Some(index) = top_tile_index {
                    let current_top_tile: &Tile = &self.tiles[index];
                    if current_top_tile.position.z > tile.position.z {
                        return top_tile_index
                    }
                }
                Some(index)
            })
    }
}

// TODO: should be removed if we dont need console printing for debugging
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("   ");
        for column in 0..self.width {
            output.push_str(&format!("{: >3} ", column));
        }
        output.push('\n');
        for _ in 0..self.width+1 {
            output.push_str("----");
        }
        output.push('\n');
        for row in 0..self.height {
            output.push_str(&format!("{: >2}|", row));
            for column in 0..self.width {
                if let Some(index) = self.get_top_tile_index_at_position(column, row) {
                    output.push_str(&format!("{}", self.tiles[index]));
                } else {
                    output.push_str("    ");
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}
