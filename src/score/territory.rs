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

use board::Color;
use board::Coord;
use board::Empty;

pub struct Territory {
    color:  Color,
    coords: Vec<Coord>,
}

impl Territory {

    pub fn new() -> Territory {
        Territory { color: Empty, coords: Vec::new() }
    }

    pub fn contains(&self, c: &Coord) -> bool {
        self.coords.contains(c)
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }

    pub fn set_color(&mut self, c: Color) {
        self.color = c;
    }

    pub fn add(&mut self, c: Coord) {
        self.coords.push(c)
    }

    pub fn dedup(&mut self) {
        self.coords.sort();
        self.coords.dedup();
    }
}
