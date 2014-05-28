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

use board::{Board, Empty, White, Black, TrompTaylor, Minimal};
use board::coord::Coord;

#[test]
fn getting_a_valid_coord_returns_a_color(){
  let b = Board::new(19, 6.5, TrompTaylor);

  assert_eq!(b.get(10,10), Empty);
}

#[test]
#[should_fail]
fn getting_invalid_coordinates_fails() {
  let b = Board::new(19, 6.5, TrompTaylor);
  b.get(14, 21);
  b.get(21, 14);
}

#[test]
fn _19_19_is_a_valid_coordinate(){
  let b = Board::new(19, 6.5, TrompTaylor);

  assert_eq!(b.get(19, 19), Empty);
}

#[test]
#[should_fail]
fn _0_0_is_not_a_valid_coordinate(){
  let b = Board::new(19, 6.5, TrompTaylor);

  b.get(0, 0);
}

#[test]
fn get_komi(){
  let b = Board::new(19, 6.5, TrompTaylor);

  assert_eq!(b.komi(), 6.5f32)
}

#[test]
fn play_adds_a_stone_to_the_correct_position() {
  let mut b = Board::new(19, 6.5, TrompTaylor); 

  b = b.play(Black, Some((14, 14))).unwrap();

  assert!(b.get(14, 14) == Black);

  for i in range(1u8, 20) {
    for j in range(1u8 , 20) {
      assert!(b.get(i, j) == Empty || (i == 14 && j == 14));
    }
  }
}

#[test]
fn playing_on_an_illegal_coordinate_should_return_error() {
  let b = Board::new(9, 6.5, Minimal);

  assert!(b.play(Black, Some((13, 13))).is_err());
}

#[test]
fn playing_on_a_non_empty_intersection_should_return_error() {
  let b = Board::new(9, 6.5, Minimal);

  let b = b.play(Black, Some((4, 4))).unwrap();
  assert!(b.play(Black, Some((4, 4))).is_err());
  assert!(b.play(White, Some((4, 4))).is_err());
}

#[test]
fn two_way_merging_works() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(White, Some((10, 10))).unwrap();
  b = b.play(White, Some((10, 12))).unwrap();

  assert_eq!(b.chains.len(), 3);

  b = b.play(White, Some((10, 11))).unwrap();
  let c_id = b.get_chain(Coord::new(10, 10)).id;

  assert_eq!(b.get_chain(Coord::new(10, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(10, 12)).id, c_id);
  assert_eq!(b.chains.len(), 2)
}

#[test]
fn three_way_merging_works() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(White, Some((10, 10))).unwrap();
  b = b.play(White, Some((11, 11))).unwrap();
  b = b.play(White, Some((10, 12))).unwrap();

  assert_eq!(b.chains.len(), 4);

  b = b.play(White, Some((10, 11))).unwrap();
  let c_id = b.get_chain(Coord::new(10, 10)).id;

  assert_eq!(b.get_chain(Coord::new(10, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(11, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(10, 12)).id, c_id);
  assert_eq!(b.chains.len(), 2)
}

#[test]
fn four_way_merging_works() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(White, Some((10, 10))).unwrap();
  b = b.play(White, Some(( 9, 11))).unwrap();
  b = b.play(White, Some((11, 11))).unwrap();
  b = b.play(White, Some((10, 12))).unwrap();

  assert_eq!(b.chains.len(), 5);

  b = b.play(White, Some((10, 11))).unwrap();
  let c_id = b.get_chain(Coord::new(10, 10)).id;

  assert_eq!(b.get_chain(Coord::new(10, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(9 , 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(11, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(10, 12)).id, c_id);
  assert_eq!(b.chains.len(), 2)
}

#[test]
fn playing_on_all_libs_in_corner_should_capture() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(Black, Some((1, 1))).unwrap();
  b = b.play(White, Some((1, 2))).unwrap();
  b = b.play(White, Some((2, 1))).unwrap();

  assert_eq!(b.get(1, 1), Empty);
  assert_eq!(b.get(1, 2), White);
  assert_eq!(b.get(2, 1), White);
}

#[test]
fn playing_on_all_libs_on_side_should_capture() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(Black, Some((1, 3))).unwrap();
  b = b.play(White, Some((1, 2))).unwrap();
  b = b.play(White, Some((1, 4))).unwrap();
  b = b.play(White, Some((2, 3))).unwrap();

  assert_eq!(b.get(1, 3), Empty);
  assert_eq!(b.get(1, 2), White);
  assert_eq!(b.get(1, 4), White);
  assert_eq!(b.get(2, 3), White);
}

#[test]
fn playing_on_all_libs_should_capture() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(Black, Some((4, 4))).unwrap();

  b = b.play(White, Some((4, 3))).unwrap();
  b = b.play(White, Some((4, 5))).unwrap();
  b = b.play(White, Some((3, 4))).unwrap();
  b = b.play(White, Some((5, 4))).unwrap();

  assert_eq!(b.get(4, 4), Empty);

  assert_eq!(b.get(4, 3), White);
  assert_eq!(b.get(4, 5), White);
  assert_eq!(b.get(3, 4), White);
  assert_eq!(b.get(5, 4), White);
}

