/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
use board::Move;
use config::Config;
use engine::Engine;
use game::Game;
use ownership::OwnershipStatistics;
use score::FinalScore;
use timer::Timer;

use std::sync::Arc;

pub struct EngineController {
    config: Arc<Config>,
    engine: Engine,
    scoring: bool,
}

impl EngineController {

    pub fn new(config: Arc<Config>, engine: Engine) -> EngineController {
        EngineController {
            config: config,
            engine: engine,
            scoring: false,
        }
    }

    pub fn reset(&mut self, size: u8, komi: f32) {
        self.engine.reset(size, komi);
        self.scoring = false;
    }

    pub fn ownership_statistics(&self) -> String {
        format!("{}", self.ownership())
    }

    pub fn final_score(&mut self, game: &Game) -> String {
        self.calculate_score(game);
        FinalScore::new(self.config.clone(), game, self.ownership()).score()
    }

    pub fn final_status_list(&mut self, game: &Game, kind: &str) -> Result<String, String> {
        self.calculate_score(game);
        FinalScore::new(self.config.clone(), game, self.ownership()).status_list(kind)

    }

    pub fn genmove(&mut self, color: Color, game: &Game, timer: &Timer) -> (Move, usize) {
        self.scoring = false;
        self.engine.genmove(color, game, timer)
    }

    pub fn genmove_cleanup(&mut self, color: Color, game: &Game, timer: &Timer) -> (Move, usize) {
        self.scoring = false;
        self.engine.genmove_cleanup(color, game, timer)
    }

    fn ownership(&self) -> &OwnershipStatistics {
        &self.engine.ownership()
    }

    fn calculate_score(&mut self, game: &Game) {
        if self.scoring { return; }
        self.scoring = true;
        self.engine.calculate_score(game);
    }
}
