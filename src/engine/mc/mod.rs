/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot                          *
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

use board::Color;
use board::Move;
use board::Pass;
use board::Resign;
use game::Game;
use playout::Playout;
use super::Engine;
use super::MoveStats;

use rand::random;
use std::collections::HashMap;
use time::PreciseTime;

pub struct McEngine;

impl McEngine {
    pub fn new() -> McEngine {
        McEngine
    }
}

impl Engine for McEngine {
    fn gen_move(&self, color: Color, game: &Game, time_to_stop: i64) -> Move {
        let moves = game.legal_moves_without_eyes();
        if moves.is_empty() {
            return Pass(color)
        }
        let start_time = PreciseTime::now();
        let mut stats = HashMap::new();
        for m in moves.iter() {
            stats.insert(m, MoveStats::new());
        }
        let mut counter = 0;
        loop {
            let m = moves[random::<usize>() % moves.len()];
            let g = game.play(m).unwrap();
            let playout = Playout::new(g.board());
            let winner = playout.run();
            let mut prev_move_stats = stats.get_mut(&m).unwrap();
            if winner == color {
                prev_move_stats.won();
            } else {
                prev_move_stats.lost();
            }
            if counter % 100 == 0 && start_time.to(PreciseTime::now()).num_milliseconds() >= time_to_stop {
                break;
            }
            counter += 1;
        }
        // resign if 0% wins
        if stats.values().all(|stats| stats.all_losses()) {
            Resign(color)
        // pass if 100% wins
        } else if stats.values().all(|stats| stats.all_wins()) {
            Pass(color)
        } else {
            let mut m = Pass(color);
            let mut move_stats = MoveStats::new();
            for (m_new, ms) in stats.iter() {
                if ms.win_ratio() > move_stats.win_ratio() {
                    m = **m_new;
                    move_stats = *ms;
                }
            }
            m
        }
    }
}
