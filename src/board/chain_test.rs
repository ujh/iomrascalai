#![cfg(test)]

use board::Black;
use board::chain::Chain;
use board::coord::Coord;

#[test]
fn add_stone_adds_a_stone() {
    let mut c1 = Chain::new(1, Black);
    c1.add_stone(Coord::new(14,14));

    assert_eq!(c1.coords().len(), 1);
}

#[test]
fn add_stone_adds_the_correct_stone() {
    let mut c1 = Chain::new(1, Black);
    c1.add_stone(Coord::new(14,14));

    assert_eq!(c1.coords().get(0), &Coord::new(14, 14));
}

#[test]
fn merge_adds_all_the_coords_of_the_merged_chain() {
    let mut c1 = Chain::new(1, Black);
    let mut c2 = Chain::new(2, Black);

    c2.add_stone(Coord::new(10,10));
    c2.add_stone(Coord::new(10,11));
    c2.add_stone(Coord::new(11,10));

    c1.merge(&c2);

    assert_eq!(c1.coords().len(), 3);
    assert!(c1.coords().contains(&Coord::new(10,10)));
    assert!(c1.coords().contains(&Coord::new(10,11)));
    assert!(c1.coords().contains(&Coord::new(11,10)));
}

#[test]
fn merge_doesnt_remove_the_previous_stones() {
    let mut c1 = Chain::new(1, Black);
    let mut c2 = Chain::new(2, Black);

    c2.add_stone(Coord::new(10,10));
    c2.add_stone(Coord::new(10,11));

    c1.merge(&c2);
    c1.add_stone(Coord::new(7,7));
    c1.add_stone(Coord::new(7,8));
    c1.add_stone(Coord::new(7,9));

    assert!(c1.coords().contains(&Coord::new(7,7)));
    assert!(c1.coords().contains(&Coord::new(7,8)));
    assert!(c1.coords().contains(&Coord::new(7,9)));
}

#[test]
fn show_returns_a_legible_string_for_the_chain() {
    let mut c1 = Chain::new(1, Black);

    c1.add_stone(Coord::new(7,7));
    c1.add_stone(Coord::new(7,8));
    c1.add_stone(Coord::new(7,9));

    let expected = String::from_str("1, Black: 7,7|7,8|7,9|");
    assert_eq!(c1.show(), expected);
}