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
pub use super::Engine;
pub use super::MoveStats;
use board::Board;
use board::Color;
use board::Move;
use board::Pass;
use board::Resign;
use config::Config;
use game::Game;
use playout::Playout;
use playout::PlayoutResult;

use rand::Rng;
use rand::weak_rng;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;

mod amaf;
mod simple;

pub trait McEngine {

    fn record_playout(&mut MoveStats, &PlayoutResult, bool);

}

fn gen_move<T: McEngine>(config: Arc<Config>, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
    let moves = game.legal_moves_without_eyes();
    if moves.is_empty() {
        if config.log {
            log!("No moves to simulate!");
        }
        sender.send(Pass(color)).unwrap();
        return;
    }
    let mut stats = MoveStats::new(&moves, color);
    let mut counter = 0;
    let (send_result, receive_result) = channel::<(MoveStats, usize)>();
    let (_guards, halt_senders) = spin_up::<T>(color, config.clone(), &moves, game, send_result);
    loop {
        select!(
            result = receive_result.recv() => {
                let (ms, count) = result.unwrap();
                stats.merge(&ms);
                counter += count;
            },
            _ = receiver.recv() => {
                let msg = finish(color, game, stats, sender, halt_senders);
                if config.log {
                    log!("{} simulations", counter);
                    log!("{}", msg);
                }
                break;
            }
            )
    }
}

fn finish(color: Color, game: &Game, stats: MoveStats, sender: Sender<Move>, halt_senders: Vec<Sender<()>>) -> String {
    for halt_sender in halt_senders.iter() {
        halt_sender.send(()).unwrap();
    }
    if stats.all_losses() {
        if game.winner() == color {
            sender.send(Pass(color)).unwrap();
        } else {
            sender.send(Resign(color)).unwrap();
        }
        String::from_str("All simulations were losses")
    } else {
        let (m, s) = stats.best();
        sender.send(m).unwrap();
        format!("Returning the best move ({}% wins)", s.win_ratio()*100.0)
    }
}

fn spin_up<'a, T: McEngine>(color: Color, config: Arc<Config>, moves: &'a Vec<Move>, game: &Game, send_result: Sender<(MoveStats, usize)>) -> (Vec<thread::JoinGuard<'a, ()>>, Vec<Sender<()>>) {
    let mut guards = Vec::new();
    let mut halt_senders = Vec::new();
    for _ in 0..config.threads {
        let (send_halt, receive_halt) = channel::<()>();
        halt_senders.push(send_halt);
        let send_result = send_result.clone();
        let config = config.clone();
        let guard = spin_up_worker::<T>(color, receive_halt, moves, game.board(), config, send_result);
        guards.push(guard);
    }
    (guards, halt_senders)
}

fn spin_up_worker<'a, T: McEngine>(color: Color, recv_halt: Receiver<()>, moves: &'a Vec<Move>, board: Board, config: Arc<Config>, send_result: Sender<(MoveStats, usize)>) -> thread::JoinGuard<'a, ()> {
    thread::scoped(move || {
        let runs = 100;
        let mut rng = weak_rng();
        let mut stats = MoveStats::new(moves, color);
        loop {
            for _ in 0..runs {
                let m = moves[rng.gen::<usize>() % moves.len()];
                let mut b = board.clone();
                let playout_result = config.playout.run(&mut b, Some(&m), &mut rng);
                let winner = playout_result.winner();
                T::record_playout(&mut stats, &playout_result, winner == color);
            }
            if recv_halt.try_recv().is_ok() {
                break;
            } else {
                send_result.send((stats, runs)).unwrap();
                stats = MoveStats::new(moves, color);
            }
        }
    })
}
