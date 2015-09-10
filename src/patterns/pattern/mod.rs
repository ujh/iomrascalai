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

mod test;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    vec: Vec<Vec<char>>
}

impl Pattern {

    pub fn new(vec: Vec<Vec<char>>) -> Pattern {
        Pattern { vec: vec }
    }

    pub fn expand(&self) -> Vec<Pattern> {
        vec!(
            self.clone(),
            self.rotated()
            )
    }

    fn rotated(&self) -> Pattern {
        let line1 = vec!(self.vec[2][0], self.vec[1][0], self.vec[0][0]);
        let line2 = vec!(self.vec[2][1], self.vec[1][1], self.vec[0][1]);
        let line3 = vec!(self.vec[2][2], self.vec[1][2], self.vec[0][2]);
        Pattern::new(vec!(line1, line2, line3))
    }
}
