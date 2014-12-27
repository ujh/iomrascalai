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

use board::Black;
use board::Board;
use board::Coord;
use board::Empty;
use board::IllegalMove;
use board::Pass;
use board::Play;
use board::White;
use board::ZobristHashTable;
use board::chain::Chain;
use ruleset::AnySizeTrompTaylor;
use ruleset::KgsChinese;
use ruleset::Minimal;

use std::rc::Rc;

#[test]
fn getting_a_valid_coord_returns_a_color() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  assert_eq!(b.get_coord(Coord::new(10, 10)), Empty);
}

#[test]
#[should_fail]
fn getting_invalid_coordinates_fails() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  b.get_coord(Coord::new(14, 21));
  b.get_coord(Coord::new(21, 14));
}

#[test]
fn _19_19_is_a_valid_coordinate(){
  let zht = Rc::new(ZobristHashTable::new(19));
  let b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  assert_eq!(b.get_coord(Coord::new(19, 19)), Empty);
}

#[test]
#[should_fail]
fn _0_0_is_not_a_valid_coordinate(){
  let zht = Rc::new(ZobristHashTable::new(19));
  let b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  b.get_coord(Coord::new(0, 0));
}

#[test]
fn play_adds_a_stone_to_the_correct_position() {
    let zht = Rc::new(ZobristHashTable::new(19));
    let mut b = Board::new(19, AnySizeTrompTaylor, zht.clone());

    b = b.play(Play(Black, 14, 14)).unwrap();

    assert!(b.get_coord(Coord::new(14, 14)) == Black);

    for i in range(1u8, 20) {
        for j in range(1u8 , 20) {
            assert!(b.get_coord(Coord::new(i, j)) == Empty || (i == 14 && j == 14));
        }
    }
}

#[test]
fn playing_on_an_illegal_coordinate_should_return_error() {
  let zht = Rc::new(ZobristHashTable::new(9));
  let b = Board::new(9, Minimal, zht.clone());

  assert!(b.play(Play(Black, 13, 13)).is_err());
}

#[test]
fn playing_on_a_non_empty_intersection_should_return_error() {
  let zht = Rc::new(ZobristHashTable::new(9));
  let b = Board::new(9, Minimal, zht.clone());

  let b = b.play(Play(Black, 4, 4)).unwrap();
  assert!(b.play(Play(Black, 4, 4)).is_err());
  assert!(b.play(Play(White, 4, 4)).is_err());
}

#[test]
fn two_way_merging_works() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

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
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

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
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

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
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

  b = b.play(Play(Black, 1, 1)).unwrap();
  b = b.play(Play(White, 1, 2)).unwrap();
  b = b.play(Play(White, 2, 1)).unwrap();

  assert_eq!(b.get_coord(Coord::new(1, 1)), Empty);
  assert_eq!(b.get_coord(Coord::new(1, 2)), White);
  assert_eq!(b.get_coord(Coord::new(2, 1)), White);
}

#[test]
fn playing_on_all_libs_on_side_should_capture() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

  b = b.play(Play(Black, 1, 3)).unwrap();
  b = b.play(Play(White, 1, 2)).unwrap();
  b = b.play(Play(White, 1, 4)).unwrap();
  b = b.play(Play(White, 2, 3)).unwrap();

  assert_eq!(b.get_coord(Coord::new(1, 3)), Empty);
  assert_eq!(b.get_coord(Coord::new(1, 2)), White);
  assert_eq!(b.get_coord(Coord::new(1, 4)), White);
  assert_eq!(b.get_coord(Coord::new(2, 3)), White);
}

#[test]
fn playing_on_all_libs_should_capture() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

  b = b.play(Play(Black, 4, 4)).unwrap();

  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(White, 4, 5)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();

  assert_eq!(b.get_coord(Coord::new(4, 4)), Empty);

  assert_eq!(b.get_coord(Coord::new(4, 3)), White);
  assert_eq!(b.get_coord(Coord::new(4, 5)), White);
  assert_eq!(b.get_coord(Coord::new(3, 4)), White);
  assert_eq!(b.get_coord(Coord::new(5, 4)), White);
}

