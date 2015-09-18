extern crate rand;

mod tile;
mod board;

use board::Board;

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    println!("{}", board.make_match(4, 0, 8, 2));
}
