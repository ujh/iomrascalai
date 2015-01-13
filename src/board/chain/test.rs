#![cfg(test)]

use board::Black;
use board::Chain;
use board::Coord;

#[test]
fn show_returns_a_legible_string_for_the_chain() {
    let mut c1 = Chain::new(1, Black, Coord::new(7,7), vec!(Coord::new(1,1)));

    c1.merge(&Chain::new(2, Black, Coord::new(7,8), vec!()));
    c1.merge(&Chain::new(2, Black, Coord::new(7,9), vec!()));

    let expected = String::from_str("1  | Black, libs: HashSet {\"(1,1)\"}, stones: [\"(7,7)\", \"(7,8)\", \"(7,9)\"]");
    assert_eq!(c1.show(), expected);
}
