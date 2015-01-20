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

pub use self::Ruleset::AnySizeTrompTaylor;
pub use self::Ruleset::CGOS;
pub use self::Ruleset::KgsChinese;
pub use self::Ruleset::Minimal;

mod test;

#[derive(Clone, Show, Eq, PartialEq, Copy)]
pub enum Ruleset {
    AnySizeTrompTaylor,
    CGOS,
    KgsChinese,
    Minimal,
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
            Minimal            => true,
            _ => false
        }
    }
}
