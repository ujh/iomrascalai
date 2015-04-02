/************************************************************************
 *                                                                      *
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

use board::Board;
use board::Color;
use board::Empty;
use board::Move;
use board::Pass;
use board::Resign;
use config::Config;
use engine::Engine;
use game::Game;
use self::node::Node;

use rand::weak_rng;
use std::old_io::Writer;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;

mod node;

pub struct UctEngine {
    config: Arc<Config>
}

impl UctEngine {

    pub fn new(config: Arc<Config>) -> UctEngine {
        UctEngine { config: config }
    }

}

impl Engine for UctEngine {

    fn gen_move(&self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        let mut root = Node::root(game, color);
        let mut rng = weak_rng();
        if root.has_no_children() {
            if self.config.log {
                log!("No moves to simulate!");
            }
            sender.send(Pass(color));
            return;
        }
        let (send_result_to_main, receive_result_from_threads) = channel::<((Vec<usize>, Color), Sender<(Vec<usize>, Vec<Move>, bool)>)>();
        let (guards, halt_senders) = spin_up(self.config.clone(), game, color, send_result_to_main);
        loop {
            select!(
                _ = receiver.recv() => {
                    finish(root, game, color, sender, self.config.clone(), halt_senders);
                    break;
                },
                res = receive_result_from_threads.recv() => {
                    let ((path, winner), send_to_thread) = res.unwrap();
                    root.record_on_path(&path, winner);
                    let data = root.find_leaf_and_expand(game, self.config.uct_expand_after);
                    send_to_thread.send(data);
                }
                )
        }
    }

    fn engine_type(&self) -> &'static str {
        "uct"
    }
}

fn spin_up<'a>(config: Arc<Config>, game: &Game, color: Color, send_to_main: Sender<((Vec<usize>, Color), Sender<(Vec<usize>, Vec<Move>, bool)>)>) -> (Vec<thread::JoinGuard<'a, ()>>, Vec<Sender<()>>) {
    let mut guards = Vec::new();
    let mut halt_senders = Vec::new();
    for _ in 0..config.threads {
        let (send_halt, receive_halt) = channel::<()>();
        halt_senders.push(send_halt);
        let send_to_main = send_to_main.clone();
        let config = config.clone();
        let guard = spin_up_worker(config, game.board(), color, send_to_main, receive_halt);
        guards.push(guard);
    }
    (guards, halt_senders)
}

fn spin_up_worker<'a>(config: Arc<Config>, board: Board, color: Color, send_to_main: Sender<((Vec<usize>, Color),Sender<(Vec<usize>, Vec<Move>, bool)>)>, receive_halt: Receiver<()>) -> thread::JoinGuard<'a, ()> {
    thread::scoped(move || {
        let mut rng = weak_rng();
        let (send_to_self, receive_from_main) = channel::<(Vec<usize>, Vec<Move>, bool)>();
        // Send this empty message to get everything started
        send_to_main.send(((vec!(), Empty), send_to_self.clone()));
        loop {
            select!(
                _ = receive_halt.recv() => { break; },
                task = receive_from_main.recv() => {
                    let (path, moves, expanded) = task.unwrap();
                    // TODO: We do a clone here and then one when we
                    // call run(). One is enough!
                    let mut b = board.clone();
                    for &m in moves.iter() {
                        b.play_legal_move(m);
                    }
                    // Playout is smart enough to correctly handle the
                    // case where the game is already over.
                    let playout_result = config.playout.run(&b, None, &mut rng);
                    let winner = playout_result.winner();
                    let send_to_self = send_to_self.clone();
                    send_to_main.send(((path, winner), send_to_self));
                }
                )
        }
    })
}

fn finish(root: Node, game: &Game, color: Color, sender: Sender<Move>, config: Arc<Config>, halt_senders: Vec<Sender<()>>) {
    for halt_sender in halt_senders.iter() {
        halt_sender.send(());
    }
    if root.all_losses() {
        if game.winner() == color {
            sender.send(Pass(color));
        } else {
            sender.send(Resign(color));
        }
        if config.log {
            log!("All simulations were losses");
        }
    } else {
        let best_node = root.best();
        if config.log {
            log!("{} simulations ({}% wins on average)", root.plays()-1, root.win_ratio()*100.0);
            log!("Returning the best move({}% wins)", best_node.win_ratio()*100.0);
        }
        sender.send(best_node.m());
    }
}
