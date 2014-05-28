#![cfg(test)]

use board::coord::Coord;

#[test]
fn test_neighbours_contain_NSEW() {
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
