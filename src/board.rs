use std::cell::Cell;
use std::fmt;
use std::fmt::Write;

use tile::*;

pub struct Board {
    height: u8,
    width: u8,
    tiles: Vec<Tile>,
    played: Vec<usize>,
    blocking_data: Vec<TileBlockingData>,
    reachable_tiles: Vec<usize>,
}

pub struct BoardPosition {
    pub x: u8,
    pub y: u8,
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

impl Board {
    pub fn new() -> Board {
        let mut tiles = Vec::new();
        let (mut height, mut width) = (0, 0);

        let mut tile_factory = TileFactory::new();

        while let Some(tile) = tile_factory.get_random_tile() {
            if tile.position.x >= width { width = tile.position.x + 1; }
            if tile.position.y >= height { height = tile.position.y + 1; }

            tiles.push(tile);
        }

        let blocking_data = Board::generate_blocking_data(&tiles);

        let mut board = Board {
            height: height,
            width: width,
            tiles: tiles,
            played: Vec::new(),
            blocking_data: blocking_data,
            reachable_tiles: Vec::new(),
        };

        Board::update_blocked_by_data(&board);

        Board::set_reachable_tiles(&mut board);

        board
    }

    // TODO: make this more readable?
    fn generate_blocking_data(tiles: &Vec<Tile>) -> Vec<TileBlockingData> {
        let mut blocking_data = Vec::new();
        for (index, tile) in tiles.iter().enumerate() {
            let position = &tile.position;
            let blocking_verticaly = match position.z {
                0 => Vec::new(),
                _ => tiles.iter()
                          .enumerate()
                          .filter(|&(_, other_tile)| {
                              let position1 = TilePosition { x: position.x, y: position.y, z: position.z - 1 };
                              let position2 = TilePosition { x: position.x + 1, y: position.y, z: position.z - 1 };
                              let position3 = TilePosition { x: position.x, y: position.y + 1, z: position.z - 1 };
                              let position4 = TilePosition { x: position.x + 1, y: position.y + 1, z: position.z - 1 };

                              other_tile.is_on_position(position1) ||
                              other_tile.is_on_position(position2) ||
                              other_tile.is_on_position(position3) ||
                              other_tile.is_on_position(position4)
                          })
                          .map(|(index, _)| index)
                          .collect()
            };

            let blocking_left = match position.x {
                0 => Vec::new(),
                _ => tiles.iter()
                          .enumerate()
                          .filter(|&(_, other_tile)| {
                              let position1 = TilePosition {
                                  x: position.x - 1,
                                  y: position.y,
                                  z: position.z
                              };
                              let position2 = TilePosition {
                                  x: position.x - 1,
                                  y: position.y + 1,
                                  z: position.z
                              };

                              other_tile.is_on_position(position1) ||
                              other_tile.is_on_position(position2)
                          })
                          .map(|(index, _)| index)
                          .collect()
            };

            let blocking_right =
                tiles.iter()
                     .enumerate()
                     .filter(|&(_, other_tile)| {
                         let position1 = TilePosition {
                             x: position.x + 2,
                             y: position.y,
                             z: position.z
                         };
                         let position2 = TilePosition {
                             x: position.x + 2,
                             y: position.y + 1,
                             z: position.z
                         };

                         other_tile.is_on_position(position1) ||
                         other_tile.is_on_position(position2)
                     })
                     .map(|(index, _)| index)
                     .collect();


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

    // TODO: get rid of repetition, maybe use macro for this?
    fn update_blocked_by_data(&self) {
        for data in self.blocking_data.iter() {
            let index = data.tile_index;
            let blocked_by_verticaly =
                self.blocking_data.iter()
                                  .filter(|&data| self.played.contains(&data.tile_index))
                                  .fold(0, |count, data| {
                                      if data.blocking_verticaly.contains(&index) {
                                          count + 1
                                      } else {
                                          count
                                      }
                                  });

            let blocked_by_left =
                self.blocking_data.iter()
                                  .filter(|&data| self.played.contains(&data.tile_index))
                                  .fold(0, |count, data| {
                                      if data.blocking_left.contains(&index) {
                                          count + 1
                                      } else {
                                          count
                                      }
                                  });

            let blocked_by_right =
                self.blocking_data.iter()
                                  .filter(|&data| self.played.contains(&data.tile_index))
                                  .fold(0, |count, data| {
                                      if data.blocking_right.contains(&index) {
                                          count + 1
                                      } else {
                                          count
                                      }
                                  });

            data.blocked_by_verticaly.set(blocked_by_verticaly);
            data.blocked_by_left.set(blocked_by_left);
            data.blocked_by_right.set(blocked_by_right);
        }
    }

    fn set_reachable_tiles(&mut self) {
        for tile_blocking_data in self.blocking_data.iter() {
            if tile_blocking_data.blocked_by_verticaly.get() == 0 &&
               tile_blocking_data.blocked_by_left.get() == 0 ||
               tile_blocking_data.blocked_by_right.get() == 0 {
                self.reachable_tiles.push(tile_blocking_data.tile_index);
            }
        }
    }

    // TODO: refactor return type into Result
    // TODO: create position type for the arguments
    pub fn make_match(&mut self, position1: BoardPosition, position2: BoardPosition) -> bool {
        let tile1_index;
        let tile2_index;

        if let Some(index) = self.get_top_tile_index_at_position(position1) {
            tile1_index = index;
        } else {
            return false
        }

        if let Some(index) = self.get_top_tile_index_at_position(position2) {
            tile2_index = index;
        } else {
            return false
        }

        if !self.reachable_tiles.contains(&tile1_index) || !self.reachable_tiles.contains(&tile2_index) {
            return false
        }

        {
            let tile1 = &self.tiles[tile1_index];
            let tile2 = &self.tiles[tile2_index];

            if !Tile::matches(&tile1, &tile2) {
                return false
            }
        }

        self.played.push(tile1_index);
        self.played.push(tile2_index);
        self.update_blocked_by_data();
        self.set_reachable_tiles();

        true
    }

    // TODO: create position type for the arguments
    fn get_top_tile_index_at_position(&self, position: BoardPosition) -> Option<usize> {
        self.tiles.iter()
                  .enumerate()
                  .filter(|&(index, tile)| {
                      tile.position.x == position.x &&
                      tile.position.y == position.y &&
                      !self.played.contains(&index)
                  })
                  .fold(None, |top_tile_index, (index, tile)| {
                      if top_tile_index.is_some() {
                          let current_top_tile: &Tile = &self.tiles[top_tile_index.unwrap()];
                          if current_top_tile.position.z > tile.position.z {
                              return top_tile_index
                          }
                      }
                      Some(index)
                  })
    }
}

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
                if let Some(index) = self.get_top_tile_index_at_position(BoardPosition { x: column, y: row }) {
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
