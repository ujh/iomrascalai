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
        self.rotated()
            .iter()
            .chain(self.swapped().iter())
            .cloned()
            .collect()
    }

    fn rotated(&self) -> Vec<Pattern> {
        vec!(
            self.clone(),
            self.rotated90(),
            self.rotated180(),
            self.rotated270(),
            self.horizontally_flipped(),
            self.vertically_flipped())
    }

    fn swapped(&self) -> Vec<Pattern> {
        self.rotated()
            .iter()
            .map(|pat| pat.swap())
            .collect()
    }

    fn swap(&self) -> Pattern {
        let swapped_vec = self.vec
            .iter()
            .map(|subvec|
                 subvec.iter().map(|&c| self.swap_char(c)).collect())
            .collect();
        Pattern::new(swapped_vec)
    }

    fn swap_char(&self, c: char) -> char {
        match c {
            'x' => 'o',
            'X' => 'O',
            'o' => 'x',
            'O' => 'X',
            _   => c
        }
    }

    fn rotated90(&self) -> Pattern {
        let line1 = vec!(self.vec[2][0], self.vec[1][0], self.vec[0][0]);
        let line2 = vec!(self.vec[2][1], self.vec[1][1], self.vec[0][1]);
        let line3 = vec!(self.vec[2][2], self.vec[1][2], self.vec[0][2]);
        Pattern::new(vec!(line1, line2, line3))
    }

    fn rotated180(&self) -> Pattern {
        let line1 = vec!(self.vec[2][2], self.vec[2][1], self.vec[2][0]);
        let line2 = vec!(self.vec[1][2], self.vec[1][1], self.vec[1][0]);
        let line3 = vec!(self.vec[0][2], self.vec[0][1], self.vec[0][0]);
        Pattern::new(vec!(line1, line2, line3))
    }

    fn rotated270(&self) -> Pattern {
        let line1 = vec!(self.vec[0][2], self.vec[1][2], self.vec[2][2]);
        let line2 = vec!(self.vec[0][1], self.vec[1][1], self.vec[2][1]);
        let line3 = vec!(self.vec[0][0], self.vec[1][0], self.vec[2][0]);
        Pattern::new(vec!(line1, line2, line3))
    }

    fn horizontally_flipped(&self) -> Pattern {
        let line1 = vec!(self.vec[2][0], self.vec[2][1], self.vec[2][2]);
        let line2 = vec!(self.vec[1][0], self.vec[1][1], self.vec[1][2]);
        let line3 = vec!(self.vec[0][0], self.vec[0][1], self.vec[0][2]);
        Pattern::new(vec!(line1, line2, line3))
    }

    fn vertically_flipped(&self) -> Pattern {
        let line1 = vec!(self.vec[0][2], self.vec[0][1], self.vec[0][0]);
        let line2 = vec!(self.vec[1][2], self.vec[1][1], self.vec[1][0]);
        let line3 = vec!(self.vec[2][2], self.vec[2][1], self.vec[2][0]);
        Pattern::new(vec!(line1, line2, line3))
    }
}
