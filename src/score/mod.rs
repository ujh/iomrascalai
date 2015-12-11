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

pub use self::final_score::FinalScore;
use board::Black;
use board::Board;
use board::Color;
use board::Coord;
use board::Empty;
use board::White;
use self::territory::Territory;

use core::fmt::Display;
use std::fmt;

mod final_score;
mod territory;
mod test;

pub struct Score {
    black_stones: usize,
    komi: f32,
    owner: Vec<Color>,
    white_stones: usize,
}

impl Score {

    pub fn new(board: &Board) -> Score {
        let (bs, ws, owners) = Score::score_tt(board);
        Score {
            black_stones: bs,
            komi: board.komi(),
            owner: owners,
            white_stones: ws
        }
    }

    pub fn empty() -> Score {
        Score {
            black_stones: 0,
            komi: 0.0,
            owner: vec![],
            white_stones: 0,
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

    pub fn owner(&self) -> &Vec<Color> {
        &self.owner
    }

    fn score(&self) -> f32 {
        (self.black_stones as f32 - (self.white_stones as f32 + self.komi)).abs()
    }

    fn score_tt(board: &Board) -> (usize, usize, Vec<Color>) {
        let len = board.size() as usize * board.size() as usize;
        let mut owners = vec![Empty; len];
        Score::count_stones(board, &mut owners);
        Score::count_territory(board, &mut owners);
        let mut black_score = 0;
        let mut white_score = 0;
        for &c in &owners {
            match c {
                Black => { black_score += 1; }
                White => { white_score += 1; }
                Empty => {}
            }
        }
        (black_score, white_score, owners)
    }

    fn count_territory(board: &Board, owners: &mut Vec<Color>) {
        let mut empty_intersections = board.vacant().clone();
        while empty_intersections.len() > 0 {
            let territory = Score::build_territory_chain(empty_intersections[0], board);
            for coord in territory.coords().iter() {
                owners[coord.to_index(board.size())] = territory.color();
            }
            empty_intersections = empty_intersections
                .into_iter()
                .filter(|coord| !territory.contains(coord))
                .collect();
        }
    }

    fn count_stones(board: &Board, owners: &mut Vec<Color>) {
        for (index, point) in board.points().iter().enumerate() {
            owners[index] = point.color;
        }
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

impl Display for Score {
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
