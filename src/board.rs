use std::fmt;
use std::fmt::Write;

use tiles::*;

pub struct Board {
    height: u8,
    width: u8,
    depth: u8,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new() -> Board {
        let mut tiles = Vec::new();
        let (mut height, mut width, mut depth) = (0, 0, 0);

        let mut tile_factory = TileFactory::new();

        while let Some(tile) = tile_factory.get_random_tile() {
            if tile.position.x >= width { width = tile.position.x + 1; }
            if tile.position.y >= height { height = tile.position.y + 1; }
            if tile.position.z >= depth { depth = tile.position.z + 1; }

            tiles.push(tile);
        }

        Board {
            height: height,
            width: width,
            depth: depth,
            tiles: tiles,
        }
    }

    #[allow(unused_variables)]
    fn top_tile_at_position(&self, x: u8, y: u8) -> Option<&Tile> {
        let index = self.tiles.iter()
                              .enumerate()
                              .filter(|&(index, tile)| tile.position.x == x && tile.position.y == y)
                              .fold(None, |top_tile_index, (index, &ref tile)| {
                                  if top_tile_index.is_some() {
                                      let current_top_tile: &Tile = &self.tiles[top_tile_index.unwrap()];
                                      if current_top_tile.position.z > tile.position.z {
                                          return top_tile_index
                                      }
                                  }
                                  Some(index)
                              });
        if let Some(index) = index {
            return self.tiles.get(index);
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in 0..self.height {
            for column in 0..self.width {
                if let Some(tile) = self.top_tile_at_position(column, row) {
                    output.push_str(&format!("{}", tile));
                } else {
                    output.push_str("    ");
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}
