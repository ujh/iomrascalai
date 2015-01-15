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
#![cfg(test)]

use board::Black;
use board::Board;
use board::Pass;
use board::Play;
use board::White;
use ruleset::Minimal;

use super::Score;

#[test]
fn counting_simple_case() {
    let mut b = Board::new(4, 6.5, Minimal);

    b.play(Play(Black, 2, 1));
    b.play(Play(White, 3, 1));
    b.play(Play(Black, 2, 2));
    b.play(Play(White, 3, 2));
    b.play(Play(Black, 2, 3));
    b.play(Play(White, 3, 3));
    b.play(Play(Black, 2, 4));
    b.play(Play(White, 3, 4));
    b.play(Pass(Black));
    b.play(Pass(White));

    let score = b.score();
    assert_eq!(8, score.black_stones());
    assert_eq!(8, score.white_stones());
    assert_eq!(White, score.color());
    assert_eq!("W+6.5", format!("{}", score).as_slice());
}

#[test]
fn counting_disjoint_territory() {
    let mut b = Board::new(5, 6.5, Minimal);

    b.play(Play(Black, 2, 1));
    b.play(Play(White, 3, 1));
    b.play(Play(Black, 2, 2));
    b.play(Play(White, 3, 2));
    b.play(Play(Black, 1, 3));
    b.play(Play(White, 2, 3));
    b.play(Play(Black, 5, 4));
    b.play(Play(White, 1, 4));
    b.play(Play(Black, 4, 4));
    b.play(Play(White, 5, 3));
    b.play(Play(Black, 4, 5));
    b.play(Play(White, 4, 3));
    b.play(Play(Black, 1, 2));
    b.play(Play(White, 3, 4));
    b.play(Pass(Black));
    b.play(Play(White, 3, 5));
    b.play(Pass(Black));
    b.play(Pass(White));

    let score = b.score();
    assert_eq!(9, score.black_stones());
    assert_eq!(16, score.white_stones());
    assert_eq!(White, score.color());
    assert_eq!("W+13.5", format!("{}", score).as_slice());
}

#[test]
fn counting_with_neutral_points() {
    let mut b = Board::new(5, 6.5, Minimal);

    b.play(Play(Black, 2, 1));
    b.play(Play(White, 3, 1));
    b.play(Play(Black, 2, 2));
    b.play(Play(White, 3, 2));
    b.play(Play(Black, 1, 2));
    b.play(Play(White, 2, 3));
    b.play(Pass(Black));
    b.play(Play(White, 1, 4));
    b.play(Pass(Black));
    b.play(Pass(White));

    let score = b.score();
    assert_eq!(4, score.black_stones());
    assert_eq!(20, score.white_stones());
    assert_eq!(White, score.color());
    assert_eq!("W+22.5", format!("{}", score).as_slice());
}
