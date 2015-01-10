/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot                                        *
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
use board::coord::Coord;

mod test;

#[derive(Clone, Eq, PartialEq, Show)]
pub struct Chain {
    pub id   : usize,
    pub color: Color,
    pub libs : usize,
    coords   : Vec<Coord>
}

impl Chain {
    pub fn new(id: usize, color: Color) -> Chain {
        Chain {coords: Vec::new(), color: color, id: id, libs: 1}
    }

    pub fn add_stone(&mut self, coord: Coord) {
        self.coords.push(coord);
    }

    pub fn merge(&mut self, c: &Chain) {
        self.coords.push_all(c.coords.as_slice());
    }

    pub fn coords<'a>(&'a self) -> &'a Vec<Coord> {
        &self.coords
    }

    pub fn show(&self) -> String {
        self.coords
            .iter()
            .fold(
                format!("{:<3}| {:?}, libs: {:2}, stones: ", self.id, self.color, self.libs),
                |s, c| s + format!(" {},{} |", c.col, c.row).as_slice()
            )
    }
}
