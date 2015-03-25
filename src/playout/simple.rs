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

use board::Board;
use board::Move;
use super::Playout;

#[derive(Debug)]
pub struct SimplePlayout;

impl SimplePlayout {

    pub fn new() -> SimplePlayout {
        SimplePlayout
    }

}

impl Playout for SimplePlayout {

    fn is_playable(&self, _: &Board, _: &Move) -> bool {
        true
    }

    fn include_pass(&self) -> bool {
        false
    }

    fn playout_type(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct SimpleWithPassPlayout;

impl SimpleWithPassPlayout {

    pub fn new() -> SimpleWithPassPlayout {
        SimpleWithPassPlayout
    }

}

impl Playout for SimpleWithPassPlayout {

    fn is_playable(&self, _: &Board, _: &Move) -> bool {
        true
    }

    fn include_pass(&self) -> bool {
        true
    }

    fn playout_type(&self) -> String {
        format!("{:?}", self)
    }
}
