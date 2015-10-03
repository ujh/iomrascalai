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

use board::Color;
use super::point::Point;

mod test;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    vec: Vec<Point>
}

impl Pattern {

    pub fn size() -> usize {
        8
    }

    pub fn new(vec: Vec<Vec<char>>) -> Pattern {
        let v = vec!(
            Point::from_char(vec[0][0]),  // NW
            Point::from_char(vec[0][1]),  // N
            Point::from_char(vec[0][2]),  // NE
            Point::from_char(vec[1][2]),  // E
            Point::from_char(vec[2][2]),  // SE
            Point::from_char(vec[2][1]),  // S
            Point::from_char(vec[2][0]),  // SW
            Point::from_char(vec[1][0])); // W
        Pattern { vec: v }
    }

    pub fn raw(vec: Vec<Vec<Point>>) -> Pattern {
        let v = vec!(
            vec[0][0],  // NW
            vec[0][1],  // N
            vec[0][2],  // NE
            vec[1][2],  // E
            vec[2][2],  // SE
            vec[2][1],  // S
            vec[2][0],  // SW
            vec[1][0]); // W
        Pattern { vec: v }
    }

    pub fn expand(&self) -> Vec<Pattern> {
        self.rotated()
            .iter()
            .chain(self.swapped().iter())
            .cloned()
            .collect()
    }

    pub fn matches_color_at(&self, color: Option<Color>, level: usize) -> bool {
        if level >= self.vec.len() {
            false
        } else {
            self.vec[level].matches(color)
        }
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
            .map(|pp| self.swap_point_pattern(pp))
            .collect();
        Pattern { vec: swapped_vec }
    }

    fn swap_point_pattern(&self, p: &Point) -> Point {
        match *p {
            Point::NotBlack => Point::NotWhite,
            Point::Black => Point::White,
            Point::NotWhite => Point::NotBlack,
            Point::White => Point::Black,
            Point::All => Point::All,
            Point::Empty => Point::Empty,
            Point::OffBoard => Point::OffBoard
        }
    }

    fn rotated90(&self) -> Pattern {
        Pattern::raw(vec!(
            vec!(self.at(2,0), self.at(1,0), self.at(0,0)),
            vec!(self.at(2,1), self.at(1,1), self.at(0,1)),
            vec!(self.at(2,2), self.at(1,2), self.at(0,2))))
    }

    fn rotated180(&self) -> Pattern {
        Pattern::raw(vec!(
            vec!(self.at(2,2), self.at(2,1), self.at(2,0)),
            vec!(self.at(1,2), self.at(1,1), self.at(1,0)),
            vec!(self.at(0,2), self.at(0,1), self.at(0,0))))
    }

    fn rotated270(&self) -> Pattern {
        Pattern::raw(vec!(
            vec!(self.at(0,2), self.at(1,2), self.at(2,2)),
            vec!(self.at(0,1), self.at(1,1), self.at(2,1)),
            vec!(self.at(0,0), self.at(1,0), self.at(2,0))))
    }

    fn horizontally_flipped(&self) -> Pattern {
        Pattern::raw(vec!(
            vec!(self.at(2,0), self.at(2,1), self.at(2,2)),
            vec!(self.at(1,0), self.at(1,1), self.at(1,2)),
            vec!(self.at(0,0), self.at(0,1), self.at(0,2))))
    }

    fn vertically_flipped(&self) -> Pattern {
        Pattern::raw(vec!(
            vec!(self.at(0,2), self.at(0,1), self.at(0,0)),
            vec!(self.at(1,2), self.at(1,1), self.at(1,0)),
            vec!(self.at(2,2), self.at(2,1), self.at(2,0))))
    }

    fn at(&self, row: usize, col: usize) -> Point {
        self.as_point_array()[row][col]
    }

    fn as_point_array(&self) -> Vec<Vec<Point>> {
        vec!(
            vec!(self.vec[0], self.vec[1], self.vec[2]),
            vec!(self.vec[7], Point::OffBoard, self.vec[3]),
            vec!(self.vec[6], self.vec[5], self.vec[4]))
    }

}
