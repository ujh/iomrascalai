/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner                                          *
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
use board::Coord;
use board::IllegalMove;
use board::Move;
use board::Play;
use ruleset::Ruleset;
use score::Score;
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
    move_number: u16,
    zobrist_hash_table: ZobristHashTable,
}

impl Game {
    pub fn new(size: u8, komi: f32, ruleset: Ruleset) -> Game {
        let new_board = Board::new(size, komi, ruleset);

        Game {
            board: new_board,
            move_number: 0,
            zobrist_hash_table: ZobristHashTable::new(size),
        }
    }

    pub fn play(&self, m: Move) -> Result<Game, IllegalMove> {
        let mut new_board = self.board.clone();

        match new_board.play(m) {
            Ok(_) => {
                let mut new_game_state = self.clone();
                new_game_state.board = new_board;
                new_game_state.move_number += 1;
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

    fn check_and_update_super_ko(&mut self, m: &Move) -> Result<(),()>{
        self.zobrist_hash_table.check_and_update_super_ko(m, &self.board)
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        self.board.color(&Coord::new(col, row))
    }

    pub fn ruleset(&self) -> Ruleset {
        self.board.ruleset()
    }

    pub fn move_number(&self) -> u16 {
        self.move_number
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

    pub fn score(&self) -> Score {
        self.board.score()
    }

    pub fn winner(&self) -> Color {
        self.board.winner()
    }

    pub fn set_komi(&mut self, komi: f32) {
        self.board.set_komi(komi);
    }

    pub fn board_size(&self) -> u8 {
        self.board.size()
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }

    pub fn show_chains(&self) {
        for c in self.board.chains().iter() {
            println!("{}", c.show());
        }
    }

    pub fn next_player(&self) -> Color {
        self.board.next_player()
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.board
            .legal_moves_without_superko_check()
            .into_iter()
            .filter(|&m| self.play(m).is_ok())
            .collect()
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

        s.push_str(self.board.as_string().as_slice());

        // Then we print the col numbers under the board
        s.push_str(format!("{:3}", "").as_slice());
        for col in range(1, self.board.size()+1) {
            s.push_str(format!("{:<2}", col).as_slice());
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
