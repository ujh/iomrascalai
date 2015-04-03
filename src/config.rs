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

use playout::NoEyesPlayout;
use playout::Playout;
use ruleset::Minimal;
use ruleset::Ruleset;

pub struct Config {
    pub log: bool,
    pub playout: Box<Playout>,
    pub ruleset: Ruleset,
    pub threads: usize,
    pub uct_expand_after: usize,
}

impl Config {

    pub fn default() -> Config {
        Config {
            log: false,
            playout: Box::new(NoEyesPlayout::new()),
            ruleset: Minimal,
            threads: 1,
            uct_expand_after: 1,
        }
    }

}
