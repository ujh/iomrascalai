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
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct AmafEngine;

impl AmafEngine {

    pub fn new() -> AmafEngine {
        AmafEngine
    }

}

impl Engine for AmafEngine {

    fn gen_move(&self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        let moves = game.legal_moves_without_eyes();
        if moves.is_empty() {
            log!("No moves to simulate!");
            sender.send(Pass(color));
            return;
        }
        let mut stats = MoveStats::new(&moves, color);
        loop {
            let m = moves[random::<usize>() % moves.len()];
            let g = game.play(m).unwrap();
            let mut playout = Playout::new(g.board());
            let winner = playout.run();
            for m2 in playout.moves().iter() {
                if winner == color {
                    stats.record_win(&m2);
                } else {
                    stats.record_loss(&m2);
                }
            }
            if receiver.try_recv().is_ok() {
                break;
            }
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
