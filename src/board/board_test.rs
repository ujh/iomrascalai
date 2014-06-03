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
use board::{SuperKoRuleBroken};
use board::coord::Coord;
use board::hash::ZobristHashTable;
use board::move::{Play, Pass};

#[test]
fn getting_a_valid_coord_returns_a_color() {
  let zht = ZobristHashTable::new(19);
  let b = Board::new(19, 6.5, TrompTaylor, &zht);

  assert_eq!(b.get(10,10), Empty);
}

#[test]
#[should_fail]
fn getting_invalid_coordinates_fails() {
  let zht = ZobristHashTable::new(19);
  let b = Board::new(19, 6.5, TrompTaylor, &zht);
  
  b.get(14, 21);
  b.get(21, 14);
}

#[test]
fn _19_19_is_a_valid_coordinate(){
  let zht = ZobristHashTable::new(19);
  let b = Board::new(19, 6.5, TrompTaylor, &zht);

  assert_eq!(b.get(19, 19), Empty);
}

#[test]
#[should_fail]
fn _0_0_is_not_a_valid_coordinate(){
  let zht = ZobristHashTable::new(19);
  let b = Board::new(19, 6.5, TrompTaylor, &zht);

  b.get(0, 0);
}

#[test]
fn get_komi(){
  let zht = ZobristHashTable::new(19);
  let b = Board::new(19, 6.5, TrompTaylor, &zht);

  assert_eq!(b.komi(), 6.5f32)
}

#[test]
fn play_adds_a_stone_to_the_correct_position() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, TrompTaylor, &zht); 
  
  b = b.play(Play(Black, 14, 14)).unwrap();

  assert!(b.get(14, 14) == Black);

  for i in range(1u8, 20) {
    for j in range(1u8 , 20) {
      assert!(b.get(i, j) == Empty || (i == 14 && j == 14));
    }
  }
}

#[test]
fn playing_on_an_illegal_coordinate_should_return_error() {
  let zht = ZobristHashTable::new(19);
  let b = Board::new(9, 6.5, Minimal, &zht);

  assert!(b.play(Play(Black, 13, 13)).is_err());
}

#[test]
fn playing_on_a_non_empty_intersection_should_return_error() {
  let zht = ZobristHashTable::new(19);
  let b = Board::new(9, 6.5, Minimal, &zht);

  let b = b.play(Play(Black, 4, 4)).unwrap();
  assert!(b.play(Play(Black, 4, 4)).is_err());
  assert!(b.play(Play(White, 4, 4)).is_err());
}

#[test]
fn two_way_merging_works() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(White, 10, 10)).unwrap();
  b = b.play(Play(White, 10, 12)).unwrap();

  assert_eq!(b.chains.len(), 3);

  b = b.play(Play(White, 10, 11)).unwrap();
  let c_id = b.get_chain(Coord::new(10, 10)).id;

  assert_eq!(b.get_chain(Coord::new(10, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(10, 12)).id, c_id);
  assert_eq!(b.chains.len(), 2)
}

#[test]
fn three_way_merging_works() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(White, 10, 10)).unwrap();
  b = b.play(Play(White, 11, 11)).unwrap();
  b = b.play(Play(White, 10, 12)).unwrap();

  assert_eq!(b.chains.len(), 4);

  b = b.play(Play(White, 10, 11)).unwrap();
  let c_id = b.get_chain(Coord::new(10, 10)).id;

  assert_eq!(b.get_chain(Coord::new(10, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(11, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(10, 12)).id, c_id);
  assert_eq!(b.chains.len(), 2)
}

#[test]
fn four_way_merging_works() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(White, 10, 10)).unwrap();
  b = b.play(Play(White, 9, 11)).unwrap();
  b = b.play(Play(White, 11, 11)).unwrap();
  b = b.play(Play(White, 10, 12)).unwrap();

  assert_eq!(b.chains.len(), 5);

  b = b.play(Play(White, 10, 11)).unwrap();
  let c_id = b.get_chain(Coord::new(10, 10)).id;

  assert_eq!(b.get_chain(Coord::new(10, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(9 , 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(11, 11)).id, c_id);
  assert_eq!(b.get_chain(Coord::new(10, 12)).id, c_id);
  assert_eq!(b.chains.len(), 2)
}

#[test]
fn playing_on_all_libs_in_corner_should_capture() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(Black, 1, 1)).unwrap();
  b = b.play(Play(White, 1, 2)).unwrap();
  b = b.play(Play(White, 2, 1)).unwrap();

  assert_eq!(b.get(1, 1), Empty);
  assert_eq!(b.get(1, 2), White);
  assert_eq!(b.get(2, 1), White);
}

