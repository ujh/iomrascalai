/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
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

use engine::Engine;

use board::move::{Move, Pass, Play};
use board::Color;

use game::Game;

use std::rand::random;

pub struct RandomEngine;

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine
    }
}

impl Engine for RandomEngine {
    fn gen_move(&self, color: Color, game: &Game) -> Move {
        let moves = game.legal_moves();
        *moves.get(random::<uint>() % moves.len())
    }
}
