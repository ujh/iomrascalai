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
        let pattern = Pattern::new([
            ['X', 'O', '.'],
            ['x', 'o', '?'],
            [' ', ' ', ' ']]);
        let expanded = pattern.expand();
    }

    it "includes all variations" {
        assert_that(expanded.len(), is(equal_to(16)));
    }

    it "includes the original pattern" {
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the orginal pattern swapped" {
        let pattern = Pattern::new([
            ['O', 'X', '.'],
            ['o', 'x', '?'],
            [' ', ' ', ' ']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 90deg rotated pattern" {
        let pattern = Pattern::new([
            [' ', 'x', 'X'],
            [' ', 'o', 'O'],
            [' ', '?', '.']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 90deg rotated pattern swapped" {
        let pattern = Pattern::new([
            [' ', 'o', 'O'],
            [' ', 'x', 'X'],
            [' ', '?', '.']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 180deg rotated pattern" {
        let pattern = Pattern::new([
            [' ', ' ', ' '],
            ['?', 'o', 'x'],
            ['.', 'O', 'X']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 180deg rotated pattern swapped" {
        let pattern = Pattern::new([
            [' ', ' ', ' '],
            ['?', 'x', 'o'],
            ['.', 'X', 'O']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 270deg rotated pattern" {
        let pattern = Pattern::new([
            ['.', '?', ' '],
            ['O', 'o', ' '],
            ['X', 'x', ' ']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the 270deg rotated pattern swapped" {
        let pattern = Pattern::new([
            ['.', '?', ' '],
            ['X', 'x', ' '],
            ['O', 'o', ' ']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the horizontally flipped pattern" {
        let pattern = Pattern::new([
            [' ', ' ', ' '],
            ['x', 'o', '?'],
            ['X', 'O', '.']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the horizontally flipped pattern swapped" {
        let pattern = Pattern::new([
            [' ', ' ', ' '],
            ['o', 'x', '?'],
            ['O', 'X', '.']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the vertially flipped pattern" {
        let pattern = Pattern::new([
            ['.', 'O', 'X'],
            ['?', 'o', 'x'],
            [' ', ' ', ' ']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

    it "includes the vertically flipped pattern swapped" {
        let pattern = Pattern::new([
            ['.', 'X', 'O'],
            ['?', 'x', 'o'],
            [' ', ' ', ' ']]);
        assert_that(&expanded, contains(vec!(pattern)));
    }

}
