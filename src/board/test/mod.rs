/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
#![allow(unused_must_use)]

use board::Black;
use board::Board;
use board::Coord;
use board::Empty;
use board::IllegalMove;
use board::Pass;
use board::Play;
use board::Resign;
use board::White;
use ruleset::AnySizeTrompTaylor;
use ruleset::KgsChinese;
use ruleset::Minimal;
use sgf::Parser;

use test::Bencher;

mod eye;
mod ko;
mod orthogonals;

#[test]
fn getting_a_valid_coord_returns_a_color() {
    let b = Board::new(19, 6.5, AnySizeTrompTaylor);

    assert_eq!(b.color(&Coord::new(10, 10)), Empty);
}

#[test]
#[should_panic]
fn getting_invalid_coordinates_fails() {
    let b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.color(&Coord::new(14, 21));
    b.color(&Coord::new(21, 14));
}

#[test]
fn _19_19_is_a_valid_coordinate(){
    let b = Board::new(19, 6.5, AnySizeTrompTaylor);

    assert_eq!(b.color(&Coord::new(19, 19)), Empty);
}

#[test]
#[should_panic]
fn _0_0_is_not_a_valid_coordinate(){
    let b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.color(&Coord::new(0, 0));
}

#[test]
fn play_adds_a_stone_to_the_correct_position() {
    let mut b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.play(Play(Black, 14, 14));

    assert!(b.color(&Coord::new(14, 14)) == Black);

    for i in range(1u8, 20) {
        for j in range(1u8 , 20) {
            assert!(b.color(&Coord::new(i, j)) == Empty || (i == 14 && j == 14));
        }
    }
}

#[test]
fn playing_on_an_illegal_coordinate_should_return_error() {
    let mut b = Board::new(9, 6.5, Minimal);

    assert!(b.play(Play(Black, 13, 13)).is_err());
}

#[test]
fn playing_on_a_non_empty_intersection_should_return_error() {
    let mut b = Board::new(9, 6.5, Minimal);

    b.play(Play(Black, 4, 4));
    assert!(b.play(Play(Black, 4, 4)).is_err());
    assert!(b.play(Play(White, 4, 4)).is_err());
}

#[test]
fn resigning_is_recognized_as_game_over() {
    let mut b = Board::new(9, 6.5, Minimal);

    b.play(Resign(Black));
    assert!(b.is_game_over(), "Game should be over after a resign");
}

#[test]
fn its_not_possible_to_play_after_a_resign() {
    let mut b = Board::new(9, 6.5, AnySizeTrompTaylor);

    b.play(Resign(Black));
    let res = b.play(Play(White, 1, 1));
    match res {
        Err(e) => assert_eq!(e, IllegalMove::GameAlreadyOver),
        Ok(_)  => panic!("error expected")
    }
}

#[test]
fn resigning_means_the_other_player_won() {
    let mut b = Board::new(9, 6.5, AnySizeTrompTaylor);
    b.play(Play(Black, 1, 1));
    b.play(Play(White, 2, 2));
    b.play(Resign(Black));

    assert_eq!(White, b.winner());
}

#[test]
fn legal_moves_should_return_nothing_after_a_resign() {
    let mut b = Board::new(9, 6.5, Minimal);

    b.play(Resign(Black));

    assert_eq!(vec!(), b.legal_moves_without_superko_check());
}

#[test]
fn legal_moves_without_eyes_should_return_nothing_after_a_resing() {
    let mut b = Board::new(9, 6.5, Minimal);

    b.play(Resign(Black));

    assert_eq!(vec!(), b.legal_moves_without_eyes());
}


#[test]
fn two_way_merging_works() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(White, 10, 10));
    b.play(Play(White, 10, 12));

    assert_eq!(b.chains.len(), 2);

    b.play(Play(White, 10, 11));
    let c_id = b.get_chain(Coord::new(10, 10)).unwrap().id();

    assert_eq!(b.get_chain(Coord::new(10, 11)).unwrap().id(), c_id);
    assert_eq!(b.get_chain(Coord::new(10, 12)).unwrap().id(), c_id);
    assert_eq!(b.chains.len(), 1)
}

