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
use uct_gfx::UctGfx;

use std::sync::Arc;

pub struct EngineController {
    config: Arc<Config>,
    engine: Engine,
    run_playouts_for_scoring: bool,
}

impl EngineController {

    pub fn new(config: Arc<Config>, engine: Engine) -> EngineController {
        EngineController {
            config: config,
            engine: engine,
            run_playouts_for_scoring: true,
        }
    }

    pub fn reset(&mut self, size: u8, komi: f32) {
        self.run_playouts_for_scoring = true;
        self.engine.reset(size, komi);
    }

    pub fn ownership_statistics(&self) -> String {
        format!("{}", self.ownership())
    }
    
    pub fn uct_gfx(&self) -> String {
        format!("{}", UctGfx::new(self.config.clone(), &self.engine.root))
    }

    pub fn final_score(&mut self, game: &Game) -> String {
        self.run_playouts(game);
        FinalScore::new(self.config.clone(), game, self.ownership()).score()
    }

    pub fn final_status_list(&mut self, game: &Game, kind: &str) -> Result<String, String> {
        self.run_playouts(game);
        FinalScore::new(self.config.clone(), game, self.ownership()).status_list(kind)
    }

    pub fn donplayouts(&mut self, game: &Game, playouts: usize) {
        self.run_playouts_for_scoring = false;
        self.engine.donplayouts(game, playouts);
    }

    pub fn genmove(&mut self, color: Color, game: &Game, timer: &Timer) -> (Move, usize) {
        self.run_playouts_for_scoring = true;
        self.engine.genmove(color, game, timer)
    }

    pub fn genmove_cleanup(&mut self, color: Color, game: &Game, timer: &Timer) -> (Move, usize) {
        self.run_playouts_for_scoring = true;
        self.engine.genmove_cleanup(color, game, timer)
    }

    fn ownership(&self) -> &OwnershipStatistics {
        &self.engine.ownership()
    }

    fn run_playouts(&mut self, game: &Game) {
        if !self.run_playouts_for_scoring { return; }
        let playouts = self.config.scoring.playouts;
        self.donplayouts(game, playouts);
    }
}
