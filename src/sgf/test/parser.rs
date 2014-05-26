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
