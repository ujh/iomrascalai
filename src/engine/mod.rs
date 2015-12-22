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
pub use self::engine_impl::EngineImpl;
use board::Color;
use board::Move;
use config::Config;
use game::Game;
use ownership::OwnershipStatistics;
use patterns::Matcher;
use timer::Timer;

use std::sync::Arc;

mod controller;
mod engine_impl;
mod test;

pub fn factory(config: Arc<Config>, matcher: Arc<Matcher>) -> Box<Engine> {
    Box::new(EngineImpl::new(config, matcher))
}

pub trait Engine {

    fn genmove(&mut self, Color, &Game, &Timer) -> (Move,usize);
    fn ownership(&self) -> &OwnershipStatistics;
    fn reset(&mut self, _:u8, _:f32) {}

}
