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
use board::Pass;
use board::Resign;
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
        let mut root = Node::root(game, color);
        let mut rng = weak_rng();
        if root.has_no_children() {
            if self.config.log {
                log!("No moves to simulate!");
            }
            sender.send(Pass(color));
            return;
        }
        loop {
            if receiver.try_recv().is_ok() {
                finish(root, game, color, sender, self.config.clone());
                break;
            } else {
                root.run_playout(game, color, self.config.clone(), &mut rng);
            }
        }
    }

    fn engine_type(&self) -> &'static str {
        "uct"
    }
}

fn finish(root: Node, game: &Game, color: Color, sender: Sender<Move>, config: Arc<Config>) {
    if root.all_losses() {
        if game.winner() == color {
            sender.send(Pass(color));
        } else {
            sender.send(Resign(color));
        }
        if config.log {
            log!("All simulations were losses");
        }
    } else {
        let best_node = root.best();
        if config.log {
            log!("{} simulations ({}% wins on average)", root.plays()-1, root.win_ratio()*100.0);
            log!("Returning the best move({}% wins)", best_node.win_ratio()*100.0);
        }
        sender.send(best_node.m());
    }
}
