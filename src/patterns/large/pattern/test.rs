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

pub use hamcrest::*;
pub use super::Pattern;
pub use super::point::Point;

describe! from_str {

    it "sets the probability" {
        let pattern: Pattern = "1.0 .....".parse().unwrap();
        assert_that!(1.0, is(equal_to(pattern.probability())));
    }

    it "sets the points" {
        let pattern: Pattern = "1.0 .....".parse().unwrap();
        assert_that!(
            vec!(Point::Empty, Point::Empty, Point::Empty, Point::Empty, Point::Empty),
            is(equal_to(pattern.points))
        );
    }
}

describe! expand {

    it "doesn't produce duplicates" {
        let pattern: Pattern = "1.0 .....".parse().unwrap();
        assert_that!(1, is(equal_to(pattern.expand().len())));
    }

    describe! unique_symmetries {

        before_each {
            let pattern: Pattern = "1.0 ....X#..O#.#.".parse().unwrap();
            let expanded = pattern.expand();
        }

        it "produces all symmetries" {
            assert_that!(8, is(equal_to(expanded.len())));
        }

        it "includes the original pattern" {
            assert_that!(&expanded, contains(vec!(pattern)));
        }

        it "includes the 90deg rotation" {
            let rotated90deg = "1.0 .X....O#..##.".parse().unwrap();
            assert_that!(&expanded, contains(vec!(rotated90deg)));
        }

        it "includes the 180deg rotation" {
            let rotated180deg = "1.0 ...X.O..#.#.#".parse().unwrap();
            assert_that!(&expanded, contains(vec!(rotated180deg)));
        }

        it "includes the 270deg rotation" {
            let rotated270deg = "1.0 ..X...#O.#..#".parse().unwrap();
            assert_that!(&expanded, contains(vec!(rotated270deg)));
        }

        it "includes the mirrored original" {
            let mirrored = "1.0 ...X..#O.#..#".parse().unwrap();
            assert_that!(&expanded, contains(vec!(mirrored)));
        }

        it "includes the mirrored 90deg rotation" {
            let rotated90degmirrored = "1.0 .X...O..#.#.#".parse().unwrap();
            assert_that!(&expanded, contains(vec!(rotated90degmirrored)));
        }

        it "includes the mirrored 180deg rotation" {
            let rotated180degmirrored = "1.0 ....X.O#..##.".parse().unwrap();
            assert_that!(&expanded, contains(vec!(rotated180degmirrored)));
        }

        it "includes the mirrored 270deg rotation" {
            let rotated270degmirrored = "1.0 ..X..#..O#.#.".parse().unwrap();
            assert_that!(&expanded, contains(vec!(rotated270degmirrored)));
        }

    }

}
