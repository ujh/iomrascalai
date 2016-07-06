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
use super::PATH;

use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
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
        self.all_symmetries().into_iter().collect()
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

    fn all_symmetries(&self) -> HashSet<Pattern> {
        vec!(self.clone(), self.mirrored()).iter()
            .flat_map(|pattern| pattern.rotations())
            .collect()
    }

    fn mirrored(&self) -> Self {
        self.transform(self.mirrored_path())
    }

    fn rotations(&self) -> Vec<Pattern> {
        vec!(
            self.clone(),
            self.rotated90(),
            self.rotated180(),
            self.rotated270()
        )
    }

    fn rotated90(&self) -> Self {
        self.transform(self.rotated90deg_path())
    }

    fn rotated180(&self) -> Self {
        self.rotated90().rotated90()
    }

    fn rotated270(&self) -> Self {
        self.rotated180().rotated90()
    }

    // TODO: Calculate this only once at startup
    fn mirrored_path(&self) -> Vec<(isize, isize)> {
        PATH.iter()
            .map(|&(col, row)| (col*-1,row))
            .collect()
    }

    // TODO: Calulate this only once at startup
    fn rotated90deg_path(&self) -> Vec<(isize, isize)> {
        PATH.iter()
            .map(|&(col, row)| (row,col*-1))
            .collect()
    }

    /// Takes the transformed path (mirrored and/or rotated multiple times) and calculates the
    /// indices into the points array. These new indeces can the be used to create a new pattern
    /// that's been transformed into the new path.
    fn indices_for_new_path(&self, new_path: Vec<(isize, isize)>) -> Vec<usize> {
        new_path.iter()
            .map(|new_c| PATH.iter().position(|old_c| new_c == old_c).unwrap())
            .collect()
    }


    fn transform(&self, new_path: Vec<(isize, isize)>) -> Self {
        let new_indices = self.indices_for_new_path(new_path);
        let new_points = new_indices.iter()
            .take(self.len())
            .map(|&i| self.points[i].clone())
            .collect();
        Pattern {
            points: new_points,
            probability: self.probability
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

// Two patterns are considered equal if their points are equal. We don't care about the pattern
// probability.
impl PartialEq for Pattern {

    fn eq(&self, other: &Pattern) -> bool {
        self.points == other.points
    }
}

impl Eq for Pattern {}

// Two patterns generate the same hash if they have the same points. We don't include the
// probability when calculating the hash.
impl Hash for Pattern {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.points.hash(state);
    }

}
