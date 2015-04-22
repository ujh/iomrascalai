/************************************************************************
 *                                                                      *
 * Copyright 2015 Igor Polyakov                                         *
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
use board::{Black, Coord, Play};
use sgf::Parser;

#[test]
fn bottom_left_is_ladder() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/ladders.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    let coord = Coord { col: 4, row: 4 };
    let chain = board.get_chain(coord).unwrap();

    assert_eq!(Play(Black, 4, 5), board.clone().capture_ladder(&chain).unwrap());
    assert_eq!(2, board.save_group(&chain).len());
}

#[test]
fn top_left_is_not_ladder() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/ladders.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    let coord = Coord { col: 4, row: 15 };
    let chain = board.get_chain(coord).unwrap();

    assert_eq!(None, board.clone().capture_ladder(&chain));
    assert_eq!(0, board.save_group(&chain).len());
}

#[test]
fn top_right_is_not_ladder() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/ladders.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    let coord = Coord { col: 15, row: 15 };
    let chain = board.get_chain(coord).unwrap();

    assert_eq!(None, board.clone().capture_ladder(&chain));
    assert_eq!(0, board.save_group(&chain).len());
}

#[test]
fn bottom_right_is_ladder() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/ladders.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    let coord = Coord { col: 16, row: 5 };
    let chain = board.get_chain(coord).unwrap();

    assert_eq!(Play(Black, 17, 5), board.clone().capture_ladder(&chain).unwrap());
    assert_eq!(2, board.save_group(&chain).len());
}