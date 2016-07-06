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

const LARGE_PATTERN_INPUT: &'static str = include_str!("patterns.input");
const PATH: &'static [(isize, isize)] = &[
    (0,0), // d=1
    (0,1), (0,-1), (1,0), (-1,0), // d=2
    (1,1), (-1,1), (1,-1), (-1,-1), // d=3
    (0,2), (0,-2), (2,0), (-2,0), // d=4
    (1,2), (-1,2), (1,-2), (-1,-2), (2,1), (-2,1), (2,-1), (-2,-1), // d=5
    (0,3), (0,-3), (2,2), (-2,2), (2,-2), (-2,-2), (3,0), (-3,0), // d=6
    (1,3), (-1,3), (1,-3), (-1,-3), (3,1), (-3,1), (3,-1), (-3,-1), // d=7
    (0,4), (0,-4), (2,3), (-2,3), (2,-3), (-2,-3), (3,2), (-3,2), (3,-2), (-3,-2), (4,0), (-4,0), // d=8
    (1,4), (-1,4), (1,-4), (-1,-4), (3,3), (-3,3), (3,-3), (-3,-3), (4,1), (-4,1), (4,-1), (-4,-1), // d=9
    (0,5), (0,-5), (2,4), (-2,4), (2,-4), (-2,-4), (4,2), (-4,2), (4,-2), (-4,-2), (5,0), (-5,0), // d=10
    (1,5), (-1,5), (1,-5), (-1,-5), (3,4), (-3,4), (3,-4), (-3,-4), (4,3), (-4,3), (4,-3), (-4,-3), (5,1), (-5,1), (5,-1), (-5,-1), // d=11
    (0,6), (0,-6), (2,5), (-2,5), (2,-5), (-2,-5), (4,4), (-4,4), (4,-4), (-4,-4), (5,2), (-5,2), (5,-2), (-5,-2), (6,0), (-6,0), // d=12
    (1,6), (-1,6), (1,-6), (-1,-6), (3,5), (-3,5), (3,-5), (-3,-5), (5,3), (-5,3), (5,-3), (-5,-3), (6,1), (-6,1), (6,-1), (-6,-1), // d=13
    (0,7), (0,-7), (2,6), (-2,6), (2,-6), (-2,-6), (4,5), (-4,5), (4,-5), (-4,-5), (5,4), (-5,4), (5,-4), (-5,-4), (6,2), (-6,2), (6,-2), (-6,-2), (7,0), (-7,0), // d=14
];

pub use self::pattern::Pattern;
use board::Board;
use board::Coord;
use config::Config;
use self::tree::Tree;

use rayon::prelude::*;
use std::sync::Arc;
use time::PreciseTime;

mod pattern;
mod test;
mod tree;

pub struct Matcher {
    tree: Tree
}

impl Matcher {

    pub fn new(config: Arc<Config>) -> Self {
        let patterns = Self::expand_patterns(Self::patterns(), config.clone());
        Self::with_patterns(patterns, config.clone())
    }

    #[test]
    pub fn test() -> Self {
        Matcher { tree: Tree::from_patterns(vec!()) }
    }

    pub fn pattern_probability(&self, board: &Board, coord: &Coord) -> f32 {
        self.tree.pattern_probability(board, coord)
    }

    fn with_patterns(patterns: Vec<Pattern>, config: Arc<Config>) -> Self {
        config.write(format!("Building the large pattern tree ... "));
        let start = PreciseTime::now();
        let matcher = Matcher { tree: Tree::from_patterns(patterns) };
        let duration = start.to(PreciseTime::now());
        config.log(format!("done (took {}s)", duration.num_seconds()));
        matcher
    }

    fn expand_patterns(patterns: Vec<Pattern>, config: Arc<Config>) -> Vec<Pattern> {
        config.write(format!("Loading the large patterns ... "));
        let start = PreciseTime::now();
        let mut expanded = vec!();
        patterns.par_iter()
            .map(|pattern| pattern.expand())
            .collect_into(&mut expanded);
        let duration = start.to(PreciseTime::now());
        config.log(format!("done (took {}s)", duration.num_seconds()));
        expanded.iter().flat_map(|iter| iter).cloned().collect()
    }

    fn patterns() -> Vec<Pattern> {
        Self::patterns_from_str(LARGE_PATTERN_INPUT)
    }

    fn patterns_from_str(input: &'static str) -> Vec<Pattern> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

}
