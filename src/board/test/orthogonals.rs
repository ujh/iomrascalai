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

#![cfg(test)]

use board::Board;
use board::Coord;
use ruleset::Minimal;

#[test]
fn center_has_4_orthogonal_neighbours() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(2, 2));
    assert_eq!(4, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(1, 1)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(1, 3)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(3, 1)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(3, 3)).is_some());
}

#[test]
fn sw_has_1_orthogonal_neighbour() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(1, 1));
    assert_eq!(1, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 2)).is_some());
}

#[test]
fn s_has_2_orthogonal_neighbours() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(2, 1));
    assert_eq!(2, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(1, 2)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(3, 2)).is_some());
}

#[test]
fn se_has_1_orthogonal_neighbour() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(3, 1));
    assert_eq!(1, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 2)).is_some());
}

#[test]
fn w_has_2_orthogonal_neighbours() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(1, 2));
    assert_eq!(2, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 1)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 3)).is_some());
}

#[test]
fn e_has_2_orthogonal_neighbours() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(3, 2));
    assert_eq!(2, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 1)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 3)).is_some());
}

#[test]
fn nw_has_1_orthogonal_neighbour() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(1, 3));
    assert_eq!(1, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 2)).is_some());
}

#[test]
fn n_has_2_orthogonal_neighbours() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(2, 3));
    assert_eq!(2, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(1, 2)).is_some());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(3, 2)).is_some());
}

#[test]
fn ne_has_1_orthogonal_neighbour() {
    let b = Board::new(3, 6.5, Minimal);
    let orthogonals = b.orthogonals(Coord::new(3, 3));
    assert_eq!(1, orthogonals.len());
    assert!(orthogonals.iter().find(|&c| *c == Coord::new(2, 2)).is_some());
}
