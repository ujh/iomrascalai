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
        assert_that(1.0, is(equal_to(pattern.probability())));
    }

    it "sets the points" {
        let pattern: Pattern = "1.0 .....".parse().unwrap();
        assert_that(
            vec!(Point::Empty, Point::Empty, Point::Empty, Point::Empty, Point::Empty),
            is(equal_to(pattern.points))
        );
    }
}
