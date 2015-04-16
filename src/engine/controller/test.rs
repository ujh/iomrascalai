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

use board::Color;
use board::Move;
use board::Pass;
use config::Config;
use engine::Engine;
use game::Game;
use ruleset::Minimal;
use super::EngineController;
use timer::Timer;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use time::PreciseTime;

fn config() -> Config {
    Config::default()
}

pub struct EarlyReturnEngine;

impl EarlyReturnEngine {

    pub fn new() -> EarlyReturnEngine {
        EarlyReturnEngine
    }

}

impl Engine for EarlyReturnEngine {

    fn gen_move(&mut self, c: Color, _: &Game, sender: Sender<Move>, _: Receiver<()>) {
        sender.send(Pass(c));
    }

}

#[test]
fn the_engine_can_use_less_time_than_allocated() {
    let game = Game::new(19, 6.5, Minimal);
    let color = game.next_player();
    let timer = Timer::new(config());
    let budget = timer.budget(&game);
    let engine = Box::new(EarlyReturnEngine::new());
    let mut controller = EngineController::new(Config::default(), engine);
    let start_time = PreciseTime::now();
    let (sender, receiver) = channel::<Move>();
    controller.run_and_return_move(color, &game, &timer, sender);
    let m = receiver.recv().unwrap();
    let elapsed_time = start_time.to(PreciseTime::now()).num_milliseconds();
    assert!(elapsed_time < budget as i64);
    assert_eq!(Pass(color), m);
}

pub struct WaitingEngine;

impl WaitingEngine {

    pub fn new() -> WaitingEngine {
        WaitingEngine
    }

}

impl Engine for WaitingEngine {

    fn gen_move(&mut self, c: Color, _: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        select!(
            _ = receiver.recv() => { sender.send(Pass(c)); }
        )
    }

}

//TODO: this sometimes fails
#[test]
fn the_controller_asks_the_engine_for_a_move_when_the_time_is_up() {
    let game = Game::new(19, 6.5, Minimal);
    let color = game.next_player();
    let mut timer = Timer::new(config());
    timer.setup(1, 0, 0);
    let budget = timer.budget(&game);
    let engine = Box::new(WaitingEngine::new());
    let mut controller = EngineController::new(Config::default(), engine);
    let start_time = PreciseTime::now();
    let (sender, receiver) = channel::<Move>();
    controller.run_and_return_move(color, &game, &timer, sender);
    let m = receiver.recv().unwrap();
    let elapsed_time = start_time.to(PreciseTime::now()).num_milliseconds();
    assert!(elapsed_time >= budget as i64);
    assert_eq!(Pass(color), m);
}
