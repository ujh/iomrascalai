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
pub use self::simple::SimplePlayout;
use board::Board;
use board::Move;
use board::Color;
use board::Pass;

use rand::{Rng, XorShiftRng};

mod no_eyes;
mod simple;
mod test;

pub fn factory(opt: Option<String>) -> Box<Playout> {
    match opt {
        Some(s) => {
            match s.as_slice() {
                "simple" => Box::new(SimplePlayout::new()),
                _ => Box::new(NoEyesPlayout::new()),
            }
        },
        None => Box::new(NoEyesPlayout::new())
    }
}


pub trait Playout: Sync + Send {

    fn run(&self, b: &Board, initial_move: &Move, rng: &mut XorShiftRng) -> PlayoutResult {
        let mut board = b.clone();
        let mut played_moves = Vec::new();
        board.play(*initial_move);
        played_moves.push(*initial_move);
        let max_moves = self.max_moves(board.size());
        let mut move_count = 0;
        while !board.is_game_over() && move_count < max_moves {
            let m = board.playout_move(rng);
            board.play(m);
            played_moves.push(m);
            move_count += 1;
        }
        PlayoutResult::new(played_moves, board.winner())
    }

    fn moves(&self, b: &Board) -> Vec<Move>;

    fn max_moves(&self, size: u8) -> usize {
        size as usize * size as usize * 3
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
