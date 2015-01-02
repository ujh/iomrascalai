/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
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
use board::White;
use game::Game;
use ruleset::KgsChinese;
use ruleset::Minimal;

mod ko;

#[test]
fn should_start_counting_moves_at_0() {
    let g = Game::new(5, 6.5, Minimal);
    assert_eq!(0, g.move_number());
}

#[test]
fn should_increment_move_count_by_1_for_each_move() {
    let mut g = Game::new(5, 6.5, Minimal);
    g = g.play(Play(Black, 1, 1)).unwrap();
    assert_eq!(1, g.move_number());
}

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

    assert!(play.is_err());
    assert_eq!(play.unwrap_err(), IllegalMove::SuicidePlay);
}

#[test]
fn next_player_should_return_board_next_player() {
    let g = Game::new(3, 6.5, KgsChinese);
    assert_eq!(g.board.next_player(), g.next_player());
}
