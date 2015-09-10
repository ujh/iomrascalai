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
use board::Board;
use board::Move;

mod test;

pub struct Matcher;

impl Matcher {

    pub fn new() -> Matcher {
        Matcher
    }

    fn expand_patterns(patterns: Vec<Pattern>) -> Vec<Pattern> {
        patterns.iter().flat_map(|pattern| pattern.expand()).collect()
    }


    // Patterns liften from michi.py
    // * X, O are colors
    // * . is an empty intersection
    // * x, o are the opposites of X,O (i.e. other color or empty)
    // * SPACE is off board
    // * ? is any color, empty intersection, or off board
    fn patterns() -> Vec<Pattern> {
        vec!(
            // hane pattern - enclosing hane
            Pattern::new(vec!("XOX",
                              "...",
                              "???")),
            // hane pattern - non-cutting hane
            Pattern::new(vec!("XO.",
                              "...",
                              "?.?")),
            // hane pattern - magari
            Pattern::new(vec!("XO?",
                              "X..",
                              "x.?")),
            // generic pattern - katatsuke or diagonal attachment; similar to magari
            Pattern::new(vec!(".O.",
                              "X..",
                              "...")),
            // cut1 pattern (kiri] - unprotected cut
            Pattern::new(vec!("XO?",
                              "O.o",
                              "?o?")),
            // cut1 pattern (kiri] - peeped cut
            Pattern::new(vec!("XO?",
                              "O.X",
                              "???")),
            // cut2 pattern (de]
            Pattern::new(vec!("?X?",
                              "O.O",
                              "ooo")),
            // cut keima
            Pattern::new(vec!("OX?",
                              "o.O",
                              "???")),
            // side pattern - chase
            Pattern::new(vec!("X.?",
                              "O.?",
                              "  ?")),
            // side pattern - block side cut
            Pattern::new(vec!("OX?",
                              "X.O",
                              "   ")),
            // side pattern - block side connection
            Pattern::new(vec!("?X?",
                              "x.O",
                              "   ")),
            // side pattern - sagari
            Pattern::new(vec!("?XO",
                              "x.x",
                              "   ")),
            // side pattern - cut
            Pattern::new(vec!("?OX",
                              "X.O",
                              "   ")),
            )
    }

    pub fn pattern_count(&self, _: &Board, _: &Move) -> usize {
        0
    }

}
