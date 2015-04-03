/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot, Igor Polyakov           *
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

pub use self::no_eyes::NoEyesPlayout;
pub use self::no_eyes::NoEyesWithPassPlayout;
pub use self::simple::SimplePlayout;
pub use self::simple::SimpleWithPassPlayout;
use board::Board;
use board::Color;
use board::Move;
use board::Pass;
use board::Play;

//use rand::random;
use rand::{Rng, XorShiftRng};

mod no_eyes;
mod simple;
mod test;

pub fn factory(opt: Option<String>) -> Box<Playout> {
    match opt {
        Some(s) => {
            match s.as_ref() {
                "no-eyes-with-pass" => Box::new(NoEyesWithPassPlayout::new()),
                "simple" => Box::new(SimplePlayout::new()),
                "simple-with-pass" => Box::new(SimpleWithPassPlayout::new()),
                _ => Box::new(NoEyesPlayout::new()),
            }
        },
        None => Box::new(NoEyesPlayout::new())
    }
}

pub trait Playout: Sync + Send {

    fn run(&self, board: &mut Board, initial_move: Option<&Move>, rng: &mut XorShiftRng) -> PlayoutResult {
        let mut played_moves = Vec::new();
        
        initial_move.map(|&m| {
            board.play_legal_move(m);
            played_moves.push(m);
        });

        let max_moves = self.max_moves(board.size());
        while !board.is_game_over() && played_moves.len() < max_moves {
            let m = self.select_move(&board, rng);
            board.play_legal_move(m);
            played_moves.push(m);
        }
        PlayoutResult::new(played_moves, board.winner())
    }

    fn is_playable(&self, board: &Board, m: &Move) -> bool;
    fn include_pass(&self) -> bool;

    fn max_moves(&self, size: u8) -> usize {
        size as usize * size as usize * 3
    }

    fn select_move(&self, board: &Board, rng: &mut XorShiftRng) -> Move {
        let color = board.next_player();
        let vacant = board.vacant();
        let playable_move =  vacant
            .iter()
            .map(|c| Play(color, c.col, c.row))
            .position(|m| board.is_legal(m).is_ok() && self.is_playable(board, &m));
        if playable_move.is_some() {
            loop {
                let add = if self.include_pass() {
                    1
                } else {
                    0
                };

                let first = playable_move.unwrap();
                let r = first + rng.gen::<usize>() % (vacant.len() + add - first);
                if r == vacant.len() {
                    return Pass(color);
                } else {
                    let c = vacant[r];
                    let m = Play(color, c.col, c.row);
                    if board.is_legal(m).is_ok() && self.is_playable(board, &m) {
                        return m;
                    }
                }
            }
        } else {
            Pass(color)
        }
    }

    fn playout_type(&self) -> String;

}

pub struct PlayoutResult {
    moves: Vec<Move>,
    winner: Color,
}

impl PlayoutResult {

    pub fn new(moves: Vec<Move>, winner: Color) -> PlayoutResult {
        PlayoutResult { moves: moves, winner: winner }
    }

    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn winner(&self) -> Color {
        self.winner
    }

}