#[test]
fn three_way_merging_works() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(White, 10, 10));
    b.play(Play(White, 11, 11));
    b.play(Play(White, 10, 12));

    assert_eq!(b.chains.len(), 3);

    b.play(Play(White, 10, 11));
    let c_id = b.get_chain(Coord::new(10, 10)).unwrap().id();

    assert_eq!(b.get_chain(Coord::new(10, 11)).unwrap().id(), c_id);
    assert_eq!(b.get_chain(Coord::new(11, 11)).unwrap().id(), c_id);
    assert_eq!(b.get_chain(Coord::new(10, 12)).unwrap().id(), c_id);
    assert_eq!(b.chains.len(), 1)
}

#[test]
fn four_way_merging_works() {
    let mut b = Board::new(19, 6.5, Minimal);
    b.play(Play(White, 10, 10));
    b.play(Play(White, 9, 11));
    b.play(Play(White, 11, 11));
    b.play(Play(White, 10, 12));

    assert_eq!(b.chains.len(), 4);

    b.play(Play(White, 10, 11));
    let c_id = b.get_chain(Coord::new(10, 10)).unwrap().id();

    assert_eq!(b.get_chain(Coord::new(10, 11)).unwrap().id(), c_id);
    assert_eq!(b.get_chain(Coord::new(9 , 11)).unwrap().id(), c_id);
    assert_eq!(b.get_chain(Coord::new(11, 11)).unwrap().id(), c_id);
    assert_eq!(b.get_chain(Coord::new(10, 12)).unwrap().id(), c_id);
    assert_eq!(b.chains.len(), 1)
}

#[test]
fn playing_on_all_libs_in_corner_should_capture() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(Black, 1, 1));
    b.play(Play(White, 1, 2));
    b.play(Play(White, 2, 1));

    assert_eq!(b.color(&Coord::new(1, 1)), Empty);
    assert_eq!(b.color(&Coord::new(1, 2)), White);
    assert_eq!(b.color(&Coord::new(2, 1)), White);
}

#[test]
fn playing_on_all_libs_on_side_should_capture() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(Black, 1, 3));
    b.play(Play(White, 1, 2));
    b.play(Play(White, 1, 4));
    b.play(Play(White, 2, 3));

    assert_eq!(b.color(&Coord::new(1, 3)), Empty);
    assert_eq!(b.color(&Coord::new(1, 2)), White);
    assert_eq!(b.color(&Coord::new(1, 4)), White);
    assert_eq!(b.color(&Coord::new(2, 3)), White);
}

#[test]
fn playing_on_all_libs_should_capture() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(Black, 4, 4));

    b.play(Play(White, 4, 3));
    b.play(Play(White, 4, 5));
    b.play(Play(White, 3, 4));
    b.play(Play(White, 5, 4));

    assert_eq!(b.color(&Coord::new(4, 4)), Empty);

    assert_eq!(b.color(&Coord::new(4, 3)), White);
    assert_eq!(b.color(&Coord::new(4, 5)), White);
    assert_eq!(b.color(&Coord::new(3, 4)), White);
    assert_eq!(b.color(&Coord::new(5, 4)), White);
}

#[test]
fn playing_on_all_libs_of_a_chain_should_capture() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(Black, 4, 4));
    b.play(Play(Black, 4, 5));

    b.play(Play(White, 4, 3));
    b.play(Play(White, 3, 4));
    b.play(Play(White, 5, 4));
    b.play(Play(White, 3, 5));
    b.play(Play(White, 5, 5));
    b.play(Play(White, 4, 6));

    assert_eq!(b.color(&Coord::new(4, 4)), Empty);
    assert_eq!(b.color(&Coord::new(4, 5)), Empty);

    assert_eq!(b.color(&Coord::new(4, 3)), White);
    assert_eq!(b.color(&Coord::new(3, 4)), White);
    assert_eq!(b.color(&Coord::new(5, 4)), White);
    assert_eq!(b.color(&Coord::new(3, 5)), White);
    assert_eq!(b.color(&Coord::new(5, 5)), White);
    assert_eq!(b.color(&Coord::new(4, 6)), White);
}

