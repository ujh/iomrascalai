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

use board::Black;
use board::Color;
use board::Empty;
use board::White;

use std::fmt;

mod test;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Point {
    Black,
    White,
    Empty,
    OffBoard
}

impl Point {

    pub fn from_char(c: char) -> Self {
        match c {
            'X' => Point::Black,
            'O' => Point::White,
            '.' => Point::Empty,
            '#' => Point::OffBoard,
            _ => panic!("Can't convert {:?} to Point", c)
        }
    }

    pub fn matches(&self, opt_color: Option<Color>) -> bool {
        match opt_color {
            Some(color) => {
                match *self {
                    Point::Black => { color == Black },
                    Point::Empty => { color == Empty },
                    Point::OffBoard => false,
                    Point::White => { color == White },
                }
            },
            None => {
                match *self {
                    Point::Black => false,
                    Point::Empty => false,
                    Point::OffBoard => true,
                    Point::White => false,
                }
            }
        }
    }

}

impl fmt::Display for Point {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Point::Black => "X",
            Point::White => "O",
            Point::Empty => ".",
            Point::OffBoard => "#"
        };
        format!("{}", s).fmt(f)
    }

}
