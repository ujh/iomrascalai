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
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

mod test;

#[derive(Debug)]
pub struct OwnershipStatistics {
    config: Arc<Config>,
    komi: f32,
    size: u8,
    stats: HashMap<Coord, (usize, usize, usize)>
}

impl OwnershipStatistics {

    pub fn new(config: Arc<Config>, size: u8, komi: f32) -> OwnershipStatistics {
        let mut os = OwnershipStatistics {
            config: config,
            komi: komi,
            size: size,
            stats: HashMap::new(),
        };
        os.setup();
        os
    }

    pub fn setup(&mut self) {
        let mut stats = HashMap::new();
        for &coord in &Coord::for_board_size(self.size) {
            stats.insert(coord, self.default_entry());
        }
        self.stats = stats;
    }

    pub fn merge(&mut self, score: &Score) {
        for (i, color) in score.owner().iter().enumerate() {
            let coord = Coord::from_index(i, self.size);
            let (b,w,e) = self.stats[&coord];
            match *color {
                Black => {
                    self.stats.insert(coord, (b+1,w,e));
                },
                White => {
                    self.stats.insert(coord, (b,w+1,e));
                },
                Empty => {
                    self.stats.insert(coord, (b,w,e+1));
                },
            }
        }
    }

    pub fn owner(&self, coord: &Coord) -> Color {
        let (b,w,e) = match self.stats.get(&coord) {
            Some(v) => *v,
            None => self.default_entry()
        };
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

    fn value_for_coord(&self, coord: Coord) -> f64 {
        match self.owner(&coord) {
            Black => 1.0,
            White => -1.0,
            Empty => 0.0
        }
    }

    fn default_entry(&self) -> (usize,usize,usize) {
        (1, 1, self.config.scoring.ownership_prior)
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
