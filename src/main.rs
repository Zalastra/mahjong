#[allow(unused_variables, dead_code, unused_imports)]

mod tile;
mod board;

use board::Board;
use tile::Position;
use tile::Type::{Circle};
use tile::Nonary::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine};

fn main() {
    let positions = vec![(Position::new(0, 0, 0), Circle(One)),
                        (Position::new(0, 2, 0), Circle(Two)),
                        (Position::new(0, 4, 0), Circle(Three)),
                        (Position::new(0, 2, 1), Circle(Four)),];
    let b = Board::new(positions);
    for tile in b.tiles.values() {
        tile.print();
        println!("");
    }
    println!("\n\n");
    for key in b.reachable_tiles {
        b.tiles.get(&key).expect("").print();
        println!("");
    }
}

// TODO: figure out the best way to implement and add tests
#[cfg(test)]
mod tests {
    use tile::Position;

    #[test]
    fn position_to_key() {
        let pos = Position::new(1, 1, 1);
        let key = pos.to_key();
        assert!(key == 0x010101)
    }
}
