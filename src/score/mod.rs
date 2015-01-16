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

use board::Black;
use board::Board;
use board::Color;
use board::Coord;
use board::Empty;
use board::White;
use self::territory::Territory;

use core::fmt::String;
use std::fmt;
use std::num::Float;

mod territory;
mod test;

pub struct Score {
    black_stones: usize,
    komi:         f32,
    white_stones: usize,
}

impl Score {

    // Figure out which methods board needs to provide for this to
    // work so that we can create a trait with just those methods for
    // our test purposes.
    //
    // Store a reference to the Board in Score and compute the score
    // in an instance method.
    pub fn new(board: &Board) -> Score {
        let (bs, ws) = Score::score_tt(board);
        Score {
            black_stones: bs,
            komi:         board.komi(),
            white_stones: ws
        }
    }

    pub fn color(&self) -> Color {
        let white_adjusted = self.white_stones as f32 + self.komi;
        if self.black_stones as f32 == white_adjusted {
            Empty
        } else if self.black_stones as f32 > white_adjusted {
            Black
        } else {
            White
        }
    }

    pub fn white_stones(&self) -> usize {
        self.white_stones
    }

    pub fn black_stones(&self) -> usize {
        self.black_stones
    }

    fn score(&self) -> f32 {
        (self.black_stones as f32 - (self.white_stones as f32 + self.komi)).abs()
    }

    fn score_tt(board: &Board) -> (usize, usize) {
        let (black_stones, white_stones) = Score::count_stones(board);
        let (black_territory, white_territory) = Score::count_territory(board);
        let black_score = black_stones + black_territory;
        let white_score = white_stones + white_territory;
        (black_score, white_score)
    }

    fn count_territory(board: &Board) -> (usize, usize) {
        let mut black = 0;
        let mut white = 0;
        let mut empty_intersections = board.vacant().clone();
        while empty_intersections.len() > 0 {
            let territory = Score::build_territory_chain(empty_intersections[0], board);
            match territory.color() {
                Black => black += territory.size(),
                White => white += territory.size(),
                Empty => () // This territory is not enclosed by a single color
            }
            empty_intersections = empty_intersections
                .into_iter()
                .filter(|coord| !territory.contains(coord))
                .collect();
        }
        (black, white)
    }

    fn count_stones(board: &Board) -> (usize, usize) {
        let mut black = 0;
        let mut white = 0;
        for point in board.points().iter() {
            match point.color {
                Black => { black += 1; },
                Empty => {},
                White => { white += 1; },
            }
        }
        (black, white)
    }

    fn build_territory_chain(first_intersection: Coord, board: &Board) -> Territory {
        let mut territory_chain = Territory::new();
        let mut to_visit = Vec::new();
        let mut neutral  = false;

        to_visit.push(first_intersection);

        while to_visit.len() > 0 {
            let current_coord = to_visit.pop().unwrap();
            territory_chain.add(current_coord);
            for &coord in board.neighbours(current_coord).iter() {
                match board.color(&coord) {
                    Empty => if !territory_chain.contains(&coord) {to_visit.push(coord)},
                    col   => if territory_chain.color() != Empty && territory_chain.color() != col {
                        neutral = true;
                    } else {
                        territory_chain.set_color(col);
                    }
                }
            }
        }

        if neutral {
            territory_chain.set_color(Empty);
        } else {
            territory_chain.dedup();
        }

        territory_chain
    }

}

impl String for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self.color() {
            Black => "B+",
            White => "W+",
            Empty => ""
        };
        let score = format!("{}", self.score());
        let s = format!("{}{}", color, score);
        s.fmt(f)
    }
}
