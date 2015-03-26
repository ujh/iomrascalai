/************************************************************************
 *                                                                      *
 * Copyright 2015 Thomas Poinsot, Urban Hafner                          *
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
use board::Play;
use playout::Playout;
use playout::SimplePlayout;
use playout::SimpleWithPassPlayout;
use ruleset::KgsChinese;

use rand::{Rng, weak_rng};
use test::Bencher;

#[test]
fn should_add_the_passed_moves_as_the_first_move() {
    let board = Board::new(9, 6.5, KgsChinese);
    let playout = SimplePlayout::new();
    let mut rng = weak_rng();
    let result = playout.run(&board, &Play(Black, 1, 1), &mut rng);
    assert_eq!(Play(Black, 1, 1), result.moves()[0]);
}

#[test]
fn max_moves() {
    let playout = SimplePlayout::new();
    assert_eq!(1083, playout.max_moves(19));
}

#[bench]
fn simple_09x09(b: &mut Bencher) {
    let board = Board::new(9, 6.5, KgsChinese);
    let playout = SimplePlayout::new();
    let mut rng = weak_rng();
    b.iter(|| playout.run(&board, &Play(Black, 1, 1), &mut rng));
}

#[bench]
fn simple_13x13(b: &mut Bencher) {
    let board = Board::new(13, 6.5, KgsChinese);
    let playout = SimplePlayout::new();
    let mut rng = weak_rng();
    b.iter(|| playout.run(&board, &Play(Black, 1, 1), &mut rng));
}

#[bench]
fn simple_19x19(b: &mut Bencher) {
    let board = Board::new(19, 6.5, KgsChinese);
    let playout = SimplePlayout::new();
    let mut rng = weak_rng();
    b.iter(|| playout.run(&board, &Play(Black, 1, 1), &mut rng));
}

#[bench]
fn with_pass_09x09(b: &mut Bencher) {
    let board = Board::new(9, 6.5, KgsChinese);
    let playout = SimpleWithPassPlayout::new();
    let mut rng = weak_rng();
    b.iter(|| playout.run(&board, &Play(Black, 1, 1), &mut rng));
}

#[bench]
fn with_pass_13x13(b: &mut Bencher) {
    let board = Board::new(13, 6.5, KgsChinese);
    let playout = SimpleWithPassPlayout::new();
    let mut rng = weak_rng();
    b.iter(|| playout.run(&board, &Play(Black, 1, 1), &mut rng));
}

#[bench]
fn with_pass_19x19(b: &mut Bencher) {
    let board = Board::new(19, 6.5, KgsChinese);
    let playout = SimpleWithPassPlayout::new();
    let mut rng = weak_rng();
    b.iter(|| playout.run(&board, &Play(Black, 1, 1), &mut rng));
}
