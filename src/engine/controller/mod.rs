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

use board::Color;
use board::Move;
use engine::Engine;
use game::Game;
use timer::Timer;

use std::old_io::stdio::stderr;
use std::sync::mpsc::channel;
use std::thread;

pub struct EngineController<'a> {
    engine: Box<Engine + 'a>,
}

impl<'a> EngineController<'a> {

    pub fn new<'b>(engine: Box<Engine + 'b>) -> EngineController<'b> {
        EngineController {
            engine: engine,
        }
    }

    pub fn run_and_return_move(&mut self, color: Color, game: &Game, timer: &mut Timer) -> Move {
        let budget = self.budget(timer, game);
        let (send_to_controller, receive_from_engine) = channel::<Move>();
        thread::scoped(|| {
            self.engine.gen_move(color, game, budget, send_to_controller);
        });
        receive_from_engine.recv().unwrap()
    }

    fn budget(&self, timer: &mut Timer, game: &Game) -> i64 {
        timer.start();
        let budget = timer.budget(game);
        let mut stream = stderr();
        stream.write_line(format!("Thinking for {}ms", budget).as_slice());
        stream.write_line(format!("{}ms time left", timer.main_time_left()).as_slice());
        budget
    }

}
