/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
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

use board::Black;
use board::IllegalMove;
use board::Pass;
use board::Play;
use board::Resign;
use board::White;
use game::Game;
use ruleset::KgsChinese;

mod ko;

#[test]
fn catch_suicide_moves_in_chinese() {
    let mut g = Game::new(3, 6.5, KgsChinese);

    g = g.play(Play(Black, 2, 2)).unwrap();
    g = g.play(Play(White, 1, 2)).unwrap();
    g = g.play(Play(Black, 2, 1)).unwrap();
    g = g.play(Play(White, 3, 2)).unwrap();
    g = g.play(Play(Black, 2, 3)).unwrap();
    g = g.play(Play(White, 3, 1)).unwrap();
    g = g.play(Pass(Black)).unwrap();
    g = g.play(Play(White, 1, 3)).unwrap();
    g = g.play(Pass(Black)).unwrap();

    let play = g.play(Play(White, 1, 1));

    match play {
        Err(e) => assert_eq!(e, IllegalMove::SuicidePlay),
        Ok(_)  => panic!("Expected Err!")
    }
}

#[test]
fn it_should_handle_resign() {
    let g = Game::new(9, 6.5, KgsChinese);
    let res = g.play(Resign(Black));
    assert!(res.is_ok());
}
