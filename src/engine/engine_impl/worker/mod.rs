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
use config::Config;
use playout::Playout;
use playout::PlayoutResult;
use super::Answer;
use super::Payload;
use super::Response;

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
    playout: Arc<Playout>,
    rng: XorShiftRng,
    send_to_main: Sender<Response>,
    send_to_self: Option<Sender<Payload>>,
}

impl Worker {

    pub fn new(config: &Arc<Config>, playout: &Arc<Playout>, id: usize, board: Board, send_to_main: &Sender<Response>) -> Worker {
        let rng = weak_rng();
        Worker {
            board: board,
            config: config.clone(),
            id: id,
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
                        self.run_playout(message);
                    });
                }
            );
        }

    }

    fn init(&self) {
        let answer = (vec!(), 0, PlayoutResult::empty(), self.id);
        self.respond(answer);
    }

    fn run_playout(&mut self, message: Payload) {
        let (path, moves, nodes_added, id) = message;
        let mut b = self.board.clone();
        for &m in moves.iter() {
            b.play_legal_move(m);
        }
        // Playout is smart enough to correctly handle the case where
        // the game is already over.
        let playout_result = self.playout.run(&mut b, None, &mut self.rng);
        let answer = (path, nodes_added, playout_result, id);
        self.respond(answer);
    }

    fn respond(&self, answer: Answer) {
        match self.send_to_self {
            Some(ref sender) => {
                let response = (answer, sender.clone());
                check!(self.config, self.send_to_main.send(response));
            }
            None => {
                panic!("Can't send message from Worker!")
            }
        }
    }
}
