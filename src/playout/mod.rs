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

use rand::random;

mod no_eyes;
mod simple;
mod test;

pub fn factory(opt: Option<String>) -> Box<Playout> {
    match opt {
        Some(s) => {
            match s.as_slice() {
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

    fn run(&self, b: &Board, initial_move: &Move) -> PlayoutResult {
        let mut board = b.clone();
        let mut played_moves = Vec::new();
        board.play(*initial_move);
        played_moves.push(*initial_move);
        let max_moves = self.max_moves(board.size());
        let mut move_count = 0;
        while !board.is_game_over() && move_count < max_moves {
            let m = self.select_move(&board);
            board.play(m);
            played_moves.push(m);
            move_count += 1;
        }
        PlayoutResult::new(played_moves, board.winner())
    }

    fn is_playable(&self, board: &Board, m: &Move) -> bool;
    fn include_pass(&self) -> bool;

    fn max_moves(&self, size: u8) -> usize {
        size as usize * size as usize * 3
    }

    fn select_move(&self, board: &Board) -> Move {
        let color = board.next_player();
        let vacant = board.vacant();
        let playable_move_exists =  vacant
            .iter()
            .map(|c| Play(color, c.col, c.row))
            .any(|m| board.is_legal(m).is_ok() && self.is_playable(board, &m));
        if playable_move_exists {
            loop {
                let add = if self.include_pass() {
                    1
                } else {
                    0
                };
                let r = random::<usize>() % (vacant.len() + add);
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

#[derive(Debug)]
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
