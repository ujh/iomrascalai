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

pub use self::pattern::Pattern;
use config::Config;
use self::tree::Tree;

use std::sync::Arc;

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

    fn with_patterns(patterns: Vec<Pattern>, config: Arc<Config>) -> Self {
        config.write(format!("Building large pattern tree ... "));
        let matcher = Matcher { tree: Tree::from_patterns(patterns) };
        config.write(format!("done"));
        matcher
    }

    fn expand_patterns(patterns: Vec<Pattern>, config: Arc<Config>) -> Vec<Pattern> {
        config.write(format!("Loading the large patterns ... "));
        let patterns = patterns.iter().flat_map(|pattern| pattern.expand()).collect();
        config.log(format!("done"));
        patterns
    }

    fn patterns() -> Vec<Pattern> {
        Self::patterns_from_str(LARGE_PATTERN_INPUT)
    }

    fn patterns_from_str(input: &'static str) -> Vec<Pattern> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

}
