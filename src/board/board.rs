use std::cell::Cell;
use std::fmt;
use std::fmt::Write;
use std::path::Path;

use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;

use sdl2_image::LoadTexture;

use board::tile::{Position, Tile};
use board::factory::{TileFactory, FactoryError};

pub struct Board {
    height: u8,
    width: u8,
    tiles: Vec<Tile>,
    played: Vec<usize>,
    blocking_data: Vec<TileBlockingData>,
    reachable_tiles: Vec<usize>,
    selected_tile: Option<usize>,
    side_texture: Texture,
    bottom_texture: Texture,
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

enum BlockingDirection {
    Verticaly, Right, Left,
}

use self::BlockingDirection::{Left, Right, Verticaly};

impl TileBlockingData {
    fn get_blocking(&self, blocking: BlockingDirection) -> &Vec<usize> {
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
                     let positions = [ $( Position::new($x, $y, $z), )+ ];
                     positions.iter().any(|position| other_tile.is_on_position(position))
                 })
                 .map(|(index, _)| index)
                 .collect()
        }
    }

    let mut blocking_data = Vec::new();
    for (index, tile) in tiles.iter().enumerate() {
        let position = &tile.position;
        let blocking_verticaly = match position.z() {
            0 => Vec::new(),
            _ => blocking_tiles_on![{position.x(),     position.y(),     position.z() - 1},
                                    {position.x() + 1, position.y(),     position.z() - 1},
                                    {position.x(),     position.y() + 1, position.z() - 1},
                                    {position.x() + 1, position.y() + 1, position.z() - 1}]
        };

        let blocking_left = match position.x() {
            0 => Vec::new(),
            _ => blocking_tiles_on![{position.x() - 1, position.y(),     position.z()},
                                    {position.x() - 1, position.y() + 1, position.z()}]
        };

        let blocking_right =
            blocking_tiles_on![{position.x() + 2, position.y(),     position.z()},
                               {position.x() + 2, position.y() + 1, position.z()}];

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

        // NOTE: There is a (good) chance that valid board creation fails, it's easy to detect
        //       but hard to prevent so we just keep trying untill we have a valid board
        let mut done = false;
        while !done {
            let mut tile_factory = TileFactory::new();

            loop {
                let two_tiles = tile_factory.get_tile(&renderer);
                if let Ok((tile1, tile2)) = two_tiles {
                    if tile1.position.x() >= width { width = tile1.position.x() + 1; }
                    if tile1.position.y() >= height { height = tile1.position.y() + 1; }

                    tiles.push(tile1);

                    if tile2.position.x() >= width { width = tile2.position.x() + 1; }
                    if tile2.position.y() >= height { height = tile2.position.y() + 1; }

                    tiles.push(tile2);
                } else {
                    match two_tiles.err().unwrap() {
                        FactoryError::Empty => { done = true; },
                        FactoryError::InvalidBoard => {},
                    }
                    break;
                }
            }

            if !done { tiles.clear(); }
        }

        tiles.sort_by(|a, b| {
            use std::cmp::Ordering::*;
            if a.position.z() < b.position.z() { Less }
            else if a.position.z() > b.position.z() { Greater }
            else if a.position.x() > b.position.x() { Less }
            else if a.position.x() < b.position.x() { Greater }
            else if a.position.y() < b.position.y() { Less }
            else { Greater }
        });

        let blocking_data = generate_blocking_data(&tiles);

        let side_texture = renderer.load_texture(Path::new("img/TileSide.png")).expect("error loading side texture");
        let bottom_texture = renderer.load_texture(Path::new("img/TileBottom.png")).expect("error loading bottom texture");

        let mut board = Board {
            height: height,
            width: width,
            tiles: tiles,
            played: Vec::new(),
            blocking_data: blocking_data,
            reachable_tiles: Vec::new(),
            selected_tile: None,
            side_texture: side_texture,
            bottom_texture: bottom_texture,
        };

        board.update_meta_data();

        board
    }

    pub fn select_tile(&mut self, mouse_x: i32, mouse_y: i32) {
        let tile_index = self.find_tile_index_by_coord(mouse_x, mouse_y);

        if let Some(tile_index) = tile_index {
            match self.selected_tile {
                Some(tile_index2) => {
                    // deselect tile
                    if tile_index == tile_index2 {
                        self.tiles[tile_index2].texture.set_color_mod(255, 255, 255);
                        self.selected_tile = None;
                        return;
                    }

                    // test tile match
                    {
                        let tile1 = &self.tiles[tile_index];
                        let tile2 = &self.tiles[tile_index2];

                        if !Tile::matches(&tile1, &tile2) {
                            return;
                        }
                    }

                    // valid match
                    self.played.push(tile_index);
                    self.played.push(tile_index2);
                    self.update_meta_data();

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

    pub fn undo(&mut self) {
        self.played.pop();
        self.played.pop();
        self.update_meta_data();
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        for (index, tile) in self.tiles.iter().enumerate() {
            if self.played.contains(&index) { continue; }

            let x = tile.position.x() as i32 * 23 + tile.position.z() as i32 * 5 + 20;
            let y = tile.position.y() as i32 * 29 - tile.position.z() as i32 * 5 + 15;

            renderer.copy(&self.side_texture, None, Rect::new(x - 5, y, 5, 62).unwrap());
            renderer.copy(&self.bottom_texture, None, Rect::new(x, y + 57, 46, 5).unwrap());

            renderer.copy(&tile.texture, None, Rect::new(x, y, 46, 57).unwrap());
        }
    }

    fn find_tile_index_by_coord(&self, x: i32, y: i32) -> Option<usize> {
        for (index, tile) in self.tiles.iter().enumerate().rev() {
            if self.played.contains(&index) || !self.reachable_tiles.contains(&index) { continue; }

            let tile_x = tile.position.x() as i32 * 23 + tile.position.z() as i32 * 5 + 15;
            let tile_y = tile.position.y() as i32 * 29 - tile.position.z() as i32 * 5 + 15;

            if x >= tile_x && x <= tile_x + 46 && y >= tile_y && y <= tile_y + 57 {
                return Some(index);
            }
        }
        None
    }

    fn update_meta_data(&mut self) {
        self.update_blocked_by_data();
        self.set_reachable_tiles();
    }

    fn update_blocked_by_data(&self) {
        for data in self.blocking_data.iter() {
            let index = data.tile_index;

            macro_rules! amount_blocked_by {
                ( $blocking_direction:expr ) => {
                    self.blocking_data
                        .iter()
                        .filter(|&data| !self.played.contains(&data.tile_index))
                        .fold(0, |count, data| {
                            if data.get_blocking($blocking_direction).contains(&index) {
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
                tile.position.x() == x &&
                tile.position.y() == y &&
                !self.played.contains(&index)
            })
            .fold(None, |top_tile_index, (index, tile)| {
                if let Some(index) = top_tile_index {
                    let current_top_tile: &Tile = &self.tiles[index];
                    if current_top_tile.position.z() > tile.position.z() {
                        return top_tile_index
                    }
                }
                Some(index)
            })
    }
}

// TODO: should be removed if we dont need console printing for debugging
impl fmt::Debug for Board {
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
