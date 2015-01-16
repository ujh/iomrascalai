/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
use core::fmt;
use std::cmp::Eq;

mod test;

#[derive(Clone, Hash, PartialEq, Eq, Copy, Ord, PartialOrd)]
pub struct Coord {
    pub col: u8,
    pub row: u8
}

impl Coord {
    pub fn new(col: u8, row: u8) -> Coord {
        Coord {col: col, row: row}
    }

    pub fn for_board_size(size: u8) -> Vec<Coord> {
        let mut coords = Vec::new();
        for i in range(0, size) {
            for j in range(0, size) {
                coords.push(Coord::new(j+1, i+1));
            }
        }
        coords
    }

    pub fn from_index(id: usize, board_size: u8) -> Coord {
        Coord {col: (id%board_size as usize + 1) as u8, row: (id/board_size as usize + 1) as u8}
    }

    pub fn neighbours(&self, board_size: u8) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for i in range(-1i8,2) {
            for j in range(-1i8,2) {
                let (col, row) = (self.col+i as u8, self.row+j as u8);
                let potential_neighbour = Coord::new(col, row);
                if ((i == 0 && j !=0) || (i != 0 && j == 0)) && (potential_neighbour.is_inside(board_size)) {
                    neighbours.push(potential_neighbour);
                }
            }
        }
        neighbours
    }

    pub fn to_index(&self, board_size: u8) -> usize {
        (self.col as usize-1 + (self.row as usize-1)*board_size as usize)
    }

    pub fn is_inside(&self, board_size: u8) -> bool {
        1 <= self.col && self.col <= board_size && 1 <= self.row && self.row <= board_size
    }

    // Note: there is no I column.
    pub fn from_gtp(gtp_vertex: &str) -> Coord {
        let col_letter = gtp_vertex.as_slice().char_at(0).to_lowercase();

        let col = if col_letter > 'i' {
            col_letter as u8 - 'a' as u8
        } else {
            col_letter as u8 - 'a' as u8 + 1
        };

        let row = (gtp_vertex.as_slice().slice(1, gtp_vertex.len())).parse::<u8>().expect("you must enter a valid coord (1 < c < 256)");

        Coord::new(col, row)
    }

    // Note: there is no I column.
    pub fn to_gtp(&self) -> String {
        let gtp_col = if self.col <= 8 {
            ('A' as u8 + self.col - 1) as char
        } else {
            ('A' as u8 + self.col) as char
        };
        let gtp_row = self.row;
        format!("{}{}", gtp_col, gtp_row)
    }
}

impl fmt::Show for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("({},{})", self.col, self.row);
        s.fmt(f)
    }
}
