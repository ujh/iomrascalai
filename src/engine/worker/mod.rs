/************************************************************************
 *                                                                      *
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

use board::Board;
use board::Move;
use config::Config;
use patterns::Matcher;
use playout::Playout;
use super::Answer;
use super::Message;
use super::Response;
use super::prior;

use rand::XorShiftRng;
use rand::weak_rng;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

pub struct Worker {
    board: Board,
    config: Arc<Config>,
    id: usize,
    matcher: Arc<Matcher>,
    playout: Arc<Playout>,
    rng: XorShiftRng,
    send_to_main: Sender<Response>,
    send_to_self: Option<Sender<Message>>,
}

impl Worker {

    pub fn new(config: &Arc<Config>, playout: &Arc<Playout>, matcher: &Arc<Matcher>, id: usize, board: Board, send_to_main: &Sender<Response>) -> Worker {
        let rng = weak_rng();
        Worker {
            board: board,
            config: config.clone(),
            id: id,
            matcher: matcher.clone(),
            playout: playout.clone(),
            rng: rng,
            send_to_main: send_to_main.clone(),
            send_to_self: None,
        }
    }

    pub fn run(&mut self, stop: Receiver<()>) {
        let (send_to_self, receive_from_main) = channel();
        self.send_to_self = Some(send_to_self);
        self.init();
        loop {
            select!(
                _ = stop.recv() => { break; },
                r = receive_from_main.recv() => {
                    check!(self.config, message = r => {
                        match message {
                            Message::RunPlayout {path, moves, nodes_added, id} => {
                                self.run_playout(path, moves, nodes_added, id);
                            },
                            Message::CalculatePriors {path, moves, child_moves, id} => {
                                self.run_prior_calculation(path, moves, child_moves, id);
                            }
                        }
                    });
                }
            );
        }

    }

    fn init(&self) {
        self.respond(Answer::SpinUp, self.id);
    }

    fn run_playout(&mut self, path: Vec<usize>, moves: Vec<Move>, nodes_added: usize, id: usize) {
        let mut b = self.board.clone();
        for &m in moves.iter() {
            b.play_legal_move(m);
        }
        // Playout is smart enough to correctly handle the case where
        // the game is already over.
        let playout_result = self.playout.run(&mut b, None, &mut self.rng);
        let answer = Answer::RunPlayout {
            nodes_added: nodes_added,
            path: path,
            playout_result: playout_result
        };
        self.respond(answer, id);
    }

    fn run_prior_calculation(&self, path: Vec<usize>, moves: Vec<Move>, child_moves: Vec<Move>, id: usize) {
        let mut b = self.board.clone();
        for &m in moves.iter() {
            b.play_legal_move(m);
        }
        let priors = prior::calculate(b, child_moves, &self.matcher, &self.config);
        let answer = Answer::CalculatePriors {
            moves: moves,
            path: path,
            priors: priors,
        };
        self.respond(answer, id);
    }

    fn respond(&self, answer: Answer, id: usize) {
        match self.send_to_self {
            Some(ref sender) => {
                let response = (answer, id, sender.clone());
                check!(self.config, self.send_to_main.send(response));
            }
            None => {
                panic!("Can't send message from Worker!")
            }
        }
    }
}