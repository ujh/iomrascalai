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

pub use self::pattern::Pattern;
use board::Board;
use board::Coord;
use self::tree::Tree;

mod pattern;
mod point;
mod test;
mod tree;

pub struct Matcher {
    tree: Tree
}

impl Matcher {

    pub fn new() -> Matcher {
        Self::with_patterns(Self::expand_patterns(Self::patterns()))
    }

    fn with_patterns(patterns: Vec<Pattern>) -> Matcher {
        Matcher { tree: Tree::from_patterns(patterns) }
    }

    pub fn pattern_count(&self, board: &Board, coord: &Coord) -> usize {
        self.tree.pattern_count(board, coord)
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
            Pattern::new([
                ['X', 'O', 'X'],
                ['.', '.', '.'],
                ['?', '?', '?']]),
            // hane pattern - non-cutting hane
            Pattern::new([
                ['X', 'O', '.'],
                ['.', '.', '.'],
                ['?', '.', '?']]),
            // hane pattern - magari
            Pattern::new([
                ['X', 'O', '?'],
                ['X', '.', '.'],
                ['x', '.', '?']]),
            // generic pattern - katatsuke or diagonal attachment; similar to magari
            Pattern::new([
                ['.', 'O', '.'],
                ['X', '.', '.'],
                ['.', '.', '.']]),
            // cut1 pattern (kiri] - unprotected cut
            Pattern::new([
                ['X', 'O', '?'],
                ['O', '.', 'o'],
                ['?', 'o', '?']]),
            // cut1 pattern (kiri] - peeped cut
            Pattern::new([
                ['X', 'O', '?'],
                ['O', '.', 'X'],
                ['?', '?', '?']]),
            // cut2 pattern (de]
            Pattern::new([
                ['?', 'X', '?'],
                ['O', '.', 'O'],
                ['o', 'o', 'o']]),
            // cut keima
            Pattern::new([
                ['O', 'X', '?'],
                ['o', '.', 'O'],
                ['?', '?', '?']]),
            // side pattern - chase
            Pattern::new([
                ['X', '.', '?'],
                ['O', '.', '?'],
                [' ', ' ', '?']]),
            // side pattern - block side cut
            Pattern::new([
                ['O', 'X', '?'],
                ['X', '.', 'O'],
                [' ', ' ', ' ']]),
            // side pattern - block side connection
            Pattern::new([
                ['?', 'X', '?'],
                ['x', '.', 'O'],
                [' ', ' ', ' ']]),
            // side pattern - sagari
            Pattern::new([
                ['?', 'X', 'O'],
                ['x', '.', 'x'],
                [' ', ' ', ' ']]),
            // side pattern - cut
            Pattern::new([
                ['?', 'O', 'X'],
                ['X', '.', 'O'],
                [' ', ' ', ' ']]))
    }

}