#[test]
fn playing_on_all_libs_of_a_chain_should_capture() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(Black, 4, 5)).unwrap();

  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();
  b = b.play(Play(White, 3, 5)).unwrap();
  b = b.play(Play(White, 5, 5)).unwrap();
  b = b.play(Play(White, 4, 6)).unwrap();

  assert_eq!(b.get_coord(Coord::new(4, 4)), Empty);
  assert_eq!(b.get_coord(Coord::new(4, 5)), Empty);

  assert_eq!(b.get_coord(Coord::new(4, 3)), White);
  assert_eq!(b.get_coord(Coord::new(3, 4)), White);
  assert_eq!(b.get_coord(Coord::new(5, 4)), White);
  assert_eq!(b.get_coord(Coord::new(3, 5)), White);
  assert_eq!(b.get_coord(Coord::new(5, 5)), White);
  assert_eq!(b.get_coord(Coord::new(4, 6)), White);
}

#[test]
fn playing_on_all_libs_of_a_bent_chain_should_capture() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, Minimal, zht.clone());

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

  assert_eq!(b.get_coord(Coord::new(4, 4)), Empty);
  assert_eq!(b.get_coord(Coord::new(4, 5)), Empty);
  assert_eq!(b.get_coord(Coord::new(3, 4)), Empty);

  assert_eq!(b.get_coord(Coord::new(3, 3)), White);
  assert_eq!(b.get_coord(Coord::new(4, 3)), White);
  assert_eq!(b.get_coord(Coord::new(2, 4)), White);
  assert_eq!(b.get_coord(Coord::new(5, 4)), White);
  assert_eq!(b.get_coord(Coord::new(3, 5)), White);
  assert_eq!(b.get_coord(Coord::new(5, 5)), White);
  assert_eq!(b.get_coord(Coord::new(4, 6)), White);
}

#[test]
fn suicide_should_be_legal_in_tromp_taylor_rules() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, AnySizeTrompTaylor, zht.clone());

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
fn suicide_should_be_illegal_in_kgs_chinese_rules() {
    let zht = Rc::new(ZobristHashTable::new(3));
    let mut b = Board::new(3, KgsChinese, zht.clone());

    b = b.play(Play(Black, 2, 2)).unwrap();
    b = b.play(Play(White, 1, 2)).unwrap();
    b = b.play(Play(Black, 2, 1)).unwrap();
    b = b.play(Play(White, 3, 2)).unwrap();
    b = b.play(Play(Black, 2, 3)).unwrap();
    b = b.play(Play(White, 3, 1)).unwrap();
    b = b.play(Pass(Black)).unwrap();
    b = b.play(Play(White, 1, 3)).unwrap();
    b = b.play(Pass(Black)).unwrap();

    let play = b.play(Play(White, 1, 1));
    assert!(play.is_err());
    assert_eq!(play.unwrap_err(), IllegalMove::SuicidePlay);
}

#[test]
fn suicide_should_remove_the_suicided_chain() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, AnySizeTrompTaylor, zht.clone());

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

  assert_eq!(b.get_coord(Coord::new(3, 4)), Empty);
  assert_eq!(b.get_coord(Coord::new(4, 4)), Empty);

  assert_eq!(b.get_coord(Coord::new(5, 4)), White);
  assert_eq!(b.get_coord(Coord::new(4, 3)), White);
  assert_eq!(b.get_coord(Coord::new(3, 3)), White);
  assert_eq!(b.get_coord(Coord::new(2, 4)), White);
  assert_eq!(b.get_coord(Coord::new(4, 5)), White);
  assert_eq!(b.get_coord(Coord::new(3, 5)), White);
}

#[test]
fn playing_twice_should_be_illegal_in_tromp_taylor_rules() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  b = b.play(Play(Black, 10, 10)).unwrap();

  assert!(b.play(Play(Black, 4, 4)).is_err());
}

