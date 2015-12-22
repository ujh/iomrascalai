/************************************************************************
 *                                                                      *
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
use board::Color;
use board::Coord;
use board::Empty;
use board::White;
use config::Config;
use score::Score;

use core::fmt::Display;
use std::cmp;
use std::fmt;
use std::sync::Arc;

mod test;

#[derive(Debug)]
pub struct OwnershipStatistics {
    black: Vec<usize>,
    config: Arc<Config>,
    empty: Vec<usize>,
    komi: f32,
    size: u8,
    white: Vec<usize>,
}

impl OwnershipStatistics {

    pub fn new(config: Arc<Config>, size: u8, komi: f32) -> OwnershipStatistics {
        let prior = config.scoring.ownership_prior;
        let len = size as usize * size as usize;
        OwnershipStatistics {
            black: vec![0; len],
            config: config,
            empty: vec![prior; len],
            komi: komi,
            size: size,
            white: vec![0; len],
        }
    }

    pub fn merge(&mut self, score: &Score) {
        for (i, color) in score.owner().iter().enumerate() {
            match *color {
                Black => {
                    self.black[i] += 1;
                },
                White => {
                    self.white[i] += 1;
                },
                Empty => {
                    self.empty[i] += 1;
                },
            }
        }
    }

    pub fn owner(&self, coord: &Coord) -> Color {
        let index = coord.to_index(self.size);
        let b = self.black[index];
        let w = self.white[index];
        let e = self.empty[index];
        let count = b + w + e;
        let fraction = cmp::max(b,w) as f32 / count as f32;
        if fraction > self.config.scoring.ownership_cutoff {
            if b > w {
                Black
            } else {
                White
            }
        } else {
            Empty
        }
    }

    pub fn gfx(&self) -> String {
        let mut b = String::from("BLACK");
        let mut w = String::from("WHITE");
        let mut bc = 0;
        let mut wc = 0;
        let mut uc = 0;
        for coord in Coord::for_board_size(self.size) {
            match self.owner(&coord) {
                Black => {
                    b.push_str(&format!(" {}", coord.to_gtp()));
                    bc += 1;
                },
                White => {
                    w.push_str(&format!(" {}", coord.to_gtp()));
                    wc += 1
                },
                Empty => { uc += 1; }
            }
        }
        let text = format!("TEXT Black: {}, White: {}(+{}), Undecided: {}", bc, wc, self.komi, uc);
        format!("gogui-gfx:\nCLEAR\n{}\n{}\n{}\n", b, w, text)
    }

    pub fn decided(&self) -> bool {
        Coord::for_board_size(self.size).iter()
            .all(|coord| self.owner(coord) != Empty)
    }

    pub fn winner(&self) -> Color {
        let mut bs = 0.0;
        let mut ws = self.komi;
        for coord in Coord::for_board_size(self.size) {
            match self.owner(&coord) {
                Black => { bs += 1.0; },
                White => { ws += 1.0; },
                Empty => {}
            }
        }
        if ws == bs {
            Empty
        } else if ws > bs {
            White
        } else {
            Black
        }
    }

    fn value_for_coord(&self, coord: Coord) -> f64 {
        match self.owner(&coord) {
            Black => 1.0,
            White => -1.0,
            Empty => 0.0
        }
    }

}

impl Display for OwnershipStatistics {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in (1u8..self.size+1).rev() {
            for col in 1u8..self.size+1 {
                let coord = Coord::new(col, row);
                s.push_str(&format!("{} ", self.value_for_coord(coord)));
            }
            s.push_str("\n");
        }
        s.fmt(f)
    }
}
