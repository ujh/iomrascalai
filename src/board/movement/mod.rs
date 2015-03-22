/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
pub use self::Move::Pass;
pub use self::Move::Play;
pub use self::Move::Resign;
use board::Color;
use board::Coord;

mod test;

#[derive(Debug, Eq, PartialEq, Hash, Copy)]
pub enum Move {
    Pass(Color),
    Play(Color, u8, u8),
    Resign(Color),
}

impl Move {
    pub fn from_gtp(gtp_color: &str, gtp_vertex: &str) -> Move {
        let color = Color::from_gtp(gtp_color);
        let lower_gtp_vertex: String = gtp_vertex.chars().map(|c| c.to_lowercase().next().unwrap()).collect();

        match lower_gtp_vertex.as_slice() {
            "pass"   => { Pass(color) },
            "resign" => { Resign(color) },
            _        => {
                let coord = Coord::from_gtp(gtp_vertex);
                Play(color, coord.col, coord.row)
            }
        }
    }

    pub fn to_gtp(&self) -> String {
        match *self {
            Pass(_)           => String::from_str("pass"),
            Play(_, col, row) => Coord::new(col, row).to_gtp(),
            Resign(_)         => String::from_str("resign"),
        }
    }

    #[inline(always)]
    pub fn color(&self) -> &Color {
        match *self {
            Play(ref c, _, _) => c,
            Pass(ref c)       => c,
            Resign(ref c)     => c,
        }
    }

    #[inline(always)]
    pub fn coord(&self) -> Coord {
        match *self {
            Play(_, col, row) => Coord::new(col, row),
            Pass(_)           => panic!("You have tried to get the coord() of a Pass move"),
            Resign(_)         => panic!("You have tried to get the coord() of a Resign"),
        }
    }

    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        match *self {
            Pass(_) => true,
            _       => false
        }
    }

    pub fn is_resign(&self) -> bool {
        match *self {
            Resign(_) => true,
            _         => false,
        }
    }
}