#[test]
#[should_fail]
fn board_size_and_size_of_zobrist_hash_need_to_agree() {
    let zht = Rc::new(ZobristHashTable::new(9));
    Board::new(13, AnySizeTrompTaylor, zht.clone());
}

#[test]
fn after_two_passes_the_game_should_be_over_in_tromp_taylor_rules() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  b = b.play(Pass(Black)).unwrap();
  b = b.play(Pass(White)).unwrap();
  assert!(b.is_game_over());

  assert!(b.play(Play(Black, 4, 4)).is_err());
}

#[test]
fn replaying_directly_on_a_ko_point_should_be_illegal() {
  let zht = Rc::new(ZobristHashTable::new(19));
  let mut b = Board::new(19, AnySizeTrompTaylor, zht.clone());

  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(White, 5, 4)).unwrap();
  b = b.play(Play(Black, 3, 3)).unwrap();
  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(Black, 3, 5)).unwrap();
  b = b.play(Play(White, 4, 5)).unwrap();
  b = b.play(Play(Black, 2, 4)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();

  match b.play(Play(Black, 4, 4)) {
    Err(IllegalMove::SuperKoRuleBroken) => (),
    Ok(_)                               => panic!("Replaying on a ko was allowed"),
    Err(x)                              => panic!("Engine crashed while trying to replay on a ko : {}", x)
  }
}

#[test]
fn counting_simple_case() {
  let zht = Rc::new(ZobristHashTable::new(4));
  let mut b = Board::new(4, Minimal, zht.clone());

  b = b.play(Play(Black, 2, 1)).unwrap();
  b = b.play(Play(White, 3, 1)).unwrap();
  b = b.play(Play(Black, 2, 2)).unwrap();
  b = b.play(Play(White, 3, 2)).unwrap();
  b = b.play(Play(Black, 2, 3)).unwrap();
  b = b.play(Play(White, 3, 3)).unwrap();
  b = b.play(Play(Black, 2, 4)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();
  b = b.play(Pass(Black)).unwrap();
  b = b.play(Pass(White)).unwrap();

  let (b_score, w_score) = b.score();
  assert_eq!(b_score, 8);
  assert_eq!(w_score, 8);
}

#[test]
fn counting_disjoint_territory() {
  let size = 5;

  let zht = Rc::new(ZobristHashTable::new(size));
  let mut b = Board::new(size, Minimal, zht.clone());

  b = b.play(Play(Black, 2, 1)).unwrap();
  b = b.play(Play(White, 3, 1)).unwrap();
  b = b.play(Play(Black, 2, 2)).unwrap();
  b = b.play(Play(White, 3, 2)).unwrap();
  b = b.play(Play(Black, 1, 3)).unwrap();
  b = b.play(Play(White, 2, 3)).unwrap();
  b = b.play(Play(Black, 5, 4)).unwrap();
  b = b.play(Play(White, 1, 4)).unwrap();
  b = b.play(Play(Black, 4, 4)).unwrap();
  b = b.play(Play(White, 5, 3)).unwrap();
  b = b.play(Play(Black, 4, 5)).unwrap();
  b = b.play(Play(White, 4, 3)).unwrap();
  b = b.play(Play(Black, 1, 2)).unwrap();
  b = b.play(Play(White, 3, 4)).unwrap();
  b = b.play(Pass(Black)).unwrap();
  b = b.play(Play(White, 3, 5)).unwrap();
  b = b.play(Pass(Black)).unwrap();
  b = b.play(Pass(White)).unwrap();

  let (b_score, w_score) = b.score();
  assert_eq!(b_score, 9);
  assert_eq!(w_score, 16);
}

#[test]
fn counting_with_neutral_points() {
  let size = 5;

  let zht = Rc::new(ZobristHashTable::new(size));
  let mut b = Board::new(size, Minimal, zht.clone());

  b = b.play(Play(Black, 2, 1)).unwrap();
  b = b.play(Play(White, 3, 1)).unwrap();
  b = b.play(Play(Black, 2, 2)).unwrap();
  b = b.play(Play(White, 3, 2)).unwrap();
  b = b.play(Play(Black, 1, 2)).unwrap();
  b = b.play(Play(White, 2, 3)).unwrap();
  b = b.play(Pass(Black)).unwrap();
  b = b.play(Play(White, 1, 4)).unwrap();
  b = b.play(Pass(Black)).unwrap();
  b = b.play(Pass(White)).unwrap();

  let (b_score, w_score) = b.score();
  assert_eq!(b_score, 4);
  assert_eq!(w_score, 20);
}

#[test]
fn capturing_two_or_more_groups_while_playing_in_an_eye_actually_captures() {
  let size = 5;

  let zht = Rc::new(ZobristHashTable::new(size));
  let mut b = Board::new(size, AnySizeTrompTaylor, zht.clone());

  b = b.play(Play(Black, 2, 1)).unwrap();
  b = b.play(Play(White, 3, 1)).unwrap();
  b = b.play(Play(Black, 1, 2)).unwrap();
  b = b.play(Play(White, 2, 2)).unwrap();
  b = b.play(Play(Black, 5, 5)).unwrap();
  b = b.play(Play(White, 1, 3)).unwrap();
  b = b.play(Play(Black, 5, 4)).unwrap();
  b = b.play(Play(White, 1, 1)).unwrap();

  assert_eq!(b.get_coord(Coord::new(1, 1)), White);
  assert_eq!(b.get_coord(Coord::new(1, 2)), Empty);
  assert_eq!(b.get_coord(Coord::new(2, 1)), Empty);
  assert_eq!(b.get_coord(Coord::new(2, 2)), White);
}

#[test]
fn next_player_should_return_black_without_moves() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    assert_eq!(Black, b.next_player());
}

