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

pub use super::Tree;

pub use hamcrest::*;

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
