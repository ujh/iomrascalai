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
use board::Color;
use board::Empty;
use engine::Engine;
use engine::random_engine::RandomEngine;
use game::Game;

mod test;

pub struct Playout;

impl Playout {
    pub fn new() -> Playout {
        Playout
    }

    pub fn run(&self, g: Game) -> Color {
        let mut game = g;
        let re = RandomEngine::new();
        while(!game.is_over()) {
            game = game.play(re.gen_move(game.next_player(), &game)).unwrap();
        }
        game.winner()
    }
}
