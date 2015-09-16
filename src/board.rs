use tiles::*;

pub struct Board {
    pub tiles: Vec<Tile>,
}

impl Board {
    pub fn new() -> Board {
        let mut tiles = Vec::new();
        let mut tile_factory = TileFactory::new();

        while let Some(tile) = tile_factory.get_random_tile() {
            tiles.push(tile);
        }

        Board {
            tiles: tiles,
        }
    }
}
