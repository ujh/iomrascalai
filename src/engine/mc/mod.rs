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

pub use self::amaf::AmafMcEngine;
pub use self::simple::SimpleMcEngine;
pub use super::MoveStats;
pub use super::Engine;
use board::Color;
use game::Game;
use playout::Playout;
use board::Resign;
use board::Pass;
use board::Move;

use rand::random;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod amaf;
mod simple;

pub trait McEngine {

    fn mc_gen_move(&self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        let moves = game.legal_moves_without_eyes();
        if moves.is_empty() {
            log!("No moves to simulate!");
            sender.send(Pass(color));
            return;
        }
        let mut stats = MoveStats::new(&moves, color);
        let mut counter = 0;
        loop {
            let m = moves[random::<usize>() % moves.len()];
            let mut playout = Playout::new(game.board());
            let winner = playout.run(&m);
            self.record_playout(&mut stats, &playout, winner == color);
            counter += 1;
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

    fn record_playout(&self, &mut MoveStats, &Playout, bool);

}
