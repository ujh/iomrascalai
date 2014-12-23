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
use board::Color;
use board::Empty;
use board::White;

use core::fmt::Show;
use std::fmt;
use std::num::Float;


pub struct Score {
    color: Color,
    score: f32
}

mod test;

impl Score {
    pub fn new(scores: (uint, uint), komi: f32) -> Score {
        let (bs, ws) = scores;
        let b_score = bs as f32;
        let w_score = (ws as f32) + komi;
        let score = (b_score - w_score).abs();
        let color = if b_score == w_score {
            Empty
        } else if b_score > w_score {
            Black
        } else {
            White
        };
        Score {color: color, score: score}
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Show for Score {
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
