/************************************************************************
 *                                                                      *
 * Copyright 2014-2015 Thomas Poinsot, Urban Hafner                     *
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

pub use self::controller::EngineController;
pub use self::mc::AmafMcEngine;
pub use self::mc::SimpleMcEngine;
pub use self::move_stats::MoveStats;
pub use self::random::RandomEngine;
pub use self::uct::UctEngine;
use board::Color;
use board::Move;
use config::Config;
use game::Game;
use patterns::Matcher;

use std::ascii::AsciiExt;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod controller;
mod mc;
mod move_stats;
mod random;
mod test;
mod uct;

pub fn factory(opt: Option<String>, config: Arc<Config>, matcher: Arc<Matcher>) -> Box<Engine> {
    let engine_arg = opt.map(|s| s.to_ascii_lowercase());
    match engine_arg {
        Some(s) => {
            match s.as_ref() {
                "random" => Box::new(RandomEngine::new()),
                "mc"     => Box::new(SimpleMcEngine::new(config, matcher)),
                "amaf"   => Box::new(AmafMcEngine::new(config, matcher)),
                _        => Box::new(UctEngine::new(config, matcher)),
            }
        },
        None => Box::new(UctEngine::new(config, matcher))
    }
}

pub trait Engine: Send + Sync {

    fn gen_move(&mut self, Color, &Game, sender: Sender<Move>, receiver: Receiver<()>);
    fn engine_type(&self) -> &'static str {
        ""
    }
    fn reset(&mut self) {}

}
