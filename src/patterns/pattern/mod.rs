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

use board::Black;
use board::Board;
use board::Coord;
use board::Empty;
use board::White;
use patterns::point::Point;

mod test;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    vec: Vec<Point>
}

impl Pattern {

    pub fn new(vec: Vec<Vec<char>>) -> Pattern {
        let mut v = vec!();
        // Can be done with flat_map, I think.
        for sv in vec.iter() {
            for c in sv.iter() {
                v.push(Point::from_char(c));
            }
        }
        Pattern { vec: v }
    }

    fn raw(vec: Vec<Point>) -> Pattern {
        Pattern { vec: vec }
    }

    pub fn matches(&self, board: &Board, coord: &Coord) -> bool {
        board.neighbours8_unchecked(*coord)
            .iter()
            .all(|nc| self.matches_at(board, coord, nc))
    }

    pub fn expand(&self) -> Vec<Pattern> {
        self.rotated()
            .iter()
            .chain(self.swapped().iter())
            .cloned()
            .collect()
    }

    fn matches_at(&self, board: &Board, coord: &Coord, neighbour: &Coord) -> bool {
        let point = self.point_for(coord, neighbour);
        let is_inside = neighbour.is_inside(board.size());
        if is_inside {
            let color = board.color(neighbour);
            point.matches(Some(color))
        } else {
            point.matches(None)
        }
    }

    fn point_for(&self, coord: &Coord, neighbour: &Coord) -> Point {
        let offset_col = coord.col as isize - neighbour.col as isize;
        let offset_row = coord.row as isize - neighbour.row as isize;
        let col = (1 - offset_col) as usize;
        let row = (1 + offset_row) as usize;
        self.at(row, col)
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
        Pattern::raw(swapped_vec)
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
            self.at(2,0), self.at(1,0), self.at(0,0),
            self.at(2,1), self.at(1,1), self.at(0,1),
            self.at(2,2), self.at(1,2), self.at(0,2)))
    }

    fn rotated180(&self) -> Pattern {
        Pattern::raw(vec!(
            self.at(2,2), self.at(2,1), self.at(2,0),
            self.at(1,2), self.at(1,1), self.at(1,0),
            self.at(0,2), self.at(0,1), self.at(0,0)))
    }

    fn rotated270(&self) -> Pattern {
        Pattern::raw(vec!(
            self.at(0,2), self.at(1,2), self.at(2,2),
            self.at(0,1), self.at(1,1), self.at(2,1),
            self.at(0,0), self.at(1,0), self.at(2,0)))
    }

    fn horizontally_flipped(&self) -> Pattern {
        Pattern::raw(vec!(
            self.at(2,0), self.at(2,1), self.at(2,2),
            self.at(1,0), self.at(1,1), self.at(1,2),
            self.at(0,0), self.at(0,1), self.at(0,2)))
    }

    fn vertically_flipped(&self) -> Pattern {
        Pattern::raw(vec!(
            self.at(0,2), self.at(0,1), self.at(0,0),
            self.at(1,2), self.at(1,1), self.at(1,0),
            self.at(2,2), self.at(2,1), self.at(2,0)))
    }

    fn at(&self, row: usize, col: usize) -> Point {
        self.vec[(row * 3) + col]
    }
}
