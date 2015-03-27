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
            } else {
                // Setup
                let mut node = &mut root;
                let mut path = vec!();
                // Find leaf and record play. The play is recorded
                // here instead of during the backup to discourage
                // exploration of the same path by other threads
                // (search for virtual loss in the literature).
                while !node.is_leaf() {
                    node.record_play();
                    let i = node.next_uct_child_index();
                    path.push(i);
                    node = node.child(i);
                }
                // Found a child. Expand the children.
                node.expand();
                // Run simulation
                let playout_result = self.config.playout.run(&node.board(), None, &mut rng);
                // Record win
                if playout_result.winner() == color {
                    node = &mut root;
                    for &i in path.iter() {
                        node.record_win();
                        node = node.child(i);
                    }
                }
                counter += 1;
            }
        }
    }

    fn engine_type(&self) -> &'static str {
        "uct"
    }
}
