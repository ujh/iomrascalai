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
use board::Move;
use engine::Engine;
use game::Game;

mod test;

pub struct Playout<'a, E> {
    engine: &'a E
}

// TODO: Find a way to make this code easier to test. Maybe by using a
// different Engine that specialized for the tests
impl<'a, E: Engine> Playout<'a, E> {
    pub fn new(engine: &E) -> Playout<E> {
        Playout { engine: engine }
    }

    pub fn run(&self, g: &Game) -> Color {
        let mut game = g.clone();
        while !game.is_over() {
            let move = self.gen_move(&game);
            game = game.play(move).unwrap();
        }
        game.winner()
    }

    fn gen_move(&self, game: &Game) -> Move {
        self.engine.gen_move(game.next_player(), game)
    }
}
