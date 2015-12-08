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
use board::Coord;
use board::Empty;
use board::White;
use score::Score;

use core::fmt::Display;
use std::collections::HashMap;
use std::fmt;

mod test;

pub struct OwnershipStatistics {
    size: u8,
    stats: HashMap<Coord, (usize, usize, usize)>
}

impl OwnershipStatistics {

    pub fn new(size: u8) -> OwnershipStatistics {
        let mut stats = HashMap::new();
        for &coord in &Coord::for_board_size(size) {
            stats.insert(coord, Self::default_stats());
        }
        OwnershipStatistics {
            size: size,
            stats: stats
        }
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

    fn value_for_coord(&self, coord: Coord) -> f64 {
        let (b,w,e) = match self.stats.get(&coord) {
            Some(v) => *v,
            None => Self::default_stats()
        };
        let count = b + w + e;
        let mut fraction = b as f64 / count as f64;
        if fraction < 0.1  {
            fraction = 0.0;
        } else if fraction > 0.9 {
            fraction = 1.0;
        } else {
            fraction = 0.5;
        }
        // Scale to [-1,1]
        (fraction * 2.0) - 1.0
    }

    fn default_stats() -> (usize,usize,usize) {
        (1,1,100)
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
