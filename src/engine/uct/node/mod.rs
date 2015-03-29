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
use board::Move;
use board::Pass;
use config::Config;
use game::Game;

use rand::XorShiftRng;
use rand::weak_rng;
use std::f32;
use std::num::Float;
use std::sync::Arc;
use std::usize;

mod test;

pub struct Node {
    children: Vec<Node>,
    game: Game,
    m: Option<Move>,
    plays: usize,
    terminal: bool,
    wins: usize,
}

impl Node {

    pub fn new(game: Game, m: Move) -> Node {
        Node {
            children: vec!(),
            game: game,
            m: Some(m),
            plays: 0,
            terminal: false,
            wins: 0,
        }
    }

    pub fn root(game: &Game) -> Node {
        let mut root = Node {
            children: vec!(),
            game: game.clone(),
            m: None,
            plays: 0,
            terminal: false,
            wins: 0,
        };
        root.expand();
        root
    }

    pub fn run_playout(&mut self, color: Color, config: Arc<Config>, rng: &mut XorShiftRng) {
        let (path, is_win) = {
            let (path, leaf) = self.find_leaf_and_mark(vec!());
            leaf.expand();
            if leaf.is_terminal() {
                let is_win = leaf.game.winner() == color;
                leaf.mark_as_terminal(is_win);
                (path, is_win)
            } else {
                let playout_result = config.playout.run(&leaf.board(), None, rng);
                (path, playout_result.winner() == color)
            }
        };
        if is_win {
            self.record_win_on_path(&path);
        }
    }

    pub fn expand(&mut self) {
        self.terminal = self.game.is_over();
        if !self.terminal {
            let pass = Pass(self.game.next_player());
            let new_game = self.game.play(pass).unwrap();
            self.children = vec!(Node::new(new_game, pass));
            for &m in self.game.legal_moves_without_eyes().iter() {
                let new_game = self.game.play(m).unwrap();
                self.children.push(Node::new(new_game, m));
            }
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn mark_as_terminal(&mut self, is_win: bool) {
        self.plays = usize::MAX;
        if is_win {
            self.wins = usize::MAX;
        } else {
            self.wins = 0;
        }
    }

    pub fn find_leaf_and_mark(&mut self, mut path: Vec<usize>) -> (Vec<usize>, &mut Node) {
        self.record_play();
        if self.is_leaf() {
            (path, self)
        } else {
            let index = self.next_uct_child_index();
            path.push(index);
            self.children[index].find_leaf_and_mark(path)
        }
    }

    pub fn record_win_on_path(&mut self, path: &[usize]) {
        self.record_win();
        if path.len() > 0 {
            self.children[path[0]].record_win_on_path(&path[1..]);
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
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

    pub fn record_win(&mut self) {
        self.wins += 1;
    }

    pub fn record_play(&mut self) {
        self.plays += 1;
    }

    pub fn board(&self) -> Board {
        self.game.board()
    }

    pub fn m(&self) -> Option<Move> {
        self.m
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

}
