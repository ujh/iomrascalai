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

use board::Board;
use board::Color;
use board::Empty;
use board::Move;
use board::Pass;
use config::Config;
use game::Game;

use rand::XorShiftRng;
use std::f32;
use std::num::Float;
use std::sync::Arc;
use std::usize;

mod test;

pub struct Node {
    children: Vec<Node>,
    m: Move,
    plays: usize,
    wins: usize,
}

impl Node {

    pub fn new(m: Move) -> Node {
        Node {
            children: vec!(),
            m: m,
            plays: 0,
            wins: 0,
        }
    }

    pub fn root(game: &Game, color: Color) -> Node {
        let mut root = Node::new(Pass(Empty));
        // So that we don't get NaN on the first UCT calculation
        root.plays = 1;
        // Now that plays is 1, this needs to be one too to keep the
        // win ratio calculations correct.
        root.wins = 1;
        root.expand_root(&game, color);
        root
    }

    pub fn find_leaf_and_expand(&mut self, game: &Game, color: Color) -> (Vec<usize>, Vec<Move>, bool) {
        let (path, moves, leaf) = self.find_leaf_and_mark(vec!(), vec!());
        let mut board = game.board();
        for &m in moves.iter() {
            board.play(m);
        }
        let expanded = leaf.expand(&board, color);
        if !expanded {
            let is_win = board.winner() == color;
            leaf.mark_as_terminal(is_win);
        }
        (path, moves, expanded)
    }

    pub fn find_leaf_and_mark(&mut self, mut path: Vec<usize>, mut moves: Vec<Move>) -> (Vec<usize>, Vec<Move>, &mut Node) {
        self.record_play();
        if self.is_leaf() {
            (path, moves, self)
        } else {
            let index = self.next_uct_child_index();
            path.push(index);
            moves.push(self.children[index].m());
            self.children[index].find_leaf_and_mark(path, moves)
        }
    }

    fn expand_root(&mut self, game: &Game, color: Color) {
        if !game.is_over() {
            self.children = game.legal_moves_without_eyes()
                .iter()
                .map(|&m| Node::new(m))
                .collect();
        }
 }

    pub fn expand(&mut self, board: &Board, color: Color) -> bool {
        let expanded = !board.is_game_over();
        if expanded {
            self.children = board.legal_moves_without_eyes()
                .iter()
                .map(|&m| Node::new(m))
                .collect();
        }

        expanded
    }

    pub fn has_no_children(&self) -> bool {
        self.children.len() == 0
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn mark_as_terminal(&mut self, is_win: bool) {
        self.plays = usize::MAX;
        if is_win {
            self.wins = usize::MAX;
        } else {
            self.wins = 0;
        }
    }

    pub fn record_on_path(&mut self, path: &[usize], winner: Color) {
        if self.color() == winner {
            self.record_win();
        }
        if path.len() > 0 {
            self.children[path[0]].record_on_path(&path[1..], winner);
        }
    }

    pub fn best(&self) -> &Node {
        let mut best = &self.children[0];
        for n in self.children.iter() {
            if n.plays > best.plays {
                best = n;
            }
        }
        best
    }

    pub fn all_losses(&self) -> bool {
        self.children
            .iter()
            .all(|n| n.wins == 0)
    }

    pub fn record_win(&mut self) {
        self.wins += 1;
    }

    pub fn record_play(&mut self) {
        self.plays += 1;
    }

    pub fn m(&self) -> Move {
        self.m
    }

    pub fn plays(&self) -> usize {
        self.plays
    }

    pub fn next_uct_child_index(&self) -> usize {
        let mut index = 0;
        for i in 1..self.children.len() {
            if self.children[i].uct_value(self.plays) > self.children[index].uct_value(self.plays) {
                index = i;
            }
        }
        index
    }

    fn uct_value(&self, parent_plays: usize) -> f32 {
        if self.plays == 0 {
            f32::MAX
        } else {
            self.win_ratio() + self.c() * self.confidence(parent_plays)
        }
    }

    fn confidence(&self, parent_plays: usize) -> f32 {
        ((parent_plays as f32).ln()/(self.plays as f32)).sqrt()
    }

    fn c(&self) -> f32 {
        0.44 // sqrt(1/5)
    }

    pub fn win_ratio(&self) -> f32 {
        if self.plays == 0 {
            0f32
        } else {
            (self.wins as f32) / (self.plays as f32)
        }
    }

    pub fn color(&self) -> Color {
        *self.m().color()
    }

}
