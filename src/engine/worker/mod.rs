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
use patterns::SmallPatternMatcher;
use playout::Playout;
use playout::PlayoutResult;
use super::prior;
use super::prior::Prior;

use rand::XorShiftRng;
use rand::weak_rng;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

pub enum DirectMessage {
    SpinDown,
    NewState {
        board: Board,
        id: usize,
    }
}

pub enum Message {
    RunPlayout {
        moves: Vec<Move>,
        nodes_added: usize,
        path: Vec<usize>,
    },
    CalculatePriors {
        child_moves: Vec<Move>,
        moves: Vec<Move>,
        path: Vec<usize>,
    }
}
pub enum Answer {
    RunPlayout {
        nodes_added: usize,
        path: Vec<usize>,
        playout_result: PlayoutResult
    },
    CalculatePriors {
        moves: Vec<Move>,
        path: Vec<usize>,
        priors: Vec<Prior>,
    },
    NewState
}
pub type Response = (Answer, usize, Sender<Message>);

pub struct Worker {
    board: Option<Board>,
    config: Arc<Config>,
    id: Option<usize>,
    playout: Arc<Playout>,
    rng: XorShiftRng,
    send_to_main: Sender<Response>,
    send_to_self: Option<Sender<Message>>,
    small_pattern_matcher: Arc<SmallPatternMatcher>,
}

impl Worker {

    pub fn new(config: &Arc<Config>, playout: &Arc<Playout>, small_pattern_matcher: &Arc<SmallPatternMatcher>, send_to_main: &Sender<Response>) -> Worker {
        let rng = weak_rng();
        Worker {
            board: None,
            config: config.clone(),
            id: None,
            playout: playout.clone(),
            rng: rng,
            send_to_main: send_to_main.clone(),
            send_to_self: None,
            small_pattern_matcher: small_pattern_matcher.clone(),
        }
    }

    pub fn run(&mut self, direct_messages: Receiver<DirectMessage>) {
        let (send_to_self, receive_from_main) = channel();
        self.send_to_self = Some(send_to_self);
        loop {
            select!(
                r = direct_messages.recv() => {
                    check!(self.config, direct_message = r => {
                        match direct_message {
                            DirectMessage::SpinDown => { break; },
                            DirectMessage::NewState {board, id} => {
                                self.set_new_state(board, id);
                            }
                        }
                    });
                },
                r = receive_from_main.recv() => {
                    check!(self.config, message = r => {
                        match message {
                            Message::RunPlayout {path, moves, nodes_added} => {
                                self.run_playout(path, moves, nodes_added);
                            },
                            Message::CalculatePriors {path, moves, child_moves} => {
                                self.run_prior_calculation(path, moves, child_moves);
                            }
                        }
                    });
                }
            );
        }

    }

    fn set_new_state(&mut self, board: Board, id: usize) {
        self.board = Some(board);
        self.id = Some(id);
        self.respond(Answer::NewState);
    }

    fn run_playout(&mut self, path: Vec<usize>, moves: Vec<Move>, nodes_added: usize) {
        let mut b = self.board.clone().expect("no board for run_playout");
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
        self.respond(answer);
    }

    fn run_prior_calculation(&self, path: Vec<usize>, moves: Vec<Move>, child_moves: Vec<Move>) {
        let mut b = self.board.clone().expect("no board for run_prior_calculation");
        for &m in moves.iter() {
            b.play_legal_move(m);
        }
        let priors = prior::calculate(b, child_moves, &self.small_pattern_matcher, &self.config);
        let answer = Answer::CalculatePriors {
            moves: moves,
            path: path,
            priors: priors,
        };
        self.respond(answer);
    }

    fn respond(&self, answer: Answer) {
        match self.send_to_self {
            Some(ref sender) => {
                let response = (answer, self.id.unwrap(), sender.clone());
                check!(self.config, self.send_to_main.send(response));
            }
            None => {
                panic!("Can't send message from Worker!")
            }
        }
    }
}
