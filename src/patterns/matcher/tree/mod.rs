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

pub use super::Pattern;
use board::Black;
use board::Board;
use board::Color;
use board::Coord;
use board::Empty;
use board::White;

use core::fmt;

mod test;


#[derive(PartialEq)]
pub struct Tree {
    count: usize,
    black: Option<Box<Tree>>,
    white: Option<Box<Tree>>,
    empty: Option<Box<Tree>>,
    off_board: Option<Box<Tree>>,
}

impl Tree {

    pub fn empty() -> Tree {
        Tree {
            count: 0,
            black: None,
            white: None,
            empty: None,
            off_board: None
        }
    }

    pub fn from_patterns(patterns: Vec<Pattern>) -> Tree {
        match Self::build(&patterns, 0) {
            Some(tree) => *tree,
            None => Tree::empty()
        }
    }

    pub fn pattern_count(&self, board: &Board, coord: &Coord) -> usize {
        let colors = board.neighbours8_checked(*coord);
        self.walk(colors, 0, &self)
    }

    fn walk(&self, colors: Vec<Option<Color>>, i: usize, subtree: &Tree) -> usize {
        if colors.len() == i {
            return subtree.count;
        }
        let child = match colors[i] {
            Some(color) => {
                match color {
                    Black => &subtree.black,
                    White => &subtree.white,
                    Empty => &subtree.empty,
                }
            },
            None => &subtree.off_board
        };
        match child {
            &Some(ref c) => self.walk(colors, i + 1, c),
            &None => 0
        }
    }

    fn build(patterns: &Vec<Pattern>, level: usize) -> Option<Box<Tree>> {
        let count = patterns.len();
        if count == 0 || level > Pattern::size() {
            None
        } else {
            let bn = Self::build(
                &Self::filter_patterns(patterns, Some(Black), level),
                level + 1);
            let wn = Self::build(
                &Self::filter_patterns(patterns, Some(White), level),
                level + 1);
            let en = Self::build(
                &Self::filter_patterns(patterns, Some(Empty), level),
                level + 1);
            let obn = Self::build(
                &Self::filter_patterns(patterns, None, level),
                level + 1);
            let node = Tree {
                count: count,
                black: bn,
                white: wn,
                empty: en,
                off_board: obn
            };
            Some(Box::new(node))
        }
    }

    fn filter_patterns(patterns: &Vec<Pattern>, color: Option<Color>, level: usize) -> Vec<Pattern> {
        patterns.iter()
            .filter(|p| p.matches_color_at(color, level))
            .cloned()
            .collect()
    }

    fn as_string(&self, level: usize) -> String {
        let mut prefix = String::new();
        for _ in 0..level {
            prefix.push_str("    ");
        }
        let prefix1 = format!("{} +--", prefix);
        let black = format!("{}black{}", prefix1, match self.black {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        let white = format!("{}white{}", prefix1, match self.white {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        let empty = format!("{}empty{}", prefix1, match self.empty {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        let off_board = format!("{}off_board{}", prefix1, match self.off_board {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        format!("\n{}Tree({})\n{}\n{}\n{}\n{}", prefix, self.count, black, white, empty, off_board)
    }

}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string(0))
    }
}
