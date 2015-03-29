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
    let root = Node::root(&game);
    assert_eq!(5, root.children.len());
}

#[test]
fn expand_adds_the_pass_move_as_a_child() {
    let game = Game::new(2, 0.5, KgsChinese);
    let color = game.next_player();
    let mut node = Node::new(game, Pass(Black));
    node.expand();
    assert!(node.children.iter().any(|n| n.m == Some(Pass(color))));
}

#[test]
fn expand_doesnt_add_children_to_terminal_nodes() {
    let mut game = Game::new(5, 6.5, KgsChinese);
    game = game.play(Pass(Black)).unwrap();
    game = game.play(Pass(White)).unwrap();
    let mut node = Node::new(game, Pass(Black));
    node.expand();
    assert_eq!(0, node.children.len());
}

#[test]
fn run_playout_explores_all_unvisited_children_first() {
    let game = Game::new(2, 0.5, KgsChinese);
    let mut root = Node::root(&game);
    let config = Arc::new(Config::default());
    let mut rng = weak_rng();
    for _ in 0..5 {
        root.run_playout(Black, config.clone(), &mut rng);
    }
    assert_eq!(5, root.children.len());
    assert!(root.children.iter().all(|n| n.plays == 1));
}

#[test]
fn run_playout_expands_the_leaves() {
    let game = Game::new(2, 0.5, KgsChinese);
    let mut root = Node::root(&game);
    let config = Arc::new(Config::default());
    let mut rng = weak_rng();
    for _ in 0..5 {
        root.run_playout(Black, config.clone(), &mut rng);
    }
    assert_eq!(5, root.children.len());
    // The pass move
    assert_eq!(5, root.children[0].children.len());
    // The other moves
    assert!(root.children[1..].iter().all(|n| n.children.len() == 4));
}
#[bench]
fn uct_playout_19x19(b: &mut Bencher) {
    let game = Game::new(19, 6.5, KgsChinese);
    let mut rng = weak_rng();
    let mut root = Node::root(&game);
    let config = Arc::new(Config::default());
    b.iter(|| root.run_playout(Black, config.clone(), &mut rng));
}
