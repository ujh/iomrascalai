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
use std::os::num_cpus;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;

mod amaf;
mod simple;

pub trait McEngine {

    fn mc_gen_move(&self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        let moves = Arc::new(game.legal_moves_without_eyes());
        let threads = num_cpus();
        if moves.is_empty() {
            log!("No moves to simulate!");
            sender.send(Pass(color));
            return;
        }
        let mut stats = MoveStats::new(&moves, color);
        let mut counter = 0;
        let (send_result, receive_result) = channel::<Playout>();
        let mut guards = Vec::new();
        let mut halt_senders = Vec::new();
        for i in range(0, threads) {
            let moves = moves.clone();
            let (send_halt, receive_halt) = channel::<()>();
            halt_senders.push(send_halt);
            let send_result = send_result.clone();
            let guard = thread::scoped(move || {
                loop {
                    if receive_halt.try_recv().is_ok() {
                        break;
                    } else {
                        let m = moves[random::<usize>() % moves.len()];
                        let playout = Playout::run(&game.board(), &m);
                        send_result.send(playout);
                    }
                }
            });
            guards.push(guard);
        }
        loop {
            select!(
                result = receive_result.recv() => {
                    let playout = result.unwrap();
                    let winner = playout.winner();
                    self.record_playout(&mut stats, &playout, winner == color);
                    counter += 1;
                },
                _ = receiver.recv() => {
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
                    for halt_sender in halt_senders.iter() {
                        halt_sender.send(());
                    }
                    break;
                }
                )
        }
    }

    fn record_playout(&self, &mut MoveStats, &Playout, bool);

}
