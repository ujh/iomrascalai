#![cfg(test)]

use board::Black;
use board::Chain;
use board::Coord;

#[test]
fn show_returns_a_legible_string_for_the_chain() {
    let mut c = Chain::new(1, Black, Coord::new(7,7), vec!(Coord::new(1,1)));

    c.add_coord(Coord::new(7,8));
    c.add_coord(Coord::new(7,9));

    let expected = String::from_str("1  | Black, libs: HashSet {\"(1,1)\"}, stones: [\"(7,7)\", \"(7,8)\", \"(7,9)\"]");
    assert_eq!(c.show(), expected);
}
