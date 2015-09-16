extern crate rand;

mod tiles;
mod board;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board);
}
