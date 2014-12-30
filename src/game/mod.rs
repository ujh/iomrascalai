/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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

use board::Black;
use board::Board;
use board::Color;
use board::Coord;
use board::Empty;
use board::IllegalMove;
use board::Move;
use board::White;
use board::ZobristHashTable;
use ruleset::Ruleset;
use score::Score;

use std::fmt;
use core::fmt::Show;
use std::rc::Rc;

mod test;

#[deriving(Clone)]
pub struct Game<'a> {
    board: Board<'a>,
    base_zobrist_table: Rc<ZobristHashTable>,
    move_number: u8
}

impl<'a> Game<'a> {
    pub fn new<'b>(size: u8, komi: f32, ruleset: Ruleset) -> Game<'b> {
        let base_zobrist_table = Rc::new(ZobristHashTable::new(size));
        let new_board = Board::new(size, komi, ruleset, base_zobrist_table.clone());

        Game {
            board: new_board,
            base_zobrist_table: base_zobrist_table,
            move_number: 0
        }
    }

    pub fn play<'b>(&'b self, m: Move) -> Result<Game<'b>, IllegalMove> {
        let new_board = self.board.play(m);

        match new_board {
            Ok(b) => {
                let mut new_game_state = self.clone();
                new_game_state.board = b;
                new_game_state.move_number += 1;
                Ok(new_game_state)
            },
            Err(m) => Err(m)
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        self.board.get_coord(Coord::new(col, row))
    }

    pub fn ruleset(&self) -> Ruleset {
        self.board.ruleset()
    }

    pub fn move_number(&self) -> u8 {
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

    pub fn show_chains(&self) {
        for c in self.board.chains().iter() {
            println!("{}", c.show());
        }
    }

    pub fn next_player(&self) -> Color {
        self.board.next_player()
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.board.legal_moves()
    }
}

impl<'a> Show for Game<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("komi: {}\n", self.komi());

        // First we print the board
        for row in range(1u8, self.board.size()+1).rev() {

            // Prints the row number
            s.push_str(format!("{:2} ", row).as_slice());

            // Prints the actual row
            for col in range(1u8, self.board.size()+1) {
                let current_coords = Coord::new(col, row);

                match self.board.get_coord(current_coords) {
                    Empty => {
                        let hoshis = &[4u8,10,16];
                        if  hoshis.contains(&row) && hoshis.contains(&col) {
                            s.push_str("+ ");
                        } else {
                            s.push_str(". ");
                        }
                    },
                    White => { s.push_str("O "); },
                    Black => { s.push_str("X "); }
                }
            }
            s.push_str("\n");
        }

        // Then we print the col numbers under the board
        s.push_str(format!("{:3}", "").as_slice());
        for col in range(1, self.board.size()+1) {
            s.push_str(format!("{:<2}", col).as_slice());
        }
        s.push_str("\n");
        s.fmt(f)
    }
}
