/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
use board::White;
use super::Move;
use super::Pass;
use super::Resign;

#[test]
fn parse_gtp_pass() {
    let m = Move::from_gtp("B", "PASS");
    assert_eq!(m, Pass(Black));
}

#[test]
fn parse_lower_case_gtp_pass() {
    let m = Move::from_gtp("B", "pass");
    assert_eq!(m, Pass(Black));
}

#[test]
// TODO: Will this ever happen?
fn parse_resign() {
    let m = Move::from_gtp("W", "resign");
    assert_eq!(m, Resign(White));
}

#[test]
fn produce_gtp_resign() {
    let m = Resign(White);
    assert_eq!("resign", m.to_gtp());
}

#[test]
fn extract_color_from_resign() {
    let m = Resign(White);
    assert_eq!(&White, m.color());
}

#[test]
#[should_fail]
fn fail_when_trying_to_get_coord_of_resign() {
    let m = Resign(White);
    m.coord(); // This should blow up
}

#[test]
fn is_resign_recognizes_resigns() {
    let m = Resign(White);
    assert!(m.is_resign());
}
