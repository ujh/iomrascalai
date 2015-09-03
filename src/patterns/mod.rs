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

use board::Board;
use board::Move;

mod test;

pub struct Matcher;

impl Matcher {

    pub fn new() -> Matcher {
        Matcher
    }

    // Patterns liften from michi.py
    // * X, O are colors
    // * . is an empty intersection
    // * x, o are the opposites of X,O (i.e. other color or empty)
    // * SPACE is off board
    // * ? is any color, empty intersection, or off board
    fn patterns() -> Vec<Vec<&'static str>> {
        vec!(
            // hane pattern - enclosing hane
            vec!("XOX",
                 "...",
                 "???"),
            // hane pattern - non-cutting hane
            vec!("XO.",
                 "...",
                 "?.?"),
            // hane pattern - magari
            vec!("XO?",
                 "X..",
                 "x.?"),
            // generic pattern - katatsuke or diagonal attachment; similar to magari
            vec!(".O.",
                 "X..",
                 "..."),
            // cut1 pattern (kiri] - unprotected cut
            vec!("XO?",
                 "O.o",
                 "?o?"),
            // cut1 pattern (kiri] - peeped cut
            vec!("XO?",
                 "O.X",
                 "???"),
            // cut2 pattern (de]
            vec!("?X?",
                 "O.O",
                 "ooo"),
            // cut keima
            vec!("OX?",
                 "o.O",
                 "???"),
            // side pattern - chase
            vec!("X.?",
                 "O.?",
                 "  ?"),
            // side pattern - block side cut
            vec!("OX?",
                 "X.O",
                 "   "),
            // side pattern - block side connection
            vec!("?X?",
                 "x.O",
                 "   "),
            // side pattern - sagari
            vec!("?XO",
                 "x.x",
                 "   "),
            // side pattern - cut
            vec!("?OX",
                 "X.O",
                 "   "),
            )
    }

    pub fn pattern_count(&self, _: &Board, _: &Move) -> usize {
        0
    }

}
