#![cfg(test)]

use board::coord::Coord;

#[test]
fn test_neighbours_contain_NSEW() {
  let n = Coord::new(10,10).neighbours();

  assert!(n.iter().find(|c| c.col == 10 && c.row == 9 ).is_some());
  assert!(n.iter().find(|c| c.col == 9  && c.row == 10).is_some());
  assert!(n.iter().find(|c| c.col == 10 && c.row == 11).is_some());
  assert!(n.iter().find(|c| c.col == 11 && c.row == 10).is_some());
}

#[test]
fn test_neighbours_do_not_contain_diagonals() {
  let n = Coord::new(10,10).neighbours();

  assert!(n.iter().find(|c| c.col == 11 && c.row == 11).is_none());
  assert!(n.iter().find(|c| c.col == 9  && c.row == 11).is_none());
  assert!(n.iter().find(|c| c.col == 11 && c.row == 9 ).is_none());
  assert!(n.iter().find(|c| c.col == 9  && c.row == 9 ).is_none());
}

#[test]
fn test_neighbours_do_not_contain_itself() {
  let n = Coord::new(10,10).neighbours();

  assert!(n.iter().find(|c| c.col == 10 && c.row == 10).is_none());
}
