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
use board::Color;
use board::Empty;
use board::White;

mod test;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Point {
    Black,
    NotBlack,
    White,
    NotWhite,
    All,
    Empty,
    OffBoard
}

impl Point {

    pub fn from_char(c: &char) -> Point {
        match *c {
            'X' => Point::Black,
            'O' => Point::White,
            '?' => Point::All,
            'x' => Point::NotBlack,
            'o' => Point::NotWhite,
            '.' => Point::Empty,
            ' ' => Point::OffBoard,
            _   => panic!("Can't convert {:?} to PointPattern", c)
        }
    }

    pub fn matches(&self, opt_color: Option<Color>) -> bool {
        match opt_color {
            Some(color) => {
                match *self {
                    Point::Black => { color == Black }
                    Point::White => { color == White }
                    Point::All => { true }
                    Point::NotBlack => { color != Black }
                    Point::NotWhite => { color != White }
                    Point::Empty => { color == Empty }
                    Point::OffBoard => { false }
                }
            },
            None => *self == Point::OffBoard
        }
    }
}
