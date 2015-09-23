extern crate rand;

#[macro_use]
extern crate iterable_enum;

mod tile;
mod board;

use std::io;
use std::num::ParseIntError;

use board::{Board, BoardPosition};

// TODO: handle input properly
fn main() {
    let mut board = Board::new();
    loop {
        println!("{}", board);

        println!("Give the positions of the two tiles:");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("unrecoverable error trying to read from stdin");
        let input: Vec<&str> = buffer.trim().split(' ').collect();
        if input.len() != 4 {
            panic!("wrong input: incorrect <{}> amount of inputs", input.len());
        }
        let parsed_input: Vec<Result<u8, ParseIntError>> =
            input.iter().map(|&input_part| input_part.parse::<u8>()).collect();
        if parsed_input.iter().any(|parsed_input_part| parsed_input_part.is_err()) {
            panic!("wrong input: non-number input detected");
        }
        let parsed_input: Vec<u8> = parsed_input.iter().cloned().map(|input| input.unwrap()).collect();
        let position1 = BoardPosition { x: parsed_input[0], y: parsed_input[1] };
        let position2 = BoardPosition { x: parsed_input[2], y: parsed_input[3] };
        board.make_match(position1, position2);
    }
}
