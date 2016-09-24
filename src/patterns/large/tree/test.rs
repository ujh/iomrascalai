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

#![cfg(test)]

pub use board::Coord;
pub use sgf::Parser;
pub use super::Tree;
pub use board::Move;

pub use hamcrest::*;
pub use std::path::Path;

describe! from_patterns {

    it "builds the correct tree" {
        let patterns = vec!("1.0 .OX#".parse().unwrap());
        let tree = Tree::from_patterns(patterns);
        let expected_tree = Tree {
            // root
            probability: 0.0,
            black: None,
            white: None,
            off_board: None,
            empty: Some(Box::new(Tree {
                // ply 1
                probability: 0.0,
                black: None,
                empty: None,
                off_board: None,
                white: Some(Box::new(Tree {
                    // ply 2
                    probability: 0.0,
                    white: None,
                    empty: None,
                    off_board: None,
                    black: Some(Box::new(Tree {
                        // ply 3
                        probability: 0.0,
                        black: None,
                        white: None,
                        empty: None,
                        off_board: Some(Box::new(Tree {
                            // ply 4
                            probability: 1.0,
                            black: None,
                            white: None,
                            empty: None,
                            off_board: None
                        }))
                    }))
                }))
            }))
        };
        assert_that(tree, is(equal_to(expected_tree)));
    }
}

describe! pattern_probability {

    before_each {
        // This pattern translates to:
        //
        // . O .
        // . . #
        // . X .
        //
        let patterns = vec!("1.0 .OX#".parse().unwrap());
        let tree = Tree::from_patterns(patterns);
        let parser = Parser::from_path(Path::new("fixtures/sgf/pattern/two-stones.sgf")).unwrap();
        let game = parser.game().unwrap();
        let board = game.board();
    }

    it "returns the pattern probability if it matches" {
        assert_that(tree.pattern_probability(&board, &Coord::from_gtp("e3")), is(equal_to(1.0)));
    }

    it "returns 0.0 if the pattern doesn't match" {
        assert_that(tree.pattern_probability(&board, &Coord::from_gtp("a1")), is(equal_to(0.0)));
    }

}
