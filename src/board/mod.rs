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

#[deriving(Clone, Show, Eq)]
pub enum Color {
    White,
    Black,
    Empty
}

#[deriving(Clone)]
struct Point {
    color: Color
}

struct Chain<'a> {
    points: Vec<&'a Point>
}

pub struct Board {
    komi: f32,
    size: uint,
    board: Vec<Vec<Point>>
}

impl Point {
    fn new() -> Point {
        Point {color: Empty}
    }

    fn with_color(c: Color) -> Point {
        Point {color: c}
    }
}

impl<'a> Chain<'a> {
    fn new(first_point: &'a Point) -> Chain<'a> {
        Chain {points: vec!(first_point)}
    }
}

impl Board {
    pub fn new(size: uint, komi: f32) -> Board {
        let mut empty_line  = Vec::with_capacity(size);
        let mut empty_board = Vec::with_capacity(size);

        for _ in range(0, size) { empty_line.push(Point::new()) }
        for _ in range(0, size) { empty_board.push(empty_line.clone()) }

        Board {
            komi: komi,
            size: size,
            board: empty_board
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: uint, row: uint) -> Option<Color> {
        if 1 <= col && col <= self.size && 1 <= row && row <= self.size {
            Some(self.board.get(col-1).get(self.size-row).color)
        } else {
            None
        }
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&mut self, c: Color, col: uint, row: uint) {
        self.board.get_mut(col-1).get_mut(self.size-row).color = c;
    }

    pub fn show(&self) {
        let b = &self.board;

        for row in range(0, self.size) {
            print!("{:2} ", self.size - row);
            for col in range(0, self.size) {
                if      b.get(col).get(row).color == Empty {print!(". ")}
                else if b.get(col).get(row).color == White {print!("O ")}
                else if b.get(col).get(row).color == Black {print!("X ")}
            }
            println!("");
        }
    
        print!("{:3}", "");

        for col in range(1, self.size+1) {
            print!("{:<2}", col);
        }

        println!("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_board_get() {
        let b = super::Board::new(19, 6.5);

        assert!(b.get(1,1).unwrap()   == super::Empty);
        assert!(b.get(19,19).unwrap() == super::Empty);
        assert!(b.get(0,0)            == None);
        assert!(b.get(14,21)          == None);
        assert!(b.get(21,14)          == None);
    }

    #[test]
    fn test_get_komi(){
        let b = super::Board::new(19, 6.5);

        assert!(b.komi() == 6.5f32)
    }

    #[test]
    fn test_play(){
        let mut b = super::Board::new(19, 6.5);
        
        b.play(super::White, 14, 14);
        assert!(b.get(14,14).unwrap() == super::White);
    }
}