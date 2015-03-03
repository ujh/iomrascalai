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
use std::sync::mpsc::Sender;
use time::PreciseTime;

pub struct McEngine;

impl McEngine {
    pub fn new() -> McEngine {
        McEngine
    }
}

impl Engine for McEngine {
    fn gen_move(&self, color: Color, game: &Game, time_to_stop: i64, sender: Sender<Move>) {
        let moves = game.legal_moves_without_eyes();
        if moves.is_empty() {
            log!("No moves to simulate!");
            sender.send(Pass(color));
            return;
        }
        let start_time = PreciseTime::now();
        let mut stats = MoveStats::new(&moves, color);
        let mut counter = 0;
        loop {
            let m = moves[random::<usize>() % moves.len()];
            let g = game.play(m).unwrap();
            let mut playout = Playout::new(g.board());
            let winner = playout.run();
            if winner == color {
                stats.record_win(&m);
            } else {
                stats.record_loss(&m);
            }
            if counter % 100 == 0 && start_time.to(PreciseTime::now()).num_milliseconds() >= time_to_stop {
                break;
            }
            counter += 1;
        }
        log!("{} simulations", counter);
        // resign if 0% wins
        if stats.all_losses() {
            log!("All simulations were losses");
            sender.send(Resign(color));
        } else {
            let (m, s) = stats.best();
            log!("Returning the best move ({}% wins)", s.win_ratio()*100.0);
            sender.send(m);
        }
    }
}
