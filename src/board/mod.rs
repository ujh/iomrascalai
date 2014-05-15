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
use std::vec::Vec;

mod test;

#[deriving(Clone, Show, Eq)]
pub enum Color {
    White,
    Black,
    Empty
}

#[deriving(Clone, Eq)]
struct Coord {
    col: u8,
    row: u8
}

#[deriving(Clone, Eq)]
struct Stone {
    coord: Coord,
    color: Color,
    chain_id: int
}

#[deriving(Clone)]
struct Chain{
    points: Vec<Coord>
}

#[deriving(Clone)]
pub struct Board {
    komi: f32,
    size: u8,
    board: Vec<Stone>,
    chains: Vec<Chain>
}

impl Stone {
    fn with_color(c: Color, chain_id: int, col: u8, row: u8) -> Stone {
        Stone {color: c, chain_id: chain_id, coord: Coord {col: col, row: row}}
    }
}

impl Chain {
    fn new(first_point: &Stone) -> Chain {
        Chain {points: vec!(first_point.coord)}
    }
}

impl Board {
    pub fn new(size: uint, komi: f32) -> Board {
        let empty_board = Vec::from_fn(size*size, |i| Stone::with_color(Empty, -1, (i%size+1) as u8, (i/size+1) as u8));

        Board {
            komi: komi,
            size: size as u8,
            board: empty_board,
            chains: Vec::new()
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get<'a>(&'a self, col: u8, row: u8) -> Option<&'a Stone> {
        if 1 <= col && col <= self.size && 1 <= row && row <= self.size {
            Some(self.board.get((row as uint - 1) * self.size as uint + (col as uint - 1)))
        } else {
            None
        }
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, c: Color, col: u8, row: u8) -> Board {
        let mut new_state = (*self).clone();
        new_state.board.get_mut((row as uint - 1) * self.size as uint + (col as uint - 1)).color = c;
        new_state
    }

    fn neighbours<'a>(&'a self, p: &Stone) -> Vec<&'a Stone> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                if (i == 0 && j !=0) || (i != 0 && j == 0) {
                    let n = self.get(p.coord.col+i as u8, p.coord.row+j as u8);

                    if n.is_some() { neighbours.push(n.unwrap()); }
                }
            }
        }

        neighbours
    }

    pub fn show(&self) {
        // First we print the board
        for row in range(1u8, self.size+1).rev() {

            // Prints the row number
            print!("{:2} ", row);

            // Prints the actual row
            for col in range(1u8, self.size+1) {
                if self.get(col, row).unwrap().color == Empty {
                    let hoshis = &[4u,10,16];
                    if   hoshis.contains(&(row as uint)) && hoshis.contains(&(col as uint)) {print!("+ ")}
                    else                                                                {print!(". ")}
                } else if self.get(col, row).unwrap().color == White {print!("O ")}
                  else if self.get(col, row).unwrap().color == Black {print!("X ")}
            }
            println!("");
        }

        // Then we print the col numbers under the board
        print!("{:3}", "");
        for col in range(1, self.size+1) {
            print!("{:<2}", col);
        }

        println!("");
    }
}
