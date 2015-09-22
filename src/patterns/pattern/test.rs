/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner                                          *
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

pub use hamcrest::assert_that;
pub use hamcrest::contains;
pub use hamcrest::equal_to;
pub use hamcrest::is;
pub use std::path::Path;

pub use board::Board;
pub use board::Coord;
pub use sgf::Parser;

pub use super::Pattern;

describe! expand {

    before_each {
        let pattern = Pattern::new(vec!(
            vec!('X', 'O', '.'),
            vec!('x', 'o', '?'),
            vec!(' ', ' ', ' ')));
        let expanded = pattern.expand();
    }

    it "includes all variations" {
        assert_that(expanded.len(), is(equal_to(12)));
    }

    it "includes the original pattern" {
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the orginal pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!('O', 'X', '.'),
            vec!('o', 'x', '?'),
            vec!(' ', ' ', ' ')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 90deg rotated pattern" {
        let pattern = Pattern::new(vec!(
            vec!(' ', 'x', 'X'),
            vec!(' ', 'o', 'O'),
            vec!(' ', '?', '.')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 90deg rotated pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!(' ', 'o', 'O'),
            vec!(' ', 'x', 'X'),
            vec!(' ', '?', '.')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 180deg rotated pattern" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('?', 'o', 'x'),
            vec!('.', 'O', 'X')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 180deg rotated pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('?', 'x', 'o'),
            vec!('.', 'X', 'O')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 270deg rotated pattern" {
        let pattern = Pattern::new(vec!(
            vec!('.', '?', ' '),
            vec!('O', 'o', ' '),
            vec!('X', 'x', ' ')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 270deg rotated pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!('.', '?', ' '),
            vec!('X', 'x', ' '),
            vec!('O', 'o', ' ')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the horizontally flipped pattern" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('x', 'o', '?'),
            vec!('X', 'O', '.')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the horizontally flipped pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('o', 'x', '?'),
            vec!('O', 'X', '.')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the vertially flipped pattern" {
        let pattern = Pattern::new(vec!(
            vec!('.', 'O', 'X'),
            vec!('?', 'o', 'x'),
            vec!(' ', ' ', ' ')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the vertically flipped pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!('.', 'X', 'O'),
            vec!('?', 'x', 'o'),
            vec!(' ', ' ', ' ')));
        assert_that(&expanded, contains(vec!(pattern)));
    }

}

pub fn board_from_sgf(s: &str) -> Board {
    let parser = Parser::from_path(Path::new(&format!("fixtures/sgf/{}", s))).unwrap();
    let game = parser.game().unwrap();
    game.board()
}

describe! matches {

    before_each {
        let center = &Coord::new(5, 5);
        let off_center = &Coord::new(4, 4);
    }

    it "matches a single black stone NW" {
        let pattern = Pattern::new(vec!(
            vec!('X', '.', '.'),
            vec!('.', '.', '.'),
            vec!('.', '.', '.')));
        let board = &board_from_sgf("3x3/one-black-nw.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone N" {
        let pattern = Pattern::new(vec!(
            vec!('.', 'X', '.'),
            vec!('.', '.', '.'),
            vec!('.', '.', '.')));
        let board = &board_from_sgf("3x3/one-black-n.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone NE" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', 'X'),
            vec!('.', '.', '.'),
            vec!('.', '.', '.')));
        let board = &board_from_sgf("3x3/one-black-ne.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone E" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', '.'),
            vec!('.', '.', 'X'),
            vec!('.', '.', '.')));
        let board = &board_from_sgf("3x3/one-black-e.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone SE" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', '.'),
            vec!('.', '.', '.'),
            vec!('.', '.', 'X')));
        let board = &board_from_sgf("3x3/one-black-se.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone S" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', '.'),
            vec!('.', '.', '.'),
            vec!('.', 'X', '.')));
        let board = &board_from_sgf("3x3/one-black-s.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone SW" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', '.'),
            vec!('.', '.', '.'),
            vec!('X', '.', '.')));
        let board = &board_from_sgf("3x3/one-black-sw.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single black stone W" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', '.'),
            vec!('X', '.', '.'),
            vec!('.', '.', '.')));
        let board = &board_from_sgf("3x3/one-black-w.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

    it "matches a single white stone W" {
        let pattern = Pattern::new(vec!(
            vec!('.', '.', '.'),
            vec!('O', '.', '.'),
            vec!('.', '.', '.')));
        let board = &board_from_sgf("3x3/one-white-w.sgf");
        assert_that(pattern.matches(board, center), is(equal_to(true)));
        assert_that(pattern.matches(board, off_center), is(equal_to(false)));
    }

}
