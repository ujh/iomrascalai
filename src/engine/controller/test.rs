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

#![cfg(test)]
#![allow(unused_must_use)]

use std::sync::Arc;

use board::Color;
use board::Move;
use board::Pass;
use config::Config;
use engine::Engine;
use game::Game;
use ownership::OwnershipStatistics;
use ruleset::Minimal;
use super::EngineController;
use timer::Timer;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use time::PreciseTime;

fn config() -> Arc<Config> {
    Arc::new(Config::default())
}

pub struct EarlyReturnEngine {
    ownership: OwnershipStatistics
}

impl EarlyReturnEngine {

    pub fn new() -> EarlyReturnEngine {
        EarlyReturnEngine {
            ownership: OwnershipStatistics::new(config(), 3)
        }
    }

}

impl Engine for EarlyReturnEngine {

    fn genmove(&mut self, c: Color, _: u32, _: &Game, sender: Sender<(Move,usize)>, _: Receiver<()>) {
        sender.send((Pass(c),0));
    }

    fn ownership(&self) -> &OwnershipStatistics {
        &self.ownership
    }

}

#[test]
fn the_engine_can_use_less_time_than_allocated() {
    let game = Game::new(19, 6.5, Minimal);
    let color = game.board().next_player();
    let timer = Timer::new(config());
    let budget = timer.budget(&game);
    let engine = Box::new(EarlyReturnEngine::new());
    let mut controller = EngineController::new(config(), engine);
    let start_time = PreciseTime::now();
    let (m, _) = controller.run_and_return_move(color, &game, &timer);
    let elapsed_time = start_time.to(PreciseTime::now()).num_milliseconds();
    assert!(elapsed_time < budget as i64);
    assert_eq!(Pass(color), m);
}

pub struct WaitingEngine {
    ownership: OwnershipStatistics
}

impl WaitingEngine {

    pub fn new() -> WaitingEngine {
        WaitingEngine {
            ownership: OwnershipStatistics::new(config(), 3)
        }
    }

}

impl Engine for WaitingEngine {

    fn genmove(&mut self, c: Color, _: u32, _: &Game, sender: Sender<(Move,usize)>, receiver: Receiver<()>) {
        select!(
            _ = receiver.recv() => { sender.send((Pass(c),0)); }
        )
    }

    fn ownership(&self) -> &OwnershipStatistics {
        &self.ownership
    }

}

//TODO: this sometimes fails
#[test]
fn the_controller_asks_the_engine_for_a_move_when_the_time_is_up() {
    let game = Game::new(19, 6.5, Minimal);
    let color = game.board().next_player();
    let mut timer = Timer::new(config());
    timer.setup(1, 0, 0);
    let budget = timer.budget(&game);
    let engine = Box::new(WaitingEngine::new());
    let mut controller = EngineController::new(config(), engine);
    let start_time = PreciseTime::now();
    let (m, _) = controller.run_and_return_move(color, &game, &timer);
    let elapsed_time = start_time.to(PreciseTime::now()).num_milliseconds();
    assert!(elapsed_time >= budget as i64);
    assert_eq!(Pass(color), m);
}
