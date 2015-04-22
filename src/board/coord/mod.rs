/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
        for i in 0..size {
            for j in 0..size {
                coords.push(Coord::new(j+1, i+1));
            }
        }
        coords
    }

    pub fn neighbours(&self, board_size: u8) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for i in -1i8..2 {
            for j in -1i8..2 {
                let (col, row) = (self.col as i8 + i, self.row as i8 + j);
                let potential_neighbour = Coord::new(col as u8, row as u8);
                if ((i == 0 && j !=0) || (i != 0 && j == 0)) && (potential_neighbour.is_inside(board_size)) {
                    neighbours.push(potential_neighbour);
                }
            }
        }
        neighbours
    }

    pub fn diagonals(&self, board_size: u8) -> Vec<Coord> {
        vec!(
            Coord::new(self.col-1, self.row-1),
            Coord::new(self.col+1, self.row-1),
            Coord::new(self.col+1, self.row+1),
            Coord::new(self.col-1, self.row+1)
                ).iter()
            .filter(|c| c.is_inside(board_size))
            .cloned()
            .collect()
    }

    pub fn to_index(&self, board_size: u8) -> usize {
        (self.col as usize-1 + (self.row as usize-1)*board_size as usize)
    }

    pub fn is_inside(&self, board_size: u8) -> bool {
        1 <= self.col && self.col <= board_size && 1 <= self.row && self.row <= board_size
    }

    pub fn distance_to_border(&self, board_size: u8) -> u8 {
        *[self.col-1, self.row-1, board_size - self.col, board_size - self.row]
            .iter()
            .min()
            .unwrap()
    }

    pub fn manhattan_distance_three_neighbours(&self, board_size: u8) -> Vec<Coord> {
        let offsets = vec!(
                                      (0, 3),
                             (-1, 2), (0, 2), (1, 2),
                    (-2, 1), (-1, 1), (0, 1), (1, 1), (2, 1),
            (-3,0), (-2, 0), (-1, 0),         (1, 0), (2, 0), (3,0),
                    (-2,-1), (-1,-1), (0,-1), (1,-1), (2,-1),
                             (-1,-2), (0,-2), (1,-2),
                                      (0,-3)
                );
        let calc: Vec<(isize, isize)> = offsets.iter()
            .map(|&(co,ro)| (self.col as isize + co, self.row as isize + ro))
            .filter(|&(co,ro)| co > 0 && ro > 0)
            .collect();
        calc.iter()
            .map(|&(c,r)| Coord::new(c as u8,r as u8))
            .filter(|c| c.is_inside(board_size))
            .collect()
    }

    // Note: there is no I column.
    pub fn from_gtp(gtp_vertex: &str) -> Coord {
        let col_letter = gtp_vertex.chars().next().unwrap().to_lowercase().next().unwrap();

        let col = if col_letter > 'i' {
            col_letter as u8 - 'a' as u8
        } else {
            col_letter as u8 - 'a' as u8 + 1
        };

        let row = (gtp_vertex[1..gtp_vertex.len()]).parse::<u8>().unwrap();

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

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("({},{})", self.col, self.row);
        s.fmt(f)
    }
}
