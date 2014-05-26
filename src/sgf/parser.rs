/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
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

pub struct Parser {
    sgf: String
}

#[deriving(Eq,Show)]
pub struct Property<'a> {
    pub name: &'a str,
    pub val:  &'a str
}

impl Parser {
    pub fn new(sgf: String) -> Parser {
        Parser {sgf: sgf}
    }

    pub fn board(&self) -> Board {
        Board::new(self.size(), self.komi())
    }

    fn size(&self) -> uint {
        let prop = self.tokenize().iter().find(|p| p.name == "SZ").unwrap();
        from_str(prop.val).unwrap()
    }

    fn komi(&self) -> f32 {
        let re = regex!(r"KM\[(\d+\.\d+)\]");
        let captures = re.captures(self.sgf.as_slice()).unwrap();
        from_str(captures.at(1)).unwrap()
    }

    pub fn tokenize<'a>(&'a self) -> Vec<Property<'a>> {
        let mut tokens = Vec::new();
        let mut prev_name = "";
        let re = regex!(r"([:upper:]{2})?\[([^]]+)\]");
        for caps in re.captures_iter(self.sgf.as_slice()) {
            if caps.at(1) == "" {
                tokens.push(Property {name: prev_name, val: caps.at(2)});
            } else {
                tokens.push(Property {name: caps.at(1), val: caps.at(2)});
                prev_name = caps.at(1);
            }
        }
        tokens
    }
}
