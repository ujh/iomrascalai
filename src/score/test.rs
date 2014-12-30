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
#![cfg(test)]

use super::Score;

#[test]
fn draw() {
    let score = Score::new((5, 5), 0.0);
    assert_eq!("0", format!("{}", score).as_slice());
}

#[test]
fn white_win() {
    let score = Score::new((5, 5), 6.5);
    assert_eq!("W+6.5", format!("{}", score).as_slice());
}

#[test]
fn black_win() {
    let score = Score::new((10, 5), 1.0);
    assert_eq!("B+4", format!("{}", score).as_slice());
}
