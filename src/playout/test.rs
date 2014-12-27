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
use board::Color;
use board::Move;
use board::Pass;
use board::Play;
use board::White;
use engine::Engine;
use game::Game;
use ruleset::KgsChinese;
use super::Playout;

struct PlayoutTestEngine;

impl Engine for PlayoutTestEngine {

    #[allow(unused_variables)]
    fn gen_move(&self, color: Color, game: &Game) -> Move {
        Pass(color)
    }
}

#[test]
fn run_should_return_white_as_the_winner_for_an_empty_board() {
    let engine = PlayoutTestEngine;
    let game = Game::new(3, 6.5, KgsChinese);
    let playout = Playout::new(&engine);
    assert_eq!(White, playout.run(&game));
}

#[test]
fn run_should_return_black_as_winner_with_one_move() {
    let engine = PlayoutTestEngine;
    let mut game = Game::new(3, 6.5, KgsChinese);
    game = game.play(Play(Black, 1, 1)).unwrap();
    let playout = Playout::new(&engine);
    assert_eq!(Black, playout.run(&game));
}