#[test]
fn playing_on_all_libs_of_a_bent_chain_should_capture() {
    let mut b = Board::new(19, 6.5, Minimal);

    b.play(Play(Black, 4, 4));
    b.play(Play(Black, 4, 5));
    b.play(Play(Black, 3, 4));

    b.play(Play(White, 3, 3));
    b.play(Play(White, 4, 3));
    b.play(Play(White, 2, 4));
    b.play(Play(White, 5, 4));
    b.play(Play(White, 3, 5));
    b.play(Play(White, 5, 5));
    b.play(Play(White, 4, 6));

    assert_eq!(b.color(&Coord::new(4, 4)), Empty);
    assert_eq!(b.color(&Coord::new(4, 5)), Empty);
    assert_eq!(b.color(&Coord::new(3, 4)), Empty);

    assert_eq!(b.color(&Coord::new(3, 3)), White);
    assert_eq!(b.color(&Coord::new(4, 3)), White);
    assert_eq!(b.color(&Coord::new(2, 4)), White);
    assert_eq!(b.color(&Coord::new(5, 4)), White);
    assert_eq!(b.color(&Coord::new(3, 5)), White);
    assert_eq!(b.color(&Coord::new(5, 5)), White);
    assert_eq!(b.color(&Coord::new(4, 6)), White);
}

#[test]
fn suicide_should_be_legal_in_tromp_taylor_rules() {
    let mut b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.play(Play(Black, 4, 4));
    b.play(Play(White, 5, 4));
    b.play(Play(Black, 16, 16));
    b.play(Play(White, 4, 3));
    b.play(Play(Black, 16, 15));
    b.play(Play(White, 3, 3));
    b.play(Play(Black, 16, 14));
    b.play(Play(White, 2, 4));
    b.play(Play(Black, 16, 13));
    b.play(Play(White, 4, 5));
    b.play(Play(Black, 16, 12));
    b.play(Play(White, 3, 5));

    assert!(b.play(Play(Black, 3, 4)).is_ok());
}

#[test]
fn suicide_should_be_illegal_in_kgs_chinese_rules() {
    let mut b = Board::new(3, 6.5, KgsChinese);

    b.play(Play(Black, 2, 2));
    b.play(Play(White, 1, 2));
    b.play(Play(Black, 2, 1));
    b.play(Play(White, 3, 2));
    b.play(Play(Black, 2, 3));
    b.play(Play(White, 3, 1));
    b.play(Pass(Black));
    b.play(Play(White, 1, 3));
    b.play(Pass(Black));

    let play = b.play(Play(White, 1, 1));
    match play {
        Err(e) => assert_eq!(e, IllegalMove::SuicidePlay),
        Ok(_)  => panic!("Error expected")
    }
}

#[test]
fn suicide_should_remove_the_suicided_chain() {
    let mut b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.play(Play(Black, 4, 4));
    b.play(Play(White, 5, 4));
    b.play(Play(Black, 16, 16));
    b.play(Play(White, 4, 3));
    b.play(Play(Black, 16, 15));
    b.play(Play(White, 3, 3));
    b.play(Play(Black, 16, 14));
    b.play(Play(White, 2, 4));
    b.play(Play(Black, 16, 13));
    b.play(Play(White, 4, 5));
    b.play(Play(Black, 16, 12));
    b.play(Play(White, 3, 5));

    b.play(Play(Black, 3, 4));

    assert_eq!(b.color(&Coord::new(3, 4)), Empty);
    assert_eq!(b.color(&Coord::new(4, 4)), Empty);

    assert_eq!(b.color(&Coord::new(5, 4)), White);
    assert_eq!(b.color(&Coord::new(4, 3)), White);
    assert_eq!(b.color(&Coord::new(3, 3)), White);
    assert_eq!(b.color(&Coord::new(2, 4)), White);
    assert_eq!(b.color(&Coord::new(4, 5)), White);
    assert_eq!(b.color(&Coord::new(3, 5)), White);
}

