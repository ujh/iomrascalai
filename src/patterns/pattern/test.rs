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
        assert_eq!(expanded.len(), 12);
    }

    it "includes the original pattern" {
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the orginal pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!('O', 'X', '.'),
            vec!('o', 'x', '?'),
            vec!(' ', ' ', ' ')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the 90deg rotated pattern" {
        let pattern = Pattern::new(vec!(
            vec!(' ', 'x', 'X'),
            vec!(' ', 'o', 'O'),
            vec!(' ', '?', '.')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the 90deg rotated pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!(' ', 'o', 'O'),
            vec!(' ', 'x', 'X'),
            vec!(' ', '?', '.')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the 180deg rotated pattern" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('?', 'o', 'x'),
            vec!('.', 'O', 'X')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the 180deg rotated pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('?', 'x', 'o'),
            vec!('.', 'X', 'O')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the 270deg rotated pattern" {
        let pattern = Pattern::new(vec!(
            vec!('.', '?', ' '),
            vec!('O', 'o', ' '),
            vec!('X', 'x', ' ')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the 270deg rotated pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!('.', '?', ' '),
            vec!('X', 'x', ' '),
            vec!('O', 'o', ' ')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the horizontally flipped pattern" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('x', 'o', '?'),
            vec!('X', 'O', '.')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the horizontally flipped pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!(' ', ' ', ' '),
            vec!('o', 'x', '?'),
            vec!('O', 'X', '.')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the vertially flipped pattern" {
        let pattern = Pattern::new(vec!(
            vec!('.', 'O', 'X'),
            vec!('?', 'o', 'x'),
            vec!(' ', ' ', ' ')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

    it "includes the vertically flipped pattern swapped" {
        let pattern = Pattern::new(vec!(
            vec!('.', 'X', 'O'),
            vec!('?', 'x', 'o'),
            vec!(' ', ' ', ' ')));
        assert!(expanded.iter().any(|pat| *pat == pattern));
    }

}
