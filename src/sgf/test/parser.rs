/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
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

use std::io::fs::File;

use board::Black;
use board::White;
use sgf::parser::Parser;

fn sgf(name : &'static str) -> String {
    let path = Path::new(format!("fixtures/sgf/{}.sgf", name));
    let contents = File::open(&path).read_to_str();
    contents.unwrap()
}

#[test]
fn sets_the_board_size_from_sgf() {
    let parser = Parser::new(sgf("empty"));
    let board  = parser.board();
    assert_eq!(board.size(), 19);
}

#[test]
fn sets_the_komi_from_sgf() {
    let parser = Parser::new(sgf("empty"));
    let board  = parser.board();
    assert_eq!(board.komi(), 6.5);
}

#[test]
fn play_handicap_stones() {
    let parser = Parser::new(sgf("handicap"));
    let board  = parser.board();
    assert_eq!(board.get(4,4), Black);
    assert_eq!(board.get(16,4), Black);
    assert_eq!(board.get(16,16), Black);
}

#[test]
fn play_moves() {
    let parser = Parser::new(sgf("twomoves"));
    let board  = parser.board();
    assert_eq!(board.get(4, 15), Black);
    assert_eq!(board.get(16, 7), White);
}