#[test]
fn playing_twice_should_be_illegal_in_tromp_taylor_rules() {
    let mut b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.play(Play(Black, 10, 10));

    assert!(b.play(Play(Black, 4, 4)).is_err());
}

#[test]
fn after_two_passes_the_game_should_be_over_in_tromp_taylor_rules() {
    let mut b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.play(Pass(Black));
    b.play(Pass(White));
    assert!(b.is_game_over());

    assert!(b.play(Play(Black, 4, 4)).is_err());
}

#[test]
fn capturing_two_or_more_groups_while_playing_in_an_eye_actually_captures() {
    let mut b = Board::new(5, 6.5, AnySizeTrompTaylor);

    b.play(Play(Black, 2, 1));
    b.play(Play(White, 3, 1));
    b.play(Play(Black, 1, 2));
    b.play(Play(White, 2, 2));
    b.play(Play(Black, 5, 5));
    b.play(Play(White, 1, 3));
    b.play(Play(Black, 5, 4));
    b.play(Play(White, 1, 1));

    assert_eq!(b.color(&Coord::new(1, 1)), White);
    assert_eq!(b.color(&Coord::new(1, 2)), Empty);
    assert_eq!(b.color(&Coord::new(2, 1)), Empty);
    assert_eq!(b.color(&Coord::new(2, 2)), White);
}

#[test]
fn next_player_should_return_black_without_moves() {
    let b = Board::new(5, 6.5, AnySizeTrompTaylor);
    assert_eq!(Black, b.next_player());
}

#[test]
fn next_player_should_return_white_after_a_single_move() {
    let mut b = Board::new(5, 6.5, AnySizeTrompTaylor);
    b.play(Play(Black, 1, 1));
    assert_eq!(White, b.next_player());
}

#[test]
fn legal_moves_should_return_black_moves_on_a_board_without_moves() {
    let b = Board::new(5, 6.5, AnySizeTrompTaylor);
    let moves = b.legal_moves_without_superko_check();
    let all_black = moves.iter().all(|m| m.color() == &Black);
    assert!(all_black);
}

#[test]
fn legal_moves_should_return_white_moves_on_a_board_with_one_move() {
    let mut b = Board::new(5, 6.5, AnySizeTrompTaylor);
    b.play(Play(Black, 1, 1));
    let moves = b.legal_moves_without_superko_check();
    let all_white = moves.iter().all(|m| m.color() == &White);
    assert!(all_white);
}

#[test]
fn legal_moves_contains_the_right_number_of_moves_for_an_empty_board() {
    let b = Board::new(5, 6.5, AnySizeTrompTaylor);
    assert_eq!(b.legal_moves_without_superko_check().len(), 25);
}

#[test]
fn legal_moves_only_contains_legal_moves() {
    let mut b = Board::new(5, 6.5, AnySizeTrompTaylor);
    b.play(Play(Black, 1, 1));
    let moves = b.legal_moves_without_superko_check();
    assert!(!moves.iter().any(|m| m == &Play(White, 1, 1)));
}

#[test]
fn ruleset_returns_the_correct_ruleset() {
    let b = Board::new(1, 6.5, Minimal);
    assert_eq!(b.ruleset(), Minimal);
}

#[test]
fn set_komi_updates_the_komi() {
    let mut b = Board::new(1, 6.5, Minimal);
    b.set_komi(10.0);
    assert_eq!(b.komi(), 10.0);
}

#[test]
fn komi_returns_the_komi() {
    let b = Board::new(1, 6.5, Minimal);
    assert_eq!(b.komi(), 6.5);
}

#[test]
fn adv_stones_removed_only_contains_each_coord_once() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/not-superko2.sgf"));
    let game = parser.game().unwrap();
    let board = game.board();
    let mut removed_coords = board.adv_stones_removed().clone();
    removed_coords.sort();
    removed_coords.dedup();
    assert_eq!(removed_coords.len(), board.adv_stones_removed().len());
}

#[bench]
fn bench_play_method(b: &mut Bencher) {
    b.iter(|| {
        let mut board = Board::new(19, 6.5, AnySizeTrompTaylor);
        board.play(Play(Black, 14, 14)).unwrap();
    });
}
