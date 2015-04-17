/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Igor Polyakov                           *
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
use board::Black;
use board::Coord;
use board::White;
use sgf::parser::Parser;

use std::path::Path;

#[test]
fn sets_the_board_size_from_sgf() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/empty.sgf")).unwrap();
    let game  = parser.game().unwrap();
    assert_eq!(game.size(), 19);
}

#[test]
fn sets_the_komi_from_sgf() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/empty.sgf")).unwrap();
    let game  = parser.game().unwrap();
    assert_eq!(game.komi(), 6.5);
}

#[test]
fn play_handicap_stones() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/handicap.sgf")).unwrap();
    let game  = parser.game().unwrap();
    assert_eq!(game.board().color(&Coord::new(4,4)), Black);
    assert_eq!(game.board().color(&Coord::new(16,4)), Black);
    assert_eq!(game.board().color(&Coord::new(16,16)), Black);
}

#[test]
fn play_moves() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/twomoves.sgf")).unwrap();
    let game  = parser.game().unwrap();
    assert_eq!(game.board().color(&Coord::new(4, 15)), Black);
    assert_eq!(game.board().color(&Coord::new(16, 7)), White);
}

#[test]
fn finished_game() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/finished.sgf")).unwrap();
    let game   = parser.game().unwrap();
    assert!(game.is_over());
}
