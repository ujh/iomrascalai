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
pub use hamcrest::equal_to;
pub use hamcrest::is;
pub use hamcrest::is_not;
pub use hamcrest::none;
pub use std::path::Path;

pub use board::Black;
pub use board::Board;
pub use board::Coord;
pub use board::Empty;
pub use board::White;
pub use sgf::Parser;
pub use super::Pattern;
pub use super::Tree;

pub fn black_tree() -> Tree {
    Tree {
        // root
        count: 1,
        empty: Some(Box::new(Tree {
            // ply 1
            count: 1,
            black: None,
            white: None,
            off_board: None,
            empty: Some(Box::new(Tree {
                // ply 2
                count: 1,
                black: None,
                white: None,
                off_board: None,
                empty: Some(Box::new(Tree {
                    // ply 3
                    count: 1,
                    black: None,
                    white: None,
                    off_board: None,
                    empty: Some(Box::new(Tree {
                        // ply 4
                        count: 1,
                        black: None,
                        white: None,
                        off_board: None,
                        empty: Some(Box::new(Tree {
                            // ply 5
                            count: 1,
                            black: None,
                            white: None,
                            off_board: None,
                            empty: Some(Box::new(Tree {
                                // ply 6
                                count: 1,
                                black: None,
                                white: None,
                                off_board: None,
                                empty: Some(Box::new(Tree {
                                    // ply 7
                                    count: 1,
                                    empty: None,
                                    white: None,
                                    off_board: None,
                                    black: Some(Box::new(Tree {
                                        // ply 8
                                        count: 1,
                                        black: None,
                                        white: None,
                                        off_board: None,
                                        empty: None
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        })),
        white: None,
        black: None,
        off_board: None
    }
}

describe! from_patterns {

    it "builds the correct tree for a pattern with a single black stone" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['X', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree, is(equal_to(black_tree())));
    }

    it "builds the correct tree for a wildcard pattern" {
        let pattern = Pattern::new([
            ['?', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree.black, is_not(none()));
        assert_that(tree.white, is_not(none()));
        assert_that(tree.empty, is_not(none()));
        assert_that(tree.off_board, is_not(none()));
    }

    it "builds the correct tree for a white pattern" {
        let pattern = Pattern::new([
            ['O', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree.black, is(none()));
        assert_that(tree.white, is_not(none()));
        assert_that(tree.empty, is(none()));
        assert_that(tree.off_board, is(none()));
    }

    it "builds the correct tree for an empty pattern" {
        let pattern = Pattern::new([
            ['.', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree.black, is(none()));
        assert_that(tree.white, is(none()));
        assert_that(tree.empty, is_not(none()));
        assert_that(tree.off_board, is(none()));
    }

    it "builds the correct tree for an off board pattern" {
        let pattern = Pattern::new([
            [' ', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree.black, is(none()));
        assert_that(tree.white, is(none()));
        assert_that(tree.empty, is(none()));
        assert_that(tree.off_board, is_not(none()));
    }

    it "builds the correct tree for a not-white pattern" {
        let pattern = Pattern::new([
            ['o', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree.black, is_not(none()));
        assert_that(tree.white, is(none()));
        assert_that(tree.empty, is_not(none()));
        assert_that(tree.off_board, is_not(none()));
    }

    it "builds the correct tree for a not-black pattern" {
        let pattern = Pattern::new([
            ['x', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.']]);
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree.black, is(none()));
        assert_that(tree.white, is_not(none()));
        assert_that(tree.empty, is_not(none()));
        assert_that(tree.off_board, is_not(none()));
    }

}

pub fn board_from_sgf(s: &str) -> Board {
    let parser = Parser::from_path(Path::new(&format!("fixtures/sgf/{}", s))).unwrap();
    let game = parser.game().unwrap();
    game.board()
}

describe! pattern_count {

    before_each {
        let center = &Coord::new(5,5);
    }

    it "matches one pattern" {
        let board = &board_from_sgf("3x3/one-black-w.sgf");
        assert_that(black_tree().pattern_count(board, center), is(equal_to(1)));
    }

}

describe! walk {

    it "finds one pattern" {
        let colors = vec!(
            Some(Empty),
            Some(Empty),
            Some(Empty),
            Some(Empty),
            Some(Empty),
            Some(Empty),
            Some(Empty),
            Some(Black));
        assert_that(black_tree().walk(colors, 0, &black_tree()), is(equal_to(1)));
    }
}
