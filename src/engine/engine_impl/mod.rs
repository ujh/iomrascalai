/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
 * Copyright 2016 Urban Hafner                                          *
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

pub use self::node::Node;
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
use ruleset::KgsChinese;
use timer::Timer;

use rand::weak_rng;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread::spawn;
use time::PreciseTime;

mod node;

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

pub struct EngineImpl {
    config: Arc<Config>,
    matcher: Arc<Matcher>,
    ownership: OwnershipStatistics,
    playout: Arc<Playout>,
    previous_node_count: usize,
    root: Node,
    start: PreciseTime,
}

impl EngineImpl {

    pub fn new(config: Arc<Config>, matcher: Arc<Matcher>) -> EngineImpl {
        EngineImpl {
            config: config.clone(),
            matcher: matcher.clone(),
            ownership: OwnershipStatistics::new(config.clone(), 0, 0.0),
            playout: Arc::new(Playout::new(config.clone(), matcher.clone())),
            previous_node_count: 0,
            root: Node::new(NoMove, config),
            start: PreciseTime::now(),
        }
    }

    fn set_new_root(&mut self, game: &Game, color: Color) {
        self.root = self.root.find_new_root(game, color);
    }

    fn genmove_setup(&mut self, color: Color, game: &Game) {
        self.start = PreciseTime::now();
        self.config.gfx(self.ownership.gfx());
        self.ownership = OwnershipStatistics::new(self.config.clone(), game.size(), game.komi());
        self.previous_node_count = self.root.descendants();
        self.set_new_root(game, color);
        let reused_node_count = self.root.descendants();
        if self.previous_node_count > 0 {
            let percentage = reused_node_count as f32 / self.previous_node_count as f32;
            let msg = format!("Reusing {} nodes ({}%)", reused_node_count, percentage*100.0);
            self.config.log(msg);
        }
    }

    fn best_move(&self, game: &Game, color: Color) -> Move {
        let (best_node, pass) = self.root.best();
        let best_win_ratio = best_node.win_ratio();
        let pass_win_ratio = pass.win_ratio();
        let n = match game.ruleset() {
            KgsChinese => {
                if best_win_ratio > pass_win_ratio { best_node } else { pass }
            },
            _ => {
                // Only allow passing under Tromp/Taylor and CGOS
                // when we are winning.
                if game.winner() == color {
                    if best_win_ratio > pass_win_ratio { best_node } else { pass }
                } else {
                    best_node
                }
            }
        };
        let win_ratio = n.win_ratio();
        let msg = format!("Best move win ratio: {}%", win_ratio*100.0);
        self.config.log(msg);
        // Special case, when we are winning and all moves are played.
        if win_ratio == 0.0 {
            Pass(color)
        } else if win_ratio < 0.15 {
            Resign(color)
        } else {
            n.m()
        }
    }

    fn finish(&mut self, game: &Game, color: Color, halt_senders: Vec<Sender<()>>) -> (Move,usize) {
        for halt_sender in halt_senders.iter() {
            check!(self.config, halt_sender.send(()));
        }
        let msg = format!("{} simulations ({}% wins on average, {} nodes)", self.root.playouts(), self.root.win_ratio()*100.0, self.root.descendants());
        self.config.log(msg);
        let playouts = self.root.playouts();
        let m = self.best_move(game, color);
        self.set_new_root(&game.play(m).unwrap(), color);
        (m,playouts)
    }

}

impl Engine for EngineImpl {

    fn ownership(&self) -> &OwnershipStatistics {
        &self.ownership
    }

    fn genmove(&mut self, color: Color, game: &Game, timer: &Timer) -> (Move,usize) {
        self.genmove_setup(color, game);
        if self.root.has_no_children() {
            self.config.log(format!("No moves to simulate!"));
            return (Pass(color), self.root.playouts());
        }
        let (send_result_to_main, receive_result_from_threads) = channel::<((Vec<usize>, usize, PlayoutResult), Sender<(Vec<usize>, Vec<Move>, usize)>)>();
        let halt_senders = spin_up(self.config.clone(), self.playout.clone(), game, send_result_to_main);
        loop {
            let win_ratio = {
                let (best, _) = self.root.best();
                best.win_ratio()
            };
            if timer.ran_out_of_time(win_ratio) {
                return self.finish(game, color, halt_senders);
            }
            let r = receive_result_from_threads.recv();
            check!(self.config, res = r => {
                let ((path, nodes_added, playout_result), send_to_thread) = res;
                self.ownership.merge(playout_result.score());
                self.root.record_on_path(
                    &path,
                    nodes_added,
                    &playout_result);
                let data = self.root.find_leaf_and_expand(game, self.matcher.clone());
                check!(self.config, send_to_thread.send(data));
            });
        }
    }

    fn reset(&mut self, size: u8, komi: f32) {
        self.previous_node_count = 0;
        self.root = Node::new(NoMove, self.config.clone());
        self.ownership = OwnershipStatistics::new(self.config.clone(), size, komi);
    }

}

fn spin_up<'a>(config: Arc<Config>, playout: Arc<Playout>, game: &Game, send_to_main: Sender<((Vec<usize>, usize, PlayoutResult), Sender<(Vec<usize>, Vec<Move>, usize)>)>) -> Vec<Sender<()>> {
    let mut halt_senders = Vec::new();
    for _ in 0..config.threads {
        let (send_halt, receive_halt) = channel::<()>();
        halt_senders.push(send_halt);
        let send_to_main = send_to_main.clone();
        spin_up_worker(config.clone(), playout.clone(), game.board(), send_to_main, receive_halt);
    }
    halt_senders
}

fn spin_up_worker(config: Arc<Config>, playout: Arc<Playout>, board: Board, send_to_main: Sender<((Vec<usize>, usize, PlayoutResult),Sender<(Vec<usize>, Vec<Move>, usize)>)>, receive_halt: Receiver<()>){
    spawn(move || {
        let mut rng = weak_rng();
        let (send_to_self, receive_from_main) = channel::<(Vec<usize>, Vec<Move>, usize)>();
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
                        (path, moves, nodes_added) = task => {
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
    });
}
