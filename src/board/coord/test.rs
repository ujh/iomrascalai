/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 *                                                                      *
 * This file is part of Iomrascálaí.                                    *
 *                                                                      *
 * Iomrascálaí is free software: you can redistribute it and/or modify  *
 * it under the terms of the GNU General Public License as published by *
 * the Free Software Foundation, either version 3 of the License, or    *
 * (at your option) any later version.                                  *
 *                                                                      *
 * Iomrascálaí is distributed in the hope that it will be useful,       *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of       *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the        *
 * GNU General Public License for more details.                         *
 *                                                                      *
 * You should have received a copy of the GNU General Public License    *
 * along with Iomrascálaí.  If not, see <http://www.gnu.org/licenses/>. *
 *                                                                      *
 ************************************************************************/
#![cfg(test)]

use super::Coord;

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

#[test]
fn for_board_size_returns_the_right_number_of_coords() {
    let coords = Coord::for_board_size(3);
    assert_eq!(9, coords.len());
}

#[test]
fn for_board_size_sets_the_coordinates_correctly() {
    let coords = Coord::for_board_size(1);
    assert_eq!(coords[0], Coord::new(1,1));
}

#[test]
fn distance_to_border() {
    let size = 9;
    assert_eq!(0, Coord::new(1,5).distance_to_border(size));
    assert_eq!(0, Coord::new(5,1).distance_to_border(size));
    assert_eq!(0, Coord::new(9,5).distance_to_border(size));
    assert_eq!(0, Coord::new(5,9).distance_to_border(size));
    assert_eq!(1, Coord::new(2,5).distance_to_border(size));
    assert_eq!(1, Coord::new(8,6).distance_to_border(size));
}

#[test]
fn manhattan_distance_three_neighbours_middle_of_board() {
    let coord = Coord::new(5, 5);
    let size = 9;
    assert_eq!(24, coord.manhattan_distance_three_neighbours(size).len());
}

#[test]
fn manhattan_distance_three_neighbours_in_a_corner() {
    let coord = Coord::new(1, 1);
    let size = 9;
    assert_eq!(9, coord.manhattan_distance_three_neighbours(size).len());
}
