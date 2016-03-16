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

use rand::XorShiftRng;
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

type Payload = (Vec<usize>, Vec<Move>, usize, usize);
type Answer = (Vec<usize>, usize, PlayoutResult, usize);
type Response = (Answer, Sender<Payload>);

pub struct EngineImpl {
    config: Arc<Config>,
    halt_senders: Vec<Sender<()>>,
    id: usize,
    matcher: Arc<Matcher>,
    ownership: OwnershipStatistics,
    playout: Arc<Playout>,
    previous_node_count: usize,
    receive_from_threads: Receiver<Response>,
    root: Node,
    send_to_main: Sender<Response>,
    start: PreciseTime,
}

impl EngineImpl {

    pub fn new(config: Arc<Config>, matcher: Arc<Matcher>) -> EngineImpl {
        let (send_to_main, receive_from_threads) = channel();
        EngineImpl {
            config: config.clone(),
            halt_senders: vec!(),
            id: 0,
            matcher: matcher.clone(),
            ownership: OwnershipStatistics::new(config.clone(), 0, 0.0),
            playout: Arc::new(Playout::new(config.clone(), matcher.clone())),
            previous_node_count: 0,
            receive_from_threads: receive_from_threads,
            root: Node::new(NoMove, config),
            send_to_main: send_to_main,
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

    fn finish(&mut self, game: &Game, color: Color) -> (Move,usize) {
        self.id += 1;
        for halt_sender in &self.halt_senders {
            check!(self.config, halt_sender.send(()));
        }
        self.halt_senders = vec!();
        let msg = format!("{} simulations ({}% wins on average, {} nodes)", self.root.playouts(), self.root.win_ratio()*100.0, self.root.descendants());
        self.config.log(msg);
        let playouts = self.root.playouts();
        let m = self.best_move(game, color);
        self.set_new_root(&game.play(m).unwrap(), color);
        (m,playouts)
    }

    fn spin_up(&mut self, game: &Game) {
        self.halt_senders = vec!();
        for _ in 0..self.config.threads {
            self.spin_up_worker(game.board());
        }
    }

    fn spin_up_worker(&mut self, board: Board) {
        let (send_halt, receive_halt) = channel();
        self.halt_senders.push(send_halt);
        let config = self.config.clone();
        let playout = self.playout.clone();
        let send_to_main = self.send_to_main.clone();
        let id = self.id;
        spawn(move || {
            let mut rng = weak_rng();
            let (send_to_self, receive_from_main) = channel();
            // Send this empty message to get everything started
            init(config.clone(), send_to_main.clone(), send_to_self.clone(), id);
            loop {
                select!(
                    _ = receive_halt.recv() => { break; },
                    r = receive_from_main.recv() => {
                        check!(config, payload = r => {
                            run_playout(config.clone(), send_to_main.clone(), send_to_self.clone(), &board, payload, playout.clone(), &mut rng);
                        });
                    }
                )
            }
        });
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
        self.spin_up(game);
        loop {
            let win_ratio = {
                let (best, _) = self.root.best();
                best.win_ratio()
            };
            if timer.ran_out_of_time(win_ratio) {
                return self.finish(game, color);
            }
            let r = self.receive_from_threads.recv();
            check!(self.config, res = r => {
                let ((path, nodes_added, playout_result, id), send_to_thread) = res;
                // Ignore responses from the previous genmove
                if self.id == id {
                    self.ownership.merge(playout_result.score());
                    self.root.record_on_path(
                        &path,
                        nodes_added,
                        &playout_result);
                    let (path, moves, nodes_added) = self.root.find_leaf_and_expand(game, self.matcher.clone());
                    let data = (path, moves, nodes_added, self.id);
                    check!(self.config, send_to_thread.send(data));
                }
            });
        }
    }

    fn reset(&mut self, size: u8, komi: f32) {
        self.previous_node_count = 0;
        self.root = Node::new(NoMove, self.config.clone());
        self.ownership = OwnershipStatistics::new(self.config.clone(), size, komi);
    }

}


fn init(config: Arc<Config>, send_to_main: Sender<Response>, send_to_self: Sender<Payload>, id: usize) {
    let answer = (vec!(), 0, PlayoutResult::empty(), id);
    check!(config, send_to_main.send((answer, send_to_self)));
}

fn run_playout(config: Arc<Config>, send_to_main: Sender<Response>, send_to_self: Sender<Payload>, board: &Board, payload: Payload, playout: Arc<Playout>, mut rng: &mut XorShiftRng) {
    let (path, moves, nodes_added, id) = payload;
    let mut b = board.clone();
    for &m in moves.iter() {
        b.play_legal_move(m);
    }
    // Playout is smart enough to correctly handle the case where the
    // game is already over.
    let playout_result = playout.run(&mut b, None, &mut rng);
    let send_to_self = send_to_self.clone();
    check!(
        config,
        send_to_main.send(
            ((path, nodes_added, playout_result, id), send_to_self)
        )
    );
}
