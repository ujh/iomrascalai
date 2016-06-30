/************************************************************************
 *                                                                      *
 * Copyright 2016 Urban Hafner                                          *
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

use board::Color;
use self::point::Point;

use std::str::FromStr;

mod point;
mod test;

#[derive(Clone)]
pub struct Pattern {
    points: Vec<Point>,
    probability: f32,
}

impl Pattern {

    pub fn expand(&self) -> Vec<Pattern> {
        vec!(self.clone())
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn probability(&self) -> f32 {
        self.probability
    }

    pub fn matches_color_at(&self, color: Option<Color>, level: usize) -> bool {
        if level >= self.points.len() {
            false
        } else {
            self.points[level].matches(color)
        }
    }
}

impl FromStr for Pattern {

    type Err = String;

    fn from_str(s: &str) -> Result<Pattern, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let probability = parts[0].parse().unwrap();
        let points = parts[1].chars().map(|c| Point::from_char(c)).collect();
        let pattern = Pattern {
            points: points,
            probability: probability,
        };
        Ok(pattern)
    }

}
