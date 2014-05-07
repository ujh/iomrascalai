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

#[deriving(Clone)]
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
    pub fn play(&self, c: Color, col: uint, row: uint) -> Board {
        let mut new_state = (*self).clone();
        new_state.board.get_mut(col-1).get_mut(self.size-row).color = c;
        new_state
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_getting_a_valid_coord_returns_a_color(){
        let b = super::Board::new(19, 6.5);

        assert!(b.get(1,1).unwrap()   == super::Empty);
        assert!(b.get(10,10).unwrap() == super::Empty);
    }

    #[test]
    fn test_getting_invalid_coordinates_returns_None() {
        let b = super::Board::new(19, 6.5);

        assert!(b.get(14,21)          == None);
        assert!(b.get(21,14)          == None);
    }

    #[test]
    fn test_19_19_is_a_valid_coordinate(){
        let b = super::Board::new(19, 6.5);

        assert!(b.get(19,19).unwrap() == super::Empty);
    }

    #[test]
    fn test_0_0_is_not_a_valid_coordinate(){
        let b = super::Board::new(19, 6.5);
        
        assert!(b.get(0,0)            == None);
    }

    #[test]
    fn test_get_komi(){
        let b = super::Board::new(19, 6.5);

        assert!(b.komi() == 6.5f32)
    }

    #[test]
    fn test_play(){ 
        let mut b = super::Board::new(19, 6.5);
        
        b = b.play(super::White, 14, 14);
        assert!(b.get(14,14).unwrap() == super::White);
    }
}