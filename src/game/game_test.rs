/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot                                        *
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
use board::SuicidePlay;
use board::White;
use board::move::Pass;
use board::move::Play;
use game::Game;
use ruleset::KgsChinese;
use ruleset::Minimal;

#[test]
fn game_score_should_include_komi() {
  let size = 5;
  let komi = 6.5;

  let mut g = Game::new(size, komi, Minimal);

  g = g.play(Play(Black, 2, 1)).unwrap();
  g = g.play(Play(White, 3, 1)).unwrap();
  g = g.play(Play(Black, 2, 2)).unwrap();
  g = g.play(Play(White, 3, 2)).unwrap();
  g = g.play(Play(Black, 1, 3)).unwrap();
  g = g.play(Play(White, 2, 3)).unwrap();
  g = g.play(Play(Black, 5, 4)).unwrap();
  g = g.play(Play(White, 1, 4)).unwrap();
  g = g.play(Play(Black, 4, 4)).unwrap();
  g = g.play(Play(White, 5, 3)).unwrap();
  g = g.play(Play(Black, 4, 5)).unwrap();
  g = g.play(Play(White, 4, 3)).unwrap();
  g = g.play(Play(Black, 1, 2)).unwrap();
  g = g.play(Play(White, 3, 4)).unwrap();
  g = g.play(Pass(Black)).unwrap();
  g = g.play(Play(White, 3, 5)).unwrap();
  g = g.play(Pass(Black)).unwrap();
  g = g.play(Pass(White)).unwrap();

  let (b_score, w_score) = g.score();
  assert_eq!(b_score, 9);
  assert_eq!(w_score, 16f32 + komi);
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
    assert_eq!(play.unwrap_err(), SuicidePlay);
}
