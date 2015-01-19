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
use board::Color;

use std::rand::random;

mod test;

pub struct Playout<'a> {
    board: Board<'a>
}

impl<'a> Playout<'a> {
    pub fn new(b: Board) -> Playout {
        Playout { board: b }
    }

    pub fn run(&self) -> Color {
        let mut board = self.board.clone();
        let max_moves = board.size() * board.size() * 3;
        let mut move_count = 0;
        while !board.is_game_over() && move_count < max_moves {
            let moves = board.legal_moves_without_superko_check();
            let m = moves[random::<usize>() % moves.len()];
            board.play(m);
            move_count += 1;
        }
        board.winner()
    }

}