#[test]
fn playing_on_all_libs_of_a_chain_should_capture() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(Black, Some((4, 4))).unwrap();
  b = b.play(Black, Some((4, 5))).unwrap();

  b = b.play(White, Some((4, 3))).unwrap();
  b = b.play(White, Some((3, 4))).unwrap();
  b = b.play(White, Some((5, 4))).unwrap();
  b = b.play(White, Some((3, 5))).unwrap();
  b = b.play(White, Some((5, 5))).unwrap();
  b = b.play(White, Some((4, 6))).unwrap();

  assert_eq!(b.get(4, 4), Empty);
  assert_eq!(b.get(4, 5), Empty);

  assert_eq!(b.get(4, 3), White);
  assert_eq!(b.get(3, 4), White);
  assert_eq!(b.get(5, 4), White);
  assert_eq!(b.get(3, 5), White);
  assert_eq!(b.get(5, 5), White);
  assert_eq!(b.get(4, 6), White);
}

#[test]
fn playing_on_all_libs_of_a_bent_chain_should_capture() {
  let mut b = Board::new(19, 6.5, Minimal);

  b = b.play(Black, Some((4, 4))).unwrap();
  b = b.play(Black, Some((4, 5))).unwrap();
  b = b.play(Black, Some((3, 4))).unwrap();

  b = b.play(White, Some((3, 3))).unwrap();
  b = b.play(White, Some((4, 3))).unwrap();
  b = b.play(White, Some((2, 4))).unwrap();
  b = b.play(White, Some((5, 4))).unwrap();
  b = b.play(White, Some((3, 5))).unwrap();
  b = b.play(White, Some((5, 5))).unwrap();
  b = b.play(White, Some((4, 6))).unwrap();

  assert_eq!(b.get(4, 4), Empty);
  assert_eq!(b.get(4, 5), Empty);
  assert_eq!(b.get(3, 4), Empty);

  assert_eq!(b.get(3, 3), White);
  assert_eq!(b.get(4, 3), White);
  assert_eq!(b.get(2, 4), White);
  assert_eq!(b.get(5, 4), White);
  assert_eq!(b.get(3, 5), White);
  assert_eq!(b.get(5, 5), White);
  assert_eq!(b.get(4, 6), White);
}

#[test]
fn suicide_should_be_legal_in_tromp_taylor_rules() {
  let mut b = Board::new(19, 6.5, TrompTaylor);

  b = b.play(Black, Some((4 , 4 ))).unwrap();
  b = b.play(White, Some((16, 16))).unwrap();
  b = b.play(Black, Some((3 , 3 ))).unwrap();
  b = b.play(White, Some((16, 10))).unwrap();
  b = b.play(Black, Some((2 , 4 ))).unwrap();
  b = b.play(White, Some((16, 4 ))).unwrap();
  b = b.play(Black, Some((3 , 5 ))).unwrap();

  assert!(b.play(White, Some((3, 4))).is_ok());
}

#[test]
fn suicide_should_remove_the_suicided_chain() {
  let mut b = Board::new(19, 6.5, TrompTaylor);

  b = b.play(Black, Some((4 , 4 ))).unwrap();
  b = b.play(White, Some((16, 16))).unwrap();
  b = b.play(Black, Some((3 , 3 ))).unwrap();
  b = b.play(White, Some((16, 10))).unwrap();
  b = b.play(Black, Some((2 , 4 ))).unwrap();
  b = b.play(White, Some((16, 4 ))).unwrap();
  b = b.play(Black, Some((3 , 5 ))).unwrap();

  b = b.play(White, Some((3, 4))).unwrap();

  assert_eq!(b.get(3, 4), Empty);

  assert_eq!(b.get(4, 4), Black);
  assert_eq!(b.get(3, 3), Black);
  assert_eq!(b.get(2, 4), Black);
  assert_eq!(b.get(3, 5), Black);
}

#[test]
fn playing_twice_should_be_illegal_in_tromp_taylor_rules() {
  let mut b = Board::new(19, 6.5, TrompTaylor);
  
  b = b.play(Black, Some((10, 10))).unwrap();

  assert!(b.play(Black, Some((4, 4))).is_err());
}

#[test]
#[should_fail]
fn the_only_valid_size_in_TT_rules_should_be_19() {
  let b = Board::new(13, 6.5, TrompTaylor);
  let b = Board::new(9, 6.5, TrompTaylor);
  let b = Board::new(21, 6.5, TrompTaylor);
  let b = Board::new(5, 6.5, TrompTaylor);
}

#[test]
fn after_two_passes_the_game_should_be_over_in_TT_rules() {
  let mut b = Board::new(19, 6.5, TrompTaylor);

  let b = b.play(Black, None).unwrap();
  let b = b.play(White, None).unwrap();
  assert!(b.is_game_over()); 

  assert!(b.play(Black, Some((4, 4))).is_err());
}
