#![cfg(test)]

use board::coord::Coord;

#[test]
fn test_neighbours_contain_n_s_e_w() {
  let n = Coord::new(10,10).neighbours(19);

  assert!(n.iter().find(|c| c.col == 10 && c.row == 9 ).is_some());
  assert!(n.iter().find(|c| c.col == 9  && c.row == 10).is_some());
  assert!(n.iter().find(|c| c.col == 10 && c.row == 11).is_some());
  assert!(n.iter().find(|c| c.col == 11 && c.row == 10).is_some());
}

#[test]
fn test_neighbours_do_not_contain_diagonals() {
  let n = Coord::new(10,10).neighbours(19);

  assert!(n.iter().find(|c| c.col == 11 && c.row == 11).is_none());
  assert!(n.iter().find(|c| c.col == 9  && c.row == 11).is_none());
  assert!(n.iter().find(|c| c.col == 11 && c.row == 9 ).is_none());
  assert!(n.iter().find(|c| c.col == 9  && c.row == 9 ).is_none());
}

#[test]
fn test_neighbours_do_not_contain_itself() {
  let n = Coord::new(10,10).neighbours(19);

  assert!(n.iter().find(|c| c.col == 10 && c.row == 10).is_none());
}

#[test]
fn is_inside_valid_coords_pass() {
  assert!(Coord::new(1,1).is_inside(19));
  assert!(Coord::new(19,19).is_inside(19));
  assert!(Coord::new(10,10).is_inside(19));
}

#[test]
fn is_inside_0_0_fails() {
  assert!(!Coord::new(0,0).is_inside(19));
}

#[test]
fn is_inside_invalid_coords_fail() {
  assert!(!Coord::new(4,21).is_inside(19));
  assert!(!Coord::new(21,4).is_inside(19));

  assert!(!Coord::new(18,18).is_inside(9));
}

#[test]
fn from_gtp_converts_correctly() {
  assert_eq!(Coord::new(10,10), Coord::from_gtp("K10"));
  assert_eq!(Coord::new(10,10), Coord::from_gtp("k10"));

  assert_eq!(Coord::new(16,15), Coord::from_gtp("Q15"));

  assert_eq!(Coord::new(1,1), Coord::from_gtp("A1"));
  assert_eq!(Coord::new(19,19), Coord::from_gtp("T19"));

  assert_eq!(Coord::new(9,10), Coord::from_gtp("J10"));
  assert_eq!(Coord::new(8,10), Coord::from_gtp("H10"));
}

#[test]
fn to_gtp_converts_correctly() {
  assert_eq!(Coord::new(10,10).to_gtp(), String::from_str("K10"));
  assert_eq!(Coord::new(16,15).to_gtp(), String::from_str("Q15"));
  assert_eq!(Coord::new(1,1).to_gtp(), String::from_str("A1"));
  assert_eq!(Coord::new(19,19).to_gtp(), String::from_str("T19"));
  assert_eq!(Coord::new(9,10).to_gtp(), String::from_str("J10"));
  assert_eq!(Coord::new(8,10).to_gtp(), String::from_str("H10"));
}
