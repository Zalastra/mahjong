#[allow(unused_variables, dead_code, unused_imports)]

extern crate rand;

mod tile;
mod board;

use board::Board;
use tile::*;

fn main() {
    let positions = vec![tile::TilePosition::new(0, 0, 0),
                        tile::TilePosition::new(0, 2, 0),
                        tile::TilePosition::new(0, 4, 0),
                        tile::TilePosition::new(0, 2, 1),];
    let board = Board::new(&positions);
    for tile in board.tiles.iter() {
        tile.print();
        println!("");
    }
}

// TODO: figure out the best way to implement and add tests
// Start with putting this in its own file
#[cfg(test)]
mod tests {
    use tile::Position;

    #[test]
    fn position_to_key() {
        let position = Position::new(1, 1, 1);
        let key = position.to_key();
        assert!(key == 0x010101)
    }
}
