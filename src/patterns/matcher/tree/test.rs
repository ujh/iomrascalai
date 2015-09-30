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

pub use super::Pattern;
pub use super::Tree;

describe! from_patterns {

    it "builds the correct tree for a pattern with a single black stone" {
        let pattern = Pattern::new(vec!(
            vec!('X', '.', '.'),
            vec!('.', '.', '.'),
            vec!('.', '.', '.')));
        let tree = Tree::from_patterns(vec!(pattern));
        assert_that(tree, is(equal_to(Tree {
            // root
            count: 1,
            black: Some(Box::new(Tree {
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
                                        black: None,
                                        white: None,
                                        off_board: None,
                                        empty: Some(Box::new(Tree {
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
            empty: None,
            off_board: None
        })));
    }
}