#[test]
fn playing_on_all_libs_on_side_should_capture() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(Black, 1, 3)).unwrap();
  b = b.play(Play(White, 1, 2)).unwrap();
  b = b.play(Play(White, 1, 4)).unwrap();
  b = b.play(Play(White, 2, 3)).unwrap();

  assert_eq!(b.get(1, 3), Empty);
  assert_eq!(b.get(1, 2), White);
  assert_eq!(b.get(1, 4), White);
  assert_eq!(b.get(2, 3), White);
}

#[test]
fn playing_on_all_libs_should_capture() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(Black, 4, 4)).unwrap();

  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(White, 4, 5)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();

  assert_eq!(b.get(4, 4), Empty);

  assert_eq!(b.get(4, 3), White);
  assert_eq!(b.get(4, 5), White);
  assert_eq!(b.get(3, 4), White);
  assert_eq!(b.get(5, 4), White);
}

#[test]
fn playing_on_all_libs_of_a_chain_should_capture() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(Black, 4, 5)).unwrap();

  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();
  b = b.play(Play(White, 3, 5)).unwrap();
  b = b.play(Play(White, 5, 5)).unwrap();
  b = b.play(Play(White, 4, 6)).unwrap();

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
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, Minimal, &zht);

  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(Black, 4, 5)).unwrap();
  b = b.play(Play(Black, 3, 4)).unwrap();

  b = b.play(Play(White, 3, 3)).unwrap();
  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(White, 2, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();
  b = b.play(Play(White, 3, 5)).unwrap();
  b = b.play(Play(White, 5, 5)).unwrap();
  b = b.play(Play(White, 4, 6)).unwrap();

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
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, TrompTaylor, &zht);

  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();
  b = b.play(Play(Black, 16, 16)).unwrap();
  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(Black, 16, 15)).unwrap();
  b = b.play(Play(White, 3, 3)).unwrap();
  b = b.play(Play(Black, 16, 14)).unwrap();
  b = b.play(Play(White, 2, 4)).unwrap();
  b = b.play(Play(Black, 16, 13)).unwrap();
  b = b.play(Play(White, 4, 5)).unwrap();
  b = b.play(Play(Black, 16, 12)).unwrap();
  b = b.play(Play(White, 3, 5)).unwrap();

  assert!(b.play(Play(Black, 3, 4)).is_ok());
}

#[test]
fn suicide_should_remove_the_suicided_chain() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, TrompTaylor, &zht);

  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();
  b = b.play(Play(Black, 16, 16)).unwrap();
  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(Black, 16, 15)).unwrap();
  b = b.play(Play(White, 3, 3)).unwrap();
  b = b.play(Play(Black, 16, 14)).unwrap();
  b = b.play(Play(White, 2, 4)).unwrap();
  b = b.play(Play(Black, 16, 13)).unwrap();
  b = b.play(Play(White, 4, 5)).unwrap();
  b = b.play(Play(Black, 16, 12)).unwrap();
  b = b.play(Play(White, 3, 5)).unwrap();

  b = b.play(Play(Black, 3, 4)).unwrap();

  assert_eq!(b.get(3, 4), Empty);
  assert_eq!(b.get(4, 4), Empty);

  assert_eq!(b.get(5, 4), White);
  assert_eq!(b.get(4, 3), White);
  assert_eq!(b.get(3, 3), White);
  assert_eq!(b.get(2, 4), White);
  assert_eq!(b.get(4, 5), White);
  assert_eq!(b.get(3, 5), White);
}

#[test]
fn playing_twice_should_be_illegal_in_tromp_taylor_rules() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, TrompTaylor, &zht);
  
  b = b.play(Play(Black, 10, 10)).unwrap();

  assert!(b.play(Play(Black, 4, 4)).is_err());
}

#[test]
#[should_fail]
fn the_only_valid_size_in_TT_rules_should_be_19() {
  let zht = ZobristHashTable::new(19);
  let b = Board::new(13, 6.5, TrompTaylor, &zht);
  let b = Board::new(9, 6.5, TrompTaylor, &zht);
  let b = Board::new(21, 6.5, TrompTaylor, &zht);
  let b = Board::new(5, 6.5, TrompTaylor, &zht);
}

#[test]
fn after_two_passes_the_game_should_be_over_in_TT_rules() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, TrompTaylor, &zht);

  let b = b.play(Pass(Black)).unwrap();
  let b = b.play(Pass(White)).unwrap();
  assert!(b.is_game_over()); 

  assert!(b.play(Play(Black, 4, 4)).is_err());
}

