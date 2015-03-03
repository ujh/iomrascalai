/************************************************************************
 *                                                                      *
 * Copyright 2015 Thomas Poinsot                                        *
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

use playout::Playout;
use game::Game;
use ruleset::KgsChinese;
use test::Bencher;

#[bench]
fn bench_9x9_playout_speed(b: &mut Bencher) {
    let game = Game::new(9, 6.5, KgsChinese);
    let board = game.board();
    let mut playout_engine = Playout::new(board);

    b.iter(|| {playout_engine.run()})
}

#[bench]
fn bench_13x13_playout_speed(b: &mut Bencher) {
    let game = Game::new(13, 6.5, KgsChinese);
    let board = game.board();
    let mut playout_engine = Playout::new(board);

    b.iter(|| {playout_engine.run()})
}

#[bench]
fn bench_19x19_playout_speed(b: &mut Bencher) {
    let game = Game::new(19, 6.5, KgsChinese);
    let board = game.board();
    let mut playout_engine = Playout::new(board);

    b.iter(|| {playout_engine.run()})
}
