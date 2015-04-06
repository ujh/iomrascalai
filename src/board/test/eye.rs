/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
use board::Black;
use board::Coord;
use board::Pass;
use board::White;
use sgf::Parser;

#[test]
fn corner_no_enemies_is_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/corner-no-enemies.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(true, board.is_eye(&Coord::new(1,5), Black));
}

#[test]
fn corner_not_an_eye_is_no_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/corner-not-an-eye.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(false, board.is_eye(&Coord::new(1,5), Black));
}

#[test]
fn corner_one_enemy_is_no_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/corner-one-enemy.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(false, board.is_eye(&Coord::new(1,5), Black));
}

#[test]
fn edge_no_enemies_is_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/edge-no-enemies.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(true, board.is_eye(&Coord::new(1,3), Black));
}

#[test]
fn edge_not_an_eye_is_no_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/edge-not-an-eye.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(false, board.is_eye(&Coord::new(1,3), Black));
}

#[test]
fn edge_one_enemy_is_no_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/edge-one-enemy.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(false, board.is_eye(&Coord::new(1,3), Black));
}

#[test]
fn no_enemies_is_an_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/no-enemies.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(true, board.is_eye(&Coord::new(3,3), Black));
}

#[test]
fn not_an_eye_is_no_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/not-an-eye.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(false, board.is_eye(&Coord::new(3,3), Black));
}

#[test]
fn one_enemy_is_an_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/one-enemy.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(true, board.is_eye(&Coord::new(3,3), Black));
}

#[test]
fn two_enemies_is_not_an_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/two-enemies.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let board  = game.board();
    assert_eq!(false, board.is_eye(&Coord::new(3,3), Black));
}

#[test]
fn legal_moves_without_eyes_shouldnt_include_an_eye() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/eye/no-enemies.sgf")).unwrap();
    let game   = parser.game().unwrap();
    let mut board  = game.board();
    board.play(Pass(White));
    let moves  = board.legal_moves_without_eyes();
    assert!(!moves.iter().any(|m| !m.is_pass() && m.coord() == Coord::new(3, 3)));
}
