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
use board::Coord;

mod test;

pub struct Matcher {
    patterns: Vec<Pattern>
}

impl Matcher {

    pub fn new() -> Matcher {
        Self::with_patterns(Self::expand_patterns(Self::patterns()))
    }

    fn with_patterns(patterns: Vec<Pattern>) -> Matcher {
        Matcher { patterns: patterns }
    }

    pub fn pattern_count(&self, board: &Board, coord: &Coord) -> usize {
        self.patterns
            .iter()
            .filter(|pattern| pattern.matches(board, coord))
            .count()
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
            Pattern::new(vec!(
                vec!('X', 'O', 'X'),
                vec!('.', '.', '.'),
                vec!('?', '?', '?'))),
            // hane pattern - non-cutting hane
            Pattern::new(vec!(
                vec!('X', 'O', '.'),
                vec!('.', '.', '.'),
                vec!('?', '.', '?'))),
            // hane pattern - magari
            Pattern::new(vec!(
                vec!('X', 'O', '?'),
                vec!('X', '.', '.'),
                vec!('x', '.', '?'))),
            // generic pattern - katatsuke or diagonal attachment; similar to magari
            Pattern::new(vec!(
                vec!('.', 'O', '.'),
                vec!('X', '.', '.'),
                vec!('.', '.', '.'))),
            // cut1 pattern (kiri] - unprotected cut
            Pattern::new(vec!(
                vec!('X', 'O', '?'),
                vec!('O', '.', 'o'),
                vec!('?', 'o', '?'))),
            // cut1 pattern (kiri] - peeped cut
            Pattern::new(vec!(
                vec!('X', 'O', '?'),
                vec!('O', '.', 'X'),
                vec!('?', '?', '?'))),
            // cut2 pattern (de]
            Pattern::new(vec!(
                vec!('?', 'X', '?'),
                vec!('O', '.', 'O'),
                vec!('o', 'o', 'o'))),
            // cut keima
            Pattern::new(vec!(
                vec!('O', 'X', '?'),
                vec!('o', '.', 'O'),
                vec!('?', '?', '?'))),
            // side pattern - chase
            Pattern::new(vec!(
                vec!('X', '.', '?'),
                vec!('O', '.', '?'),
                vec!(' ', ' ', '?'))),
            // side pattern - block side cut
            Pattern::new(vec!(
                vec!('O', 'X', '?'),
                vec!('X', '.', 'O'),
                vec!(' ', ' ', ' '))),
            // side pattern - block side connection
            Pattern::new(vec!(
                vec!('?', 'X', '?'),
                vec!('x', '.', 'O'),
                vec!(' ', ' ', ' '))),
            // side pattern - sagari
            Pattern::new(vec!(
                vec!('?', 'X', 'O'),
                vec!('x', '.', 'x'),
                vec!(' ', ' ', ' '))),
            // side pattern - cut
            Pattern::new(vec!(
                vec!('?', 'O', 'X'),
                vec!('X', '.', 'O'),
                vec!(' ', ' ', ' '))),
            )
    }

}
