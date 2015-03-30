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

#![cfg(test)]

use board::Black;
use board::Pass;
use board::White;
use config::Config;
use game::Game;
use ruleset::KgsChinese;
use super::Node;

use rand::weak_rng;
use std::sync::Arc;
use test::Bencher;

#[test]
fn root_expands_the_children() {
    let game = Game::new(2, 0.5, KgsChinese);
    let root = Node::root(&game, Black);
    assert_eq!(4, root.children.len());
}

#[test]
fn expand_doesnt_add_children_to_terminal_nodes() {
    let mut game = Game::new(5, 6.5, KgsChinese);
    game = game.play(Pass(Black)).unwrap();
    game = game.play(Pass(White)).unwrap();
    let mut node = Node::new(Pass(Black));
    node.expand(&game.board(), Black);
    assert_eq!(0, node.children.len());
}

#[test]
fn run_playout_explores_all_unvisited_children_first() {
    let game = Game::new(2, 0.5, KgsChinese);
    let mut root = Node::root(&game, Black);
    let config = Arc::new(Config::default());
    let mut rng = weak_rng();
    for _ in 0..4 {
        root.run_playout(&game, Black, config.clone(), &mut rng);
    }
    assert_eq!(4, root.children.len());
    assert!(root.children.iter().all(|n| n.plays == 1));
}

#[test]
fn run_playout_expands_the_leaves() {
    let game = Game::new(2, 0.5, KgsChinese);
    let mut root = Node::root(&game, Black);
    let config = Arc::new(Config::default());
    let mut rng = weak_rng();
    for _ in 0..4 {
        root.run_playout(&game, Black, config.clone(), &mut rng);
    }
    assert_eq!(4, root.children.len());
    assert!(root.children.iter().all(|n| n.children.len() == 3));
}

#[test]
fn run_playout_sets_play_on_the_root() {
    let game = Game::new(2, 0.5, KgsChinese);
    let mut root = Node::root(&game, Black);
    let config = Arc::new(Config::default());
    let mut rng = weak_rng();
    root.run_playout(&game, Black, config.clone(), &mut rng);
    assert_eq!(2, root.plays);
}

#[test]
fn the_root_needs_to_be_initialized_with_1_plays_for_correct_uct_calculations() {
    let game = Game::new(2, 0.5, KgsChinese);
    let mut root = Node::root(&game, Black);
    assert_eq!(1, root.plays);
 }


// 2. Make sure that terminal nodes are "played", i.e. either a win or
//    a loss is reported and the wins are recorded in the tree.
// 3. Check if siblings of a terminal node will ever be explored
//    (check the uct value of a terminal node)
// 4. Maybe use the root node's plays and wins to keep track of the
//    number of playouts and the average win rate.
// 5. The game probably doesn't need to be stored in the nodes.
//    Keeping a list of moves and passing those to the playout is
//    probably more efficient. BUT using the current code this means
//    that we don't check for super ko violations in the tree!
// 6. Add a test to make sure that the children of the root node don't
//    violate the super ko rule.
// 7. Test the resigning support
// 8. Make sure everything works fine in the game tree when there are
//    no moves to simulate anymore.
// 9. Implement multi threading


#[bench]
fn uct_playout_19x19(b: &mut Bencher) {
    let game = Game::new(19, 6.5, KgsChinese);
    let mut rng = weak_rng();
    let mut root = Node::root(&game, Black);
    let config = Arc::new(Config::default());
    b.iter(|| root.run_playout(&game, Black, config.clone(), &mut rng));
}
