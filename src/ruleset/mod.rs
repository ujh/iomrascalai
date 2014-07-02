/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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

#[deriving(Clone, Show, Eq, PartialEq)]
pub enum Ruleset {
    AnySizeTrompTaylor,
    KgsChinese,
    Minimal
}

impl Ruleset {

    pub fn game_over_play(&self) -> bool {
        match *self {
            Minimal => true,
            _ => false
        }
    }

    pub fn same_player(&self) -> bool {
        match *self {
            Minimal => true,
            _ => false
        }
    }

    pub fn suicide_allowed(&self) -> bool {
        match *self {
            AnySizeTrompTaylor => true,
            _ => false
        }
    }
}
