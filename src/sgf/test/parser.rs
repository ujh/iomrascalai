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

#[cfg(test)]

use std::io::fs::File;

use sgf::parser::Parser;

fn empty_sgf() -> ~str {
    let contents = File::open(&Path::new("fixtures/sgf/empty.sgf")).read_to_str();
    match contents {
        Ok(c) => c,
        Err(c) => "".to_owned()
    }
}

#[test]
fn sets_the_board_size_from_sgf() {
    let parser = Parser::new(empty_sgf());
    let board  = parser.board();
    // Is there only assert! or do things like assert_equal! exist, too?
    assert!(board.size() == 19);
}
