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
use config::Config;
use engine::Engine;
use game::Game;
use self::node::Node;

use rand::weak_rng;
use std::old_io::Writer;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod node;

pub struct UctEngine {
    config: Arc<Config>
}

impl UctEngine {

    pub fn new(config: Arc<Config>) -> UctEngine {
        UctEngine { config: config }
    }

}

impl Engine for UctEngine {

    fn gen_move(&self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        let mut root = Node::root(game);
        let mut counter = 0;
        let mut rng = weak_rng();
        loop {
            if receiver.try_recv().is_ok() {
                let best_node = root.best();
                log!("{} simulations", counter);
                log!("Returning the best move({}% wins)", best_node.win_ratio()*100.0);
                sender.send(best_node.m().unwrap());
                break;
            } else {
                root.run_playout(color, self.config.clone(), &mut rng);
                counter += 1;
            }
        }
    }

    fn engine_type(&self) -> &'static str {
        "uct"
    }
}
