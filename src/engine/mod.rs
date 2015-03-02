/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
 * Copyright 2015 Thomas Poinsot                                        *
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

pub use self::amaf::AmafEngine;
pub use self::mc::McEngine;
pub use self::move_stats::MoveStats;
pub use self::random::RandomEngine;
use board::Color;
use board::Move;
use game::Game;

mod amaf;
mod mc;
mod move_stats;
mod random;

pub trait Engine {
    // args: color of the move to generate, the game on which we play, and the nb of ms we have to generate the move
    fn gen_move(&self, Color, &Game, i64) -> Move;
}
