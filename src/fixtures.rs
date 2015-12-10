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

#![cfg(test)]

use std::path::Path;

use board::Board;
use game::Game;
use sgf::Parser;

pub fn load_game(filename: &'static str) -> Game {
    let expanded_filename = format!("fixtures/sgf/{}.sgf", filename);
    let path = Path::new(&expanded_filename);
    let parser = Parser::from_path(path).unwrap();
    parser.game().unwrap()
}

pub fn load_board(filename: &'static str) -> Board {
    load_game(filename).board()
}
