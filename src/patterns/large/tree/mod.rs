/************************************************************************
 *                                                                      *
 * Copyright 2016 Urban Hafner                                          *
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

use board::Black;
use board::Color;
use board::Empty;
use board::White;
use super::Pattern;

mod test;

pub struct Tree {
    probability: f32,
    black: Option<Box<Tree>>,
    white: Option<Box<Tree>>,
    empty: Option<Box<Tree>>,
    off_board: Option<Box<Tree>>,
}

impl Tree {

    pub fn empty() -> Self {
        Tree {
            probability: 0.0,
            black: None,
            white: None,
            empty: None,
            off_board: None,
        }
    }

    pub fn from_patterns(patterns: Vec<Pattern>) -> Tree {
        match Self::build(&patterns, 0) {
            Some(tree) => *tree,
            None => Tree::empty()
        }
    }

    fn build(patterns: &Vec<Pattern>, level: usize) -> Option<Box<Tree>> {
        let count = patterns.len();
        if count == 0  {
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
            let probability = Self::probability_at(patterns, level);
            let node = Tree {
                probability: probability,
                black: bn,
                white: wn,
                empty: en,
                off_board: obn
            };
            Some(Box::new(node))
        }
    }

    fn probability_at(patterns: &Vec<Pattern>, level: usize) -> f32 {
        let end_at_level: Vec<_> = patterns.iter()
            .filter(|p| p.len() == level)
            .map(|p| p.probability())
            .collect();
        if end_at_level.len() == 0 {
            0.0
        } else if end_at_level.len() == 1 {
            end_at_level[0]
        } else {
            panic!("{} patterns match at level {}", end_at_level.len(), level)
        }
    }

    fn filter_patterns(patterns: &Vec<Pattern>, color: Option<Color>, level: usize) -> Vec<Pattern> {
        patterns.iter()
            .filter(|p| p.matches_color_at(color, level))
            .cloned()
            .collect()
    }

}
