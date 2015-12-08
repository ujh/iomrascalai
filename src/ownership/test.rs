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

pub use super::OwnershipStatistics;

describe! ownership {

    describe! statistics {

        // Tests for merge
        // Tests for formatting

        describe! formatting {

            before_each {
                let stats = OwnershipStatistics::new(3);
            }

            it "returns 0 by default" {
                let formatted = format!("{}", stats);

                assert_that(formatted, is(equal_to("0 0 0 \n0 0 0 \n0 0 0 \n".to_string())));
            }
        }
    }

}
