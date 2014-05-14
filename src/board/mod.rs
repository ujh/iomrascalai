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
struct Point {
    col: uint,
    row: uint,
    color: Color,
    chain_id: int
}

#[deriving(Clone)]
struct Chain<'a> {
    points: Vec<&'a Point>
}

#[deriving(Clone)]
pub struct Board<'a> {
    komi: f32,
    size: uint,
    board: Vec<Vec<Point>>,
    chains: Vec<Chain<'a>>
}

impl Point {
    fn with_color(c: Color, chain_id: int, col: uint, row: uint) -> Point {
        Point {color: c, chain_id: chain_id, col:col, row: row}
    }
}

impl<'a> Chain<'a> {
    fn new(first_point: &'a Point) -> Chain<'a> {
        Chain {points: vec!(first_point)}
    }
}

impl<'a> Board<'a> {
    pub fn new(size: uint, komi: f32) -> Board {
        let empty_board = Vec::from_fn(size, |i| Vec::from_fn(size, |j| Point::with_color(Empty, -1, i+1, size-j)));

        Board {
            komi: komi,
            size: size,
            board: empty_board,
            chains: Vec::new()
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get<'b>(&'b self, col: uint, row: uint) -> Option<&'b Point> {
        if 1 <= col && col <= self.size && 1 <= row && row <= self.size {
            Some(self.board.get(col-1).get(self.size-row))
        } else {
            None
        }
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, c: Color, col: uint, row: uint) -> Board<'a> {
        let mut new_state = (*self).clone();
        new_state.board.get_mut(col-1).get_mut(self.size-row).color = c;
        new_state
    }

    fn neighbours(&'a self, p: &Point) -> Vec<&'a Point> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                if (i == 0 && j !=0) || (i != 0 && j == 0) {
                    let n = self.get(p.col+i as uint, p.row+j as uint);

                    if n.is_some() { neighbours.push(n.unwrap()); }
                }
            }
        }

        neighbours
    }

    pub fn show(&self) {
        let b = &self.board;

        // First we print the board
        for row in range(0, self.size) {

            // Prints the row number
            print!("{:2} ", self.size - row);

            // Prints the actual row
            for col in range(0, self.size) {
                if      b.get(col).get(row).color == Empty {
                    let hoshis = &[3u,9,15];
                    if   hoshis.contains(&row) && hoshis.contains(&col) {print!("+ ")}
                    else                                                {print!(". ")}
                } else if b.get(col).get(row).color == White {print!("O ")}
                  else if b.get(col).get(row).color == Black {print!("X ")}
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