#[test]
fn next_player_should_return_with_after_a_single_move() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let mut b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    b = b.play(Play(Black, 1, 1)).unwrap();
    assert_eq!(White, b.next_player());
}

#[test]
fn legal_moves_should_include_pass() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    let moves = b.legal_moves();
    assert!(moves.contains(&Pass(Black)));
}

#[test]
fn legal_moves_should_return_black_moves_on_a_board_without_moves() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    let moves = b.legal_moves();
    let all_black = moves.iter().all(|m| m.color() == &Black);
    assert!(all_black);
}

#[test]
fn legal_moves_should_return_white_moves_on_a_board_with_one_move() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let mut b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    b = b.play(Play(Black, 1, 1)).unwrap();
    let moves = b.legal_moves();
    let all_white = moves.iter().all(|m| m.color() == &White);
    assert!(all_white);
}

#[test]
fn legal_moves_contains_the_right_number_of_moves_for_an_empty_board() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    assert_eq!(b.legal_moves().len(), 25+1);
}

#[test]
fn legal_moves_only_contains_legal_moves() {
    let size = 5;
    let zht = Rc::new(ZobristHashTable::new(size));
    let mut b = Board::new(size, AnySizeTrompTaylor, zht.clone());
    b = b.play(Play(Black, 1, 1)).unwrap();
    let moves = b.legal_moves();
    assert!(!moves.iter().any(|m| m == &Play(White, 1, 1)));
}

#[test]
fn ruleset_returns_the_correct_ruleset() {
    let size = 1;
    let zht = Rc::new(ZobristHashTable::new(size));
    let b = Board::new(size, Minimal, zht.clone());
    assert_eq!(b.ruleset(), Minimal);
}

#[test]
fn chains_returns_the_chains_on_the_board() {
    let size = 1;
    let zht = Rc::new(ZobristHashTable::new(size));
    let b = Board::new(size, Minimal, zht.clone());
    assert_eq!(*b.chains(), vec!(Chain::new(0, Empty)));
}
