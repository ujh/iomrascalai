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
use board::Move;
use board::NoMove;
use board::Pass;
use board::Resign;
use config::Config;
use engine::Engine;
use game::Game;
use ownership::OwnershipStatistics;
use patterns::Matcher;
use playout::Playout;
use playout::PlayoutResult;
use self::node::Node;

use rand::weak_rng;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use thread_scoped::JoinGuard;
use thread_scoped::scoped;
use time::Duration;
use time::PreciseTime;

mod node;

pub struct EngineImpl {
    config: Arc<Config>,
    matcher: Arc<Matcher>,
    ownership: OwnershipStatistics,
    playout: Arc<Playout>,
    previous_node_count: usize,
    root: Node,
}

impl EngineImpl {

    pub fn new(config: Arc<Config>, matcher: Arc<Matcher>) -> EngineImpl {
        EngineImpl {
            config: config.clone(),
            matcher: matcher.clone(),
            ownership: OwnershipStatistics::new(config.clone(), 0),
            playout: Arc::new(Playout::new(config.clone(), matcher.clone())),
            previous_node_count: 0,
            root: Node::new(NoMove, config),
        }
    }

    fn set_new_root(&mut self, game: &Game, color: Color) {
        self.root = self.root.find_new_root(game, color);
    }

}

macro_rules! check {
    ($config:expr, $r:expr) => {
        check!($config, _unused_result = $r => {})
    };
    ($config:expr, $res:pat = $r:expr => $body:expr) => {
        match $r {
            Ok(res) => {
                let $res = res;
                $body
            },
            Err(e) => {
                $config.log(format!("[DEBUG] unwrap failed with {:?} at {}:{}", e, file!(), line!()));
            }
        }
    };

}

impl Engine for EngineImpl {

    fn ownership(&self) -> &OwnershipStatistics {
        &self.ownership
    }

    fn gen_move(&mut self, color: Color, budget_ms: u32, game: &Game, sender: Sender<(Move,usize)>, receiver: Receiver<()>) {
        self.config.gfx(self.ownership.gfx());
        self.ownership = OwnershipStatistics::new(self.config.clone(), game.size());
        let start = PreciseTime::now();
        let budget5 = Duration::milliseconds((budget_ms as f32 * 0.05) as i64);
        let budget20 = Duration::milliseconds((budget_ms as f32 * 0.2) as i64);
        if !self.config.tree.reuse_subtree {
            self.root = Node::root(game, color, self.config.clone());
        } else {
            self.previous_node_count = self.root.descendants();
            self.set_new_root(game, color);
            let reused_node_count = self.root.descendants();
            if self.previous_node_count > 0 {
                let percentage = reused_node_count as f32 / self.previous_node_count as f32;
                let msg = format!("Reusing {} nodes ({}%)", reused_node_count, percentage*100.0);
                self.config.log(msg);
            }
        }
        if self.root.has_no_children() {
            self.config.log(format!("No moves to simulate!"));
            sender.send((Pass(color), self.root.plays())).unwrap();
            return;
        }
        let (send_result_to_main, receive_result_from_threads) = channel::<((Vec<usize>, usize, PlayoutResult), Sender<(Vec<usize>, Vec<Move>, bool, usize)>)>();
        let (_guards, halt_senders) = spin_up(self.config.clone(), self.playout.clone(), game, send_result_to_main);
        loop {
            let win_ratio = self.root.best().win_ratio();
            if start.to(PreciseTime::now()) > budget5 && win_ratio > self.config.time_control.fastplay5_thres {
                self.config.log(format!("Search stopped. 5% rule triggered"));
                let m = finish(&self.root, game, color, sender, self.config.clone(), halt_senders);
                self.set_new_root(&game.play(m).unwrap(), color);
                break;
            } else if start.to(PreciseTime::now()) > budget20 && win_ratio > self.config.time_control.fastplay20_thres {
                self.config.log(format!("Search stopped. 20% rule triggered"));
                let m = finish(&self.root, game, color, sender, self.config.clone(), halt_senders);
                self.set_new_root(&game.play(m).unwrap(), color);
                break;
            }
            select!(
                _ = receiver.recv() => {
                    let m = finish(&self.root, game, color, sender, self.config.clone(), halt_senders);
                    self.set_new_root(&game.play(m).unwrap(), color);
                    break;
                },
                r = receive_result_from_threads.recv() => {
                    check!(self.config, res = r => {
                        let ((path, nodes_added, playout_result), send_to_thread) = res;
                        self.ownership.merge(playout_result.score());
                        self.root.record_on_path(
                            &path,
                            playout_result.winner(),
                            nodes_added,
                            playout_result.amaf());
                        let data = self.root.find_leaf_and_expand(game, self.matcher.clone());
                        check!(self.config, send_to_thread.send(data));
                    });
                }
                )
        }
    }

    fn reset(&mut self, boardsize: u8) {
        self.previous_node_count = 0;
        self.root = Node::new(NoMove, self.config.clone());
        self.ownership = OwnershipStatistics::new(self.config.clone(), boardsize);
    }

}

fn spin_up<'a>(config: Arc<Config>, playout: Arc<Playout>, game: &Game, send_to_main: Sender<((Vec<usize>, usize, PlayoutResult), Sender<(Vec<usize>, Vec<Move>, bool, usize)>)>) -> (Vec<JoinGuard<'a, ()>>, Vec<Sender<()>>) {
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

fn spin_up_worker<'a>(config: Arc<Config>, playout: Arc<Playout>, board: Board, send_to_main: Sender<((Vec<usize>, usize, PlayoutResult),Sender<(Vec<usize>, Vec<Move>, bool, usize)>)>, receive_halt: Receiver<()>) -> JoinGuard<'a, ()> {
    unsafe { scoped(move || {
        let mut rng = weak_rng();
        let (send_to_self, receive_from_main) = channel::<(Vec<usize>, Vec<Move>, bool, usize)>();
        // Send this empty message to get everything started
        check!(
            config,
            send_to_main.send(((vec!(), 0, PlayoutResult::empty()), send_to_self.clone())));
        loop {
            select!(
                _ = receive_halt.recv() => { break; },
                task = receive_from_main.recv() => {
                    check!(
                        config,
                        (path, moves, _unused, nodes_added) = task => {
                            let mut b = board.clone();
                            for &m in moves.iter() {
                                b.play_legal_move(m);
                            }
                            // Playout is smart enough to correctly handle the
                            // case where the game is already over.
                            let playout_result = playout.run(&mut b, None, &mut rng);
                            let send_to_self = send_to_self.clone();
                            check!(
                                config,
                                send_to_main.send(((path, nodes_added, playout_result), send_to_self)));
                        })
                }
                )
        }
    })}
}

fn finish(root: &Node, game: &Game, color: Color, sender: Sender<(Move,usize)>, config: Arc<Config>, halt_senders: Vec<Sender<()>>) -> Move {
    for halt_sender in halt_senders.iter() {
        check!(config, halt_sender.send(()));
    }

    if root.mostly_losses(config.tree.end_of_game_cutoff) {
        let m = if game.winner() == color {
            Pass(color)
        } else {
            Resign(color)
        };
        sender.send((m, root.plays())).unwrap();
        config.log(format!("Almost all simulations were losses"));
        m
    } else {
        let best_node = root.best();
        let msg = format!("{} simulations ({}% wins on average, {} nodes)", root.plays()-1, root.win_ratio()*100.0, root.descendants());
        config.log(msg);
        config.log(format!("Returning the best move ({}% wins)", best_node.win_ratio()*100.0));
        check!(config, sender.send((best_node.m(), root.plays())));
        best_node.m()
    }
}
