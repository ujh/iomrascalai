/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
use board::IllegalMove;
use board::Move;
use board::NoMove;
use ruleset::Ruleset;
use self::zobrist_hash_table::ZobristHashTable;

use std::fmt;
use core::fmt::Display;

mod zobrist_hash_table;
mod test;

pub trait Info {

    fn vacant_point_count(&self) -> u16;

}


#[derive(Clone)]
pub struct Game {
    board: Board,
    last_move: Move,
    zobrist_hash_table: ZobristHashTable,
}

impl Game {
    pub fn new(size: u8, komi: f32, ruleset: Ruleset) -> Game {
        let new_board = Board::new(size, komi, ruleset);

        Game {
            board: new_board,
            last_move: NoMove,
            zobrist_hash_table: ZobristHashTable::new(size),
        }
    }

    pub fn with_new_state(board: Board, zobrist_hash_table: ZobristHashTable, last_move: Move) -> Game {
        Game {
            board: board,
            last_move: last_move,
            zobrist_hash_table: zobrist_hash_table,
       }
    }

    pub fn play(&self, m: Move) -> Result<Game, IllegalMove> {
        let mut new_board = self.board.clone();

        match new_board.play(m) {
            Ok(_) => {
                let mut new_game_state = Game::with_new_state(new_board, self.zobrist_hash_table.clone(), m);
                if !m.is_pass() && !m.is_resign() {
                    match new_game_state.check_and_update_super_ko(&m) {
                        Err(_) => return Err(IllegalMove::SuperKo),
                        Ok(_) => {}
                    }
                }
                Ok(new_game_state)
            },
            Err(m) => Err(m)
        }
    }

    pub fn reset_game_over(&mut self) {
        self.board.reset_game_over();
    }

    fn check_and_update_super_ko(&mut self, m: &Move) -> Result<(),()>{
        self.zobrist_hash_table.check_and_update_super_ko(m, &self.board)
    }

    pub fn last_move(&self) -> Move {
        self.last_move
    }

    pub fn next_player(&self) -> Color {
        self.board.next_player()
    }

    pub fn is_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn komi(&self) -> f32 {
        self.board.komi()
    }

    pub fn size(&self) -> u8 {
        self.board.size()
    }

    pub fn winner(&self) -> Color {
        self.board.winner()
    }

    pub fn ruleset(&self) -> Ruleset {
        self.board.ruleset()
    }

    pub fn set_komi(&mut self, komi: f32) {
        self.board.set_komi(komi);
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }

    pub fn legal_moves_without_eyes(&self) -> Vec<Move> {
        self.board
            .legal_moves_without_eyes()
            .into_iter()
            .filter(|&m| self.play(m).is_ok())
            .collect()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("komi: {}\n", self.komi());

        s.push_str(self.board.as_string().as_ref());

        // Then we print the col numbers under the board
        s.push_str(format!("{:3}", "").as_ref());
        for col in 1..self.board.size()+1 {
            s.push_str(format!("{:<2}", col).as_ref());
        }
        s.push_str("\n");
        s.fmt(f)
    }
}

impl Info for Game {

    fn vacant_point_count(&self) -> u16 {
        self.board.vacant_point_count()
    }
}