#[test]
fn replaying_directly_on_a_ko_point_should_be_illegal() {
  let zht = ZobristHashTable::new(19);
  let mut b = Board::new(19, 6.5, TrompTaylor, &zht);
  
  let b = b.play(Play(Black, 4, 4)).unwrap();
  let b = b.play(Play(White, 5, 4)).unwrap();
  let b = b.play(Play(Black, 3, 3)).unwrap();
  let b = b.play(Play(White, 4, 3)).unwrap();
  let b = b.play(Play(Black, 3, 5)).unwrap();
  let b = b.play(Play(White, 4, 5)).unwrap();
  let b = b.play(Play(Black, 2, 4)).unwrap();
  let b = b.play(Play(White, 3, 4)).unwrap();

  match b.play(Play(Black, 4, 4)) {
    Err(SuperKoRuleBroken) => (),
    Ok(_)                  => fail!("Replaying on a ko was allowed"),
    Err(x)                 => fail!("Engine crashed while trying to replay on a ko : {}", x)
  }
}

#[test]
fn counting_simple_case() {
  let zht = ZobristHashTable::new(4);
  let mut b = Board::new(4, 6.5, Minimal, &zht);
  
  let b = b.play(Play(Black, 2, 1)).unwrap();
  let b = b.play(Play(White, 3, 1)).unwrap();
  let b = b.play(Play(Black, 2, 2)).unwrap();
  let b = b.play(Play(White, 3, 2)).unwrap();
  let b = b.play(Play(Black, 2, 3)).unwrap();
  let b = b.play(Play(White, 3, 3)).unwrap();
  let b = b.play(Play(Black, 2, 4)).unwrap();
  let b = b.play(Play(White, 3, 4)).unwrap();
  let b = b.play(Pass(Black)).unwrap();
  let b = b.play(Pass(White)).unwrap();

  let (b_score, w_score) = b.score();
  assert_eq!(b_score, 8);
  assert_eq!(w_score, 8f32 + 6.5);
}

#[test]
fn counting_disjoint_territory() {
  let size = 5;
  let komi = 6.5;

  let zht = ZobristHashTable::new(size);
  let mut b = Board::new(size, komi, Minimal, &zht);
  
  let b = b.play(Play(Black, 2, 1)).unwrap();
  let b = b.play(Play(White, 3, 1)).unwrap();
  let b = b.play(Play(Black, 2, 2)).unwrap();
  let b = b.play(Play(White, 3, 2)).unwrap();
  let b = b.play(Play(Black, 1, 3)).unwrap();
  let b = b.play(Play(White, 2, 3)).unwrap();
  let b = b.play(Play(Black, 5, 4)).unwrap();
  let b = b.play(Play(White, 1, 4)).unwrap();
  let b = b.play(Play(Black, 4, 4)).unwrap();
  let b = b.play(Play(White, 5, 3)).unwrap();
  let b = b.play(Play(Black, 4, 5)).unwrap();
  let b = b.play(Play(White, 4, 3)).unwrap();
  let b = b.play(Play(Black, 1, 2)).unwrap();
  let b = b.play(Play(White, 3, 4)).unwrap();
  let b = b.play(Pass(Black)).unwrap();
  let b = b.play(Play(White, 3, 5)).unwrap();
  let b = b.play(Pass(Black)).unwrap();
  let b = b.play(Pass(White)).unwrap();

  let (b_score, w_score) = b.score();
  assert_eq!(b_score, 9);
  assert_eq!(w_score, 16f32 + komi);
}

#[test]
fn counting_with_neutral_points() {
  let size = 5;
  let komi = 6.5;

  let zht = ZobristHashTable::new(size);
  let mut b = Board::new(size, komi, Minimal, &zht);
  
  let b = b.play(Play(Black, 2, 1)).unwrap();
  let b = b.play(Play(White, 3, 1)).unwrap();
  let b = b.play(Play(Black, 2, 2)).unwrap();
  let b = b.play(Play(White, 3, 2)).unwrap();
  let b = b.play(Play(Black, 1, 2)).unwrap();
  let b = b.play(Play(White, 2, 3)).unwrap();
  let b = b.play(Pass(Black)).unwrap();
  let b = b.play(Play(White, 1, 4)).unwrap();
  let b = b.play(Pass(Black)).unwrap();
  let b = b.play(Pass(White)).unwrap();

  let (b_score, w_score) = b.score();
  assert_eq!(b_score, 4);
  assert_eq!(w_score, 20f32 + komi);
}
