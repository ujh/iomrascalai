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

pub use super::Matcher;
pub use super::Pattern;

describe! expand_patterns {

    before_each {
        let pattern = Pattern::new(vec!(
            vec!('X', 'O', '.'),
            vec!('x', 'o', '?'),
            vec!(' ', ' ', ' ')));
        let patterns = vec!(pattern);
    }

    it "includes all variations" {
        let expanded = Matcher::expand_patterns(patterns);
        assert_that(expanded.len(), is(equal_to(12)));
    }

}
