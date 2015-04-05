/************************************************************************
 *                                                                      *
 * Copyright 2015 Igor Polyakov                                         *
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
use std::path::Path;
use board::Black;
use board::Coord;
use board::Pass;
use board::White;
use board::movement::Play;
use sgf::Parser;

#[test]
fn top_left_has_one_liberty() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(1, board.liberty_count(Coord::new(1,19)));
}

#[test]
fn one_below_top_left_has_two_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(2, board.liberty_count(Coord::new(1,18)));
}

#[test]
fn two_below_top_left_has_three_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(3, board.liberty_count(Coord::new(1,17)));
}

#[test]
fn first_square_surrounded_by_four_liberties_in_top_left() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(4, board.liberty_count(Coord::new(5,16)));
}

#[test]
fn removes_one_stone() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(1, board.removes_enemy_neighbouring_stones(Play(Black, 4, 19)));
}

#[test]
fn removes_two_stones() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(2, board.removes_enemy_neighbouring_stones(Play(Black, 4, 15)));
}

#[test]
fn removes_three_stones() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(3, board.removes_enemy_neighbouring_stones(Play(Black, 4, 10)));
}

#[test]
fn removes_four_stones() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(4, board.removes_enemy_neighbouring_stones(Play(Black, 4, 4)));
}

#[test]
fn removes_three_neighbours() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(3, board.removes_enemy_neighbouring_stones(Play(Black, 9, 18)));
}

#[test]
fn removes_two_neighbours() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(2, board.removes_enemy_neighbouring_stones(Play(Black, 9, 15)));
}

#[test]
fn two_stones_have_six_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    
    let play = Play(Black, 10, 12);
    assert_eq!(2, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 3));
    assert_eq!(6, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 5));
}

#[test]
fn three_stones_have_eight_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    let play = Play(Black, 10, 10);
    assert_eq!(3, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 4));
    assert_eq!(8, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 7));
}

#[test]
fn four_stones_have_eight_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    let play = Play(Black, 10, 7);
    assert_eq!(4, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 5));
    assert_eq!(8, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 7));
}

#[test]
fn five_stones_have_eight_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    let play = Play(Black, 10, 4);
    assert_eq!(5, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 6));
    assert_eq!(8, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 7));
}

#[test]
fn six_stones_have_nine_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    let play = Play(Black, 15, 17);
    assert_eq!(6, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 7));
    assert_eq!(9, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 8));
}

#[test]
fn seven_stones_have_ten_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    let play = Play(Black, 15, 13);

    assert_eq!(7, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 8));
    assert_eq!(10, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 9));
}

#[test]
fn nine_stones_have_twelve_liberties() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/hypothetical-plays.sgf"));
    let game   = parser.game().unwrap();
    let board  = game.board();
    let play = Play(Black, 15, 9);
    assert_eq!(9, board.new_chain_length(play));
    assert!(board.new_chain_length_less_than(play, 10));
    assert_eq!(12, board.new_chain_liberties(play));
    assert!(board.new_chain_liberties_greater_than(play, 11));
}