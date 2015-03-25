/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
 * Copyright 2015 Thomas Poinsot                                        *
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
use board::Pass;
use engine::Engine;
use game::Game;

use rand::random;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct RandomEngine;

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine
    }
}

impl Engine for RandomEngine {

    fn gen_move(&self, color: Color, game: &Game, sender: Sender<Move>, _: Receiver<()>) {
        let mut moves = game.legal_moves();
        moves.push(Pass(color));
        let m = moves[random::<usize>() % moves.len()];
        sender.send(m);
    }

    fn engine_type(&self) -> &'static str {
        "random"
    }

}
