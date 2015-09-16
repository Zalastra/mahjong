#[allow(unused_variables, dead_code, unused_imports)]

extern crate rand;

mod tiles;
mod board;

use board::Board;

fn main() {
    let board = Board::new();
    for tile in board.tiles.iter() {
        tile.print();
        println!("");
    }
}
