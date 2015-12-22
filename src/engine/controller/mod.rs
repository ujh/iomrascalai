/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
use config::Config;
use engine::Engine;
use game::Game;
use ownership::OwnershipStatistics;
use score::FinalScore;
use timer::Timer;

use std::sync::Arc;

pub struct EngineController<'a> {
    config: Arc<Config>,
    engine: Box<Engine + 'a>,
}

impl<'a> EngineController<'a> {

    pub fn new<'b>(config: Arc<Config>, engine: Box<Engine + 'b>) -> EngineController<'b> {
        EngineController {
            config: config,
            engine: engine,
        }
    }

    pub fn reset(&mut self, size: u8, komi: f32) {
        self.engine.reset(size, komi);
    }

    pub fn ownership_statistics(&self) -> String {
        format!("{}", self.ownership())
    }

    pub fn final_score(&self, game: &Game) -> String {
        FinalScore::new(self.config.clone(), game, self.ownership()).score()
    }

    pub fn final_status_list(&self, game: &Game, kind: &str) -> Result<String, String> {
        FinalScore::new(self.config.clone(), game, self.ownership()).status_list(kind)

    }

    pub fn run_and_return_move(&mut self, color: Color, game: &Game, timer: &Timer) -> (Move, usize) {
        self.engine.genmove(color, game, timer)
    }

    fn ownership(&self) -> &OwnershipStatistics {
        &self.engine.ownership()
    }

}
