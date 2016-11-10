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

pub use hamcrest::prelude::*;
pub use std::path::Path;

pub use board::Board;
pub use board::Coord;
pub use sgf::Parser;

pub use super::Matcher;
pub use super::Pattern;

describe! expand_patterns {

    before_each {
        let pattern = Pattern::new([
            ['X', 'O', '.'],
            ['x', 'o', '?'],
            [' ', ' ', ' ']]);
        let patterns = vec!(pattern);
    }

    it "includes all variations" {
        let expanded = Matcher::expand_patterns(patterns);
        assert_that!(expanded.len(), is(equal_to(16)));
    }

}

pub fn board_from_sgf(s: &str) -> Board {
    let parser = Parser::from_path(Path::new(&format!("fixtures/sgf/{}", s))).unwrap();
    let game = parser.game().unwrap();
    game.board()
}

describe! pattern_count {

    before_each {
        let center = &Coord::new(5, 5);
        let off_center = &Coord::new(4, 4);
    }

    it "matches patterns with black stones" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['X', '.', '.'],
            ['.', '.', '.']]);
        let matcher = Matcher::with_patterns(vec!(pattern));
        let board = &board_from_sgf("3x3/one-black-w.sgf");
        assert_that!(matcher.pattern_count(board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(board, off_center), is(equal_to(0)));
    }

    it "matches patterns with white stones" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['O', '.', '.'],
            ['.', '.', '.']]);
        let matcher = Matcher::with_patterns(vec!(pattern));
        let board = &board_from_sgf("3x3/one-white-w.sgf");
        assert_that!(matcher.pattern_count(board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(board, off_center), is(equal_to(0)));
    }

    it "matches patterns with non-black stones" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['x', '.', '.'],
            ['.', '.', '.']]);
        let matcher = Matcher::with_patterns(vec!(pattern));
        let board = &board_from_sgf("3x3/one-white-w.sgf");
        assert_that!(matcher.pattern_count(board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(board, off_center), is(equal_to(0)));
    }

    it "matches patterns with non-white stones" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['o', '.', '.'],
            ['.', '.', '.']]);
        let matcher = Matcher::with_patterns(vec!(pattern));
        let board = &board_from_sgf("3x3/one-black-w.sgf");
        assert_that!(matcher.pattern_count(board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(board, off_center), is(equal_to(0)));
    }

    it "matches patterns with wildcards" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['?', '.', '.'],
            ['.', '.', '.']]);
        let matcher = Matcher::with_patterns(vec!(pattern));
        let black_board = &board_from_sgf("3x3/one-black-w.sgf");
        assert_that!(matcher.pattern_count(black_board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(black_board, off_center), is(equal_to(0)));
        let white_board = &board_from_sgf("3x3/one-white-w.sgf");
        assert_that!(matcher.pattern_count(white_board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(white_board, off_center), is(equal_to(0)));
        let empty_board = &board_from_sgf("empty.sgf");
        assert_that!(matcher.pattern_count(empty_board, center), is(equal_to(1)));
        assert_that!(matcher.pattern_count(empty_board, off_center), is(equal_to(1)));
    }

    it "matches off board patterns" {
        let pattern = Pattern::new([
            [' ', '.', '.'],
            [' ', '.', '.'],
            [' ', ' ', ' ']]);
        let matcher = Matcher::with_patterns(vec!(pattern));
        let board = &board_from_sgf("empty.sgf");
        assert_that!(matcher.pattern_count(board, &Coord::new(1,1)), is(equal_to(1)));
        assert_that!(matcher.pattern_count(board, center), is(equal_to(0)));
        assert_that!(matcher.pattern_count(board, off_center), is(equal_to(0)));
    }
}
