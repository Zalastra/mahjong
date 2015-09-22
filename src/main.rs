#![feature(associated_consts)]

extern crate rand;

#[macro_use]
extern crate iterable_enum;

mod tile;
mod board;

type A = std::slice::Iter<'static, i32>;

use board::Board;

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    println!("{}", board.make_match(4, 0, 8, 2));
}
