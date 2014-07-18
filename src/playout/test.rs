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
use board::White;
use board::move::Pass;
use game::Game;
use ruleset::KgsChinese;
use super::Playout;

#[test]
fn run_should_return_the_winner() {
    let playout = Playout::new();
    let mut game = Game::new(3, 6.5, KgsChinese);
    game = game.play(Pass(Black)).unwrap();
    game = game.play(Pass(White)).unwrap();
    let winner = playout.run(game);
    assert_eq!(White, winner);
}
