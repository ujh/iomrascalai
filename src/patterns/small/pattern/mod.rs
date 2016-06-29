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
    points: [Point; 8]
}

impl Pattern {

    pub fn new(vec: [[char; 3]; 3]) -> Pattern {
        let points = [
            Point::from_char(vec[0][0]),  // NW
            Point::from_char(vec[0][1]),  // N
            Point::from_char(vec[0][2]),  // NE
            Point::from_char(vec[1][2]),  // E
            Point::from_char(vec[2][2]),  // SE
            Point::from_char(vec[2][1]),  // S
            Point::from_char(vec[2][0]),  // SW
            Point::from_char(vec[1][0])]; // W
        Pattern { points: points }
    }

    pub fn raw(vec: [[Point; 3]; 3]) -> Pattern {
        let points = [
            vec[0][0],  // NW
            vec[0][1],  // N
            vec[0][2],  // NE
            vec[1][2],  // E
            vec[2][2],  // SE
            vec[2][1],  // S
            vec[2][0],  // SW
            vec[1][0]]; // W
        Pattern { points: points }
    }

    pub fn expand(&self) -> Vec<Pattern> {
        self.rotated()
            .iter()
            .chain(self.swapped().iter())
            .cloned()
            .collect()
    }

    pub fn matches_color_at(&self, color: Option<Color>, level: usize) -> bool {
        if level >= self.points.len() {
            false
        } else {
            self.points[level].matches(color)
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
        let mut swapped = [Point::Empty; 8];
        for i in 0..self.points.len() {
            swapped[i] = self.points[i].swap();
        }
        Pattern { points: swapped }
    }

    fn rotated90(&self) -> Pattern {
        Pattern::raw([
            [self.at(2,0), self.at(1,0), self.at(0,0)],
            [self.at(2,1), self.at(1,1), self.at(0,1)],
            [self.at(2,2), self.at(1,2), self.at(0,2)]])
    }

    fn rotated180(&self) -> Pattern {
        Pattern::raw([
            [self.at(2,2), self.at(2,1), self.at(2,0)],
            [self.at(1,2), self.at(1,1), self.at(1,0)],
            [self.at(0,2), self.at(0,1), self.at(0,0)]])
    }

    fn rotated270(&self) -> Pattern {
        Pattern::raw([
            [self.at(0,2), self.at(1,2), self.at(2,2)],
            [self.at(0,1), self.at(1,1), self.at(2,1)],
            [self.at(0,0), self.at(1,0), self.at(2,0)]])
    }

    fn horizontally_flipped(&self) -> Pattern {
        Pattern::raw([
            [self.at(2,0), self.at(2,1), self.at(2,2)],
            [self.at(1,0), self.at(1,1), self.at(1,2)],
            [self.at(0,0), self.at(0,1), self.at(0,2)]])
    }

    fn vertically_flipped(&self) -> Pattern {
        Pattern::raw([
            [self.at(0,2), self.at(0,1), self.at(0,0)],
            [self.at(1,2), self.at(1,1), self.at(1,0)],
            [self.at(2,2), self.at(2,1), self.at(2,0)]])
    }

    fn at(&self, row: usize, col: usize) -> Point {
        self.as_point_array()[row][col]
    }

    fn as_point_array(&self) -> [[Point; 3]; 3] {
        [
            [self.points[0], self.points[1],  self.points[2]],
            [self.points[7], Point::OffBoard, self.points[3]],
            [self.points[6], self.points[5],  self.points[4]]]
    }

}
