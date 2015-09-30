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

use core::fmt;
pub use super::Pattern;

mod test;


#[derive(PartialEq)]
pub struct Tree {
    count: usize,
    black: Option<Box<Tree>>,
    white: Option<Box<Tree>>,
    empty: Option<Box<Tree>>,
    off_board: Option<Box<Tree>>,
}

impl Tree {

    pub fn from_patterns(patterns: Vec<Pattern>) -> Tree {
        Tree {
            count: 0,
            black: None,
            white: None,
            empty: None,
            off_board: None
        }
    }

    fn as_string(&self, level: usize) -> String {
        let mut prefix = String::new();
        for _ in 0..level {
            prefix.push_str("    ");
        }
        let prefix1 = format!("{} +--", prefix);
        let black = format!("{}black{}", prefix1, match self.black {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        let white = format!("{}white{}", prefix1, match self.white {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        let empty = format!("{}empty{}", prefix1, match self.empty {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        let off_board = format!("{}off_board{}", prefix1, match self.off_board {
            None => String::new(),
            Some(ref subtree) => subtree.as_string(level + 1)
        });
        format!("\n{}Tree({})\n{}\n{}\n{}\n{}", prefix, self.count, black, white, empty, off_board)
    }

}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string(0))
    }
}
