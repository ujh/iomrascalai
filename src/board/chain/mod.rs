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
use board::Coord;

use std::collections::HashSet;

mod test;

#[derive(Clone, Eq, PartialEq, Show)]
pub struct Chain {
    color:  Color,
    coords: Vec<Coord>,
    id:     usize,
    libs:   HashSet<Coord>,
}

impl Chain {
    pub fn new(id: usize, color: Color, c: Coord, libs: Vec<Coord>) -> Chain {
        Chain {
            color:  color,
            coords: vec!(c),
            id:     id,
            libs:   libs.into_iter().collect(),
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn coords<'a>(&'a self) -> &'a Vec<Coord> {
        &self.coords
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn add_liberty(&mut self, coord: Coord) {
        self.libs.insert(coord);
    }

    pub fn remove_liberty(&mut self, coord: Coord) {
        self.libs.remove(&coord);
    }

    pub fn merge(&mut self, c: &Chain) {
        self.coords.push_all(c.coords.as_slice());
        for &l in c.libs.iter() {
            self.libs.insert(l);
        }
    }

    pub fn is_captured(&self) -> bool {
        self.libs.len() == 0
    }

    pub fn show(&self) -> String {
        format!("{:<3}| {:?}, libs: {:?}, stones: {:?}", self.id, self.color, self.libs, self.coords)
    }
}
