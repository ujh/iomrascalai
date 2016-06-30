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

use super::Pattern;

mod test;

pub struct Tree {
    probability: f32,
    black: Option<Box<Tree>>,
    white: Option<Box<Tree>>,
    empty: Option<Box<Tree>>,
    off_board: Option<Box<Tree>>,
}

impl Tree {

    pub fn empty() -> Self {
        Tree {
            probability: 0.0,
            black: None,
            white: None,
            empty: None,
            off_board: None,
        }
    }

    pub fn from_patterns(patterns: Vec<Pattern>) -> Self {
        Self::empty()
    }

}
