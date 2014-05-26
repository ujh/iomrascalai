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

use sgf::parser::Parser;

fn read_sgf(name : &'static str) -> String {
    let path = Path::new(format!("fixtures/sgf/{}.sgf", name));
    let contents = File::open(&path).read_to_str();
    contents.unwrap()
}

fn empty_sgf() -> String {
    read_sgf("empty")
}

#[test]
fn sets_the_board_size_from_sgf() {
    let parser = Parser::new(empty_sgf());
    let board  = parser.board();
    assert_eq!(board.size(), 19);
}

#[test]
fn sets_the_komi_from_sgf() {
    let parser = Parser::new(empty_sgf());
    let board  = parser.board();
    assert_eq!(board.komi(), 6.5);
}

// TODO: The tokenization code should be private in the end and be
// tested through the board setup.
#[test]
fn tokenize_empty_sgf() {
    let parser = Parser::new(empty_sgf());
    let tokens = parser.tokenize();
    let expected = vec!(("GM", "1"), ("FF", "4"), ("CA", "UTF-8"), ("AP", "CGoban:3"),
                        ("ST", "2"), ("RU", "Japanese"), ("SZ", "19"), ("KM", "6.50"),
                        ("PW", "White"), ("PB", "Black"));
    assert_eq!(tokens, expected);
}

#[test]
fn tokenize_handicap_setup() {
    let parser = Parser::new(read_sgf("handicap"));
    let tokens = parser.tokenize();
    let expected = vec!(("FF", "4"), ("CA", "UTF-8"), ("AP", "GoGui:1.4.9"), ("AB", "dp"),
                        ("AB", "dd"), ("AB", "pd"), ("PL", "W"));
    assert_eq!(tokens, expected);
}
