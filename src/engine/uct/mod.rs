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
use board::NoMove;
use board::Pass;
use board::Resign;
use config::Config;
use engine::Engine;
use game::Game;
use patterns::Matcher;
use playout::Playout;
use self::node::Node;

use rand::weak_rng;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use thread_scoped::JoinGuard;
use thread_scoped::scoped;

mod node;

pub struct UctEngine {
    config: Arc<Config>,
    matcher: Arc<Matcher>,
    playout: Arc<Box<Playout>>,
    previous_node_count: usize,
    root: Node,
}

impl UctEngine {

    pub fn new(config: Arc<Config>, playout: Box<Playout>, matcher: Arc<Matcher>) -> UctEngine {
        UctEngine {
            config: config.clone(),
            matcher: matcher,
            playout: Arc::new(playout),
            previous_node_count: 0,
            root: Node::new(NoMove, config),
        }
    }

    fn set_new_root(&mut self, game: &Game, color: Color) {
        self.root = self.root.find_new_root(game, color);
    }

}

impl Engine for UctEngine {

    fn gen_move(&mut self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        if !self.config.uct.reuse_subtree {
            self.root = Node::root(game, color, self.config.clone());
        } else {
            self.previous_node_count = self.root.descendants();
            self.set_new_root(game, color);
            let reused_node_count = self.root.descendants();
            if self.config.log && self.previous_node_count > 0 {
                let percentage = reused_node_count as f32 / self.previous_node_count as f32;
                log!("Reusing {} nodes ({}%)", reused_node_count, percentage*100.0)
            }
        }
        if self.root.has_no_children() {
            if self.config.log {
                log!("No moves to simulate!");
            }
            sender.send(Pass(color)).unwrap();
            return;
        }
        let (send_result_to_main, receive_result_from_threads) = channel::<((Vec<usize>, Color, usize), Sender<(Vec<usize>, Vec<Move>, bool, usize)>)>();
        let (_guards, halt_senders) = spin_up(self.config.clone(), self.playout.clone(), game, send_result_to_main);
        loop {
            select!(
                _ = receiver.recv() => {
                    let m = finish(&self.root, game, color, sender, self.config.clone(), halt_senders);
                    self.set_new_root(&game.play(m).unwrap(), color);
                    break;
                },
                res = receive_result_from_threads.recv() => {
                    let ((path, winner, nodes_added), send_to_thread) = res.unwrap();
                    self.root.record_on_path(&path, winner, nodes_added);
                    let data = self.root.find_leaf_and_expand(game, self.matcher.clone());
                    match send_to_thread.send(data) {
                        Ok(_) => {},
                        Err(e) => {
                            if self.config.debug {
                                log!("[DEBUG] send_to_thread failed with {:?}", e);
                            }
                        }
                    }
                }
                )
        }
    }

    fn engine_type(&self) -> &'static str {
        "uct"
    }

    fn reset(&mut self) {
        self.previous_node_count = 0;
        self.root = Node::new(NoMove, self.config.clone());
    }

}

fn spin_up<'a>(config: Arc<Config>, playout: Arc<Box<Playout>>, game: &Game, send_to_main: Sender<((Vec<usize>, Color, usize), Sender<(Vec<usize>, Vec<Move>, bool, usize)>)>) -> (Vec<JoinGuard<'a, ()>>, Vec<Sender<()>>) {
    let mut guards = Vec::new();
    let mut halt_senders = Vec::new();
    for _ in 0..config.threads {
        let (send_halt, receive_halt) = channel::<()>();
        halt_senders.push(send_halt);
        let send_to_main = send_to_main.clone();
        let guard = spin_up_worker(config.clone(), playout.clone(), game.board(), send_to_main, receive_halt);
        guards.push(guard);
    }
    (guards, halt_senders)
}

fn spin_up_worker<'a>(config: Arc<Config>, playout: Arc<Box<Playout>>, board: Board, send_to_main: Sender<((Vec<usize>, Color, usize),Sender<(Vec<usize>, Vec<Move>, bool, usize)>)>, receive_halt: Receiver<()>) -> JoinGuard<'a, ()> {
    unsafe { scoped(move || {
        let mut rng = weak_rng();
        let (send_to_self, receive_from_main) = channel::<(Vec<usize>, Vec<Move>, bool, usize)>();
        // Send this empty message to get everything started
        send_to_main.send(((vec!(), Empty, 0), send_to_self.clone())).unwrap();
        loop {
            select!(
                _ = receive_halt.recv() => { break; },
                task = receive_from_main.recv() => {
                    let (path, moves, _, nodes_added) = task.unwrap();
                    let mut b = board.clone();
                    for &m in moves.iter() {
                        b.play_legal_move(m);
                    }
                    // Playout is smart enough to correctly handle the
                    // case where the game is already over.
                    let playout_result = playout.run(&mut b, None, &mut rng);
                    let winner = playout_result.winner();
                    let send_to_self = send_to_self.clone();
                    match send_to_main.send(((path, winner, nodes_added), send_to_self)) {
                        Ok(_) => {},
                        Err(e) => {
                            if config.debug {
                                log!("[DEBUG] send_to_main failed with {:?}", e);
                            }
                        }
                    }
                }
                )
        }
    })}
}

fn finish(root: &Node, game: &Game, color: Color, sender: Sender<Move>, config: Arc<Config>, halt_senders: Vec<Sender<()>>) -> Move {
    for halt_sender in halt_senders.iter() {
        halt_sender.send(()).unwrap();
    }

    if root.mostly_losses(config.uct.end_of_game_cutoff) {
        let m = if game.winner() == color {
            Pass(color)
        } else {
            Resign(color)
        };
        sender.send(m).unwrap();
        if config.log {
            log!("Almost all simulations were losses");
        }
        m
    } else {
        let best_node = root.best();
        if config.log {
            log!("{} simulations ({}% wins on average, {} nodes)", root.plays()-1, root.win_ratio()*100.0, root.descendants());
            log!("Returning the best move ({}% wins)", best_node.win_ratio()*100.0);
        }
        sender.send(best_node.m()).unwrap();
        best_node.m()
    }
}
