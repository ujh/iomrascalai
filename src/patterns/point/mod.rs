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

}
