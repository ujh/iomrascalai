/************************************************************************
 *                                                                      *
 * Copyright 2014-2015 Urban Hafner, Thomas Poinsot                     *
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
use board::Color;
use board::Pass;

use rand::random;

mod test;

pub struct Playout {
    moves: Vec<Move>,
    winner: Color,
}

impl Playout {
    pub fn new(b: &Board, initial_move: &Move) -> Playout {
        let mut board = b.clone();
        let mut played_moves = Vec::new();
        board.play(*initial_move);
        played_moves.push(*initial_move);
        let max_moves = board.size() * board.size() * 3;
        let mut move_count = 0;
        while !board.is_game_over() && move_count < max_moves {
            let moves = board.legal_moves_without_eyes();
            let m = if moves.is_empty() {
                let color = board.next_player();
                Pass(color)
            } else {
                moves[random::<usize>() % moves.len()]
            };
            board.play(m);
            played_moves.push(m);
            move_count += 1;
        }
        Playout {
            moves: played_moves,
            winner: board.winner(),
        }
    }

    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn winner(&self) -> Color {
        self.winner
    }
}
