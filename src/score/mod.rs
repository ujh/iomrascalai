/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
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

use core::fmt::String;
use std::fmt;
use std::num::Float;


pub struct Score {
    black_stones: usize,
    color:        Color,
    score:        f32,
    white_stones: usize,
}

struct Territory {
    color:  Color,
    coords: Vec<Coord>,
}

impl Territory {

    pub fn new() -> Territory {
        Territory { color: Empty, coords: Vec::new() }
    }
}

mod test;

impl Score {

    // Figure out which methods board needs to provide for this to
    // work so that we can create a trait with just those methods for
    // our test purposes.
    //
    // Store a reference to the Board in Score and compute the score
    // in an instance method.
    pub fn new(board: &Board) -> Score {
        let (bs, ws) = Score::score_tt(board);
        let b_score = bs as f32;
        let w_score = (ws as f32) + board.komi();
        let score = (b_score - w_score).abs();
        let color = if b_score == w_score {
            Empty
        } else if b_score > w_score {
            Black
        } else {
            White
        };
        Score {color: color, score: score, white_stones: ws, black_stones: bs}
    }

    // TODO: Make this private
    pub fn score_tt(board: &Board) -> (usize, usize) {
        let mut black_score = 0;
        let mut white_score = 0;
        for point in board.points().iter() {
            match point.color {
                Black => { black_score += 1; },
                Empty => {},
                White => { white_score += 1; },
            }
        }
        let mut empty_intersections = board.vacant().clone();
        while empty_intersections.len() > 0 {
            let territory = Score::build_territory_chain(empty_intersections[0], board);

            match territory.color {
                Black => black_score += territory.coords.len(),
                White => white_score += territory.coords.len(),
                Empty => () // This territory is not enclosed by a single color
            }

            empty_intersections = empty_intersections.into_iter().filter(|coord| !territory.coords.contains(coord)).collect();
        }
        (black_score, white_score)
    }

    fn build_territory_chain(first_intersection: Coord, board: &Board) -> Territory {
        let mut territory_chain = Territory::new();
        let mut to_visit = Vec::new();
        let mut neutral  = false;

        to_visit.push(first_intersection);

        while to_visit.len() > 0 {
            let current_coord = to_visit.pop().unwrap();
            if !territory_chain.coords.contains(&current_coord) {territory_chain.coords.push(current_coord);}

            for &coord in board.neighbours(current_coord).iter() {
                match board.color(&coord) {
                    Empty => if !territory_chain.coords.contains(&coord) {to_visit.push(coord)},
                    col   => if territory_chain.color != Empty && territory_chain.color != col {
                        neutral = true;
                    } else {
                        territory_chain.color = col;
                    }
                }
            }
        }

        if neutral {
            territory_chain.color = Empty;
        }

        territory_chain
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn white_stones(&self) -> usize {
        self.white_stones
    }

    pub fn black_stones(&self) -> usize {
        self.black_stones
    }
}

impl String for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self.color {
            Black => "B+",
            White => "W+",
            Empty => ""
        };
        let score = format!("{}", self.score);
        let s = format!("{}{}", color, score);
        s.fmt(f)
    }
}
