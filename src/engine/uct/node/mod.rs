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

use board::Board;
use board::Color;
use board::Empty;
use board::Move;
use board::NoMove;
use board::Pass;
use config::Config;
use game::Game;

use std::f32;
use std::usize;

mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    config: Config,
    descendants: usize,
    m: Move,
    plays: usize,
    wins: usize,
}

impl Node {

    pub fn new(m: Move, config: Config) -> Node {
        Node {
            children: vec!(),
            config: config,
            descendants: 0,
            m: m,
            plays: config.uct.priors.neutral_plays,
            wins: config.uct.priors.neutral_wins,
        }
    }

    pub fn root(game: &Game, color: Color, config: Config) -> Node {
        let mut root = Node::new(Pass(color), config);
        // So that we don't get NaN on the first UCT calculation
        root.plays = 1;
        // Now that plays is 1, this needs to be one too to keep the
        // win ratio calculations correct.
        root.wins = 1;
        root.expand_root(&game);
        root
    }

    pub fn find_new_root(&self, game: &Game, color: Color) -> Node {
        let mut new_root = self.find_child(game.last_move());
        new_root.make_root(color);
        new_root.remove_illegal_children(game);
        // We don't currently include pass moves in the tree, so
        // we need to handle the case where the opponent plays a
        // pass move separately. This branch also handles the
        // first move where we don't have a tree, yet.
        if new_root.has_no_children() {
            Node::root(game, color, self.config)
        } else {
            new_root
        }
    }

    pub fn make_root(&mut self, color: Color) {
        // Set these values to zero, as the new root is actually a
        // node of the opponent. Otherwise the win ratio would
        // approach 0% as we win the game. And then we would resign!
        self.plays = 0;
        self.wins = 0;
        // The root has to have the color of the player we want to
        // simulate. Otherwise the win statistics are for the wrong
        // player!
        self.m = Pass(color);
    }

    pub fn remove_illegal_children(&mut self, game: &Game) {
        let mut to_remove = vec!();
        for (index, node) in self.children.iter().enumerate() {
            if game.play(node.m()).is_err() {
                to_remove.push(index);
            }
        }
        to_remove.reverse();
        for &index in to_remove.iter() {
            self.descendants -= self.children[index].descendants;
            self.children.remove(index);
        }
    }

    pub fn find_leaf_and_expand(&mut self, game: &Game) -> (Vec<usize>, Vec<Move>, bool, usize) {
        let (path, moves, leaf) = self.find_leaf_and_mark(vec!(), vec!());
        let mut board = game.board();
        for &m in moves.iter() {
            board.play_legal_move(m);
        }
        let previous_desc = leaf.descendants;
        let not_terminal = leaf.expand(&board);
        if !not_terminal {
            let is_win = board.winner() == leaf.color();
            leaf.mark_as_terminal(is_win);
        }
        let new_desc = leaf.descendants - previous_desc;
        (path, moves, not_terminal, new_desc)
    }

    pub fn find_leaf_and_mark(&mut self, mut path: Vec<usize>, mut moves: Vec<Move>) -> (Vec<usize>, Vec<Move>, &mut Node) {
        self.record_play();
        if self.is_leaf() {
            (path, moves, self)
        } else {
            let index = if self.config.uct.tuned {
                self.next_uct_tuned_child_index()
            } else {
                self.next_uct_child_index()
            };
            path.push(index);
            moves.push(self.children[index].m());
            self.children[index].find_leaf_and_mark(path, moves)
        }
    }

    fn expand_root(&mut self, game: &Game) {
        if !game.is_over() {
            self.children = game.legal_moves_without_eyes()
                .iter()
                .map(|&m| Node::new(m, self.config))
                .collect();
            self.descendants = self.children.len();
        }
 }

    pub fn expand(&mut self, board: &Board) -> bool {
        let not_terminal = !board.is_game_over();
        let mut children = vec![];
        if not_terminal && self.plays >= self.config.uct.expand_after {
            children = board.legal_moves_without_eyes()
                .iter()
                .map(|m| self.new_leaf(board, m))
                .collect();
            self.descendants = self.children.len();
        }
        self.priors(&mut children, board);
        self.children = children;
        not_terminal
    }
    
    pub fn priors(&self, children: &mut Vec<Node>, board: &Board) {
            let color = board.next_player().opposite();

            let in_danger = board.chains().iter()
                .filter(|chain| chain.color() == color && chain.coords().len() == 1 && chain.liberties().len() <= 2);
                
            for one_stone in in_danger {
                if let Some(solution) = board.capture_ladder(one_stone) {
                    if let Some(node) = children.iter_mut().find(|c| c.m() == solution) {
                        node.plays += self.config.uct.priors.capture_one;
                        node.wins += self.config.uct.priors.capture_one;
                    }
                }
            }
            
            let in_danger = board.chains().iter()
                .filter(|chain| chain.color() == color && chain.coords().len() > 1 && chain.liberties().len() <= 2);
   
            for many_stones in in_danger {
                if let Some(solution) = board.capture_ladder(many_stones) {
                    if let Some(node) = children.iter_mut().find(|c| c.m() == solution) {
                        node.plays += self.config.uct.priors.capture_many;
                        node.wins += self.config.uct.priors.capture_many;
                    }
                }
            }
    }

    pub fn new_leaf(&self, board: &Board, m: &Move) -> Node {
        let mut node = Node::new(*m, self.config);

        if !board.is_not_self_atari(m) {
            node.plays += self.config.uct.priors.self_atari;
            node.wins += 0; // That's a negative prior
        }
        let distance = m.coord().distance_to_border(board.size());
        if distance <= 2 && self.in_empty_area(board, m) {
            if distance <= 1 {
                node.plays += self.config.uct.priors.empty;
                node.wins += 0; // That's a negative prior
            } else {
                node.plays += self.config.uct.priors.empty;
                node.wins += self.config.uct.priors.empty;
            }
        }
        node
    }
    
    fn in_empty_area(&self, board: &Board, m: &Move) -> bool {
        m.coord().manhattan_distance_three_neighbours(board.size())
            .iter()
            .all(|c| board.color(c) == Empty)
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

    pub fn record_on_path(&mut self, path: &[usize], winner: Color, new_nodes: usize) {
        if self.color() == winner {
            self.record_win();
        }
        if path.len() > 0 {
            self.descendants += new_nodes;
            self.children[path[0]].record_on_path(&path[1..], winner, new_nodes);
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

    pub fn mostly_losses(&self, cutoff: f32) -> bool {
        self.win_ratio() < cutoff
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

    pub fn descendants(&self) -> usize {
        self.descendants
    }

    pub fn find_child(&self, m: Move) -> Node {
        match self.children.iter().find(|c| c.m() == m) {
            Some(node) => node.clone(),
            None => Node::new(NoMove, self.config),
        }
    }

    fn next_uct_tuned_child_index(&self) -> usize {
        let mut index = 0;
        for i in 1..self.children.len() {
            if self.children[i].uct_tuned_value(self.plays) > self.children[index].uct_tuned_value(self.plays) {
                index = i;
            }
        }
        index
    }

    fn uct_tuned_value(&self, parent_plays: usize) -> f32 {
        const MAX_BERNOULLI_VARIANCE: f32 = 0.25;
        let p = self.win_ratio(); //bernoulli distribution parameter
        let variance = p * (1.0 - p);
        let variance_upper_bound = variance + ((2.0 * (parent_plays as f32).ln())/(self.plays as f32)).sqrt();
        let smaller_upper_bound = MAX_BERNOULLI_VARIANCE.min(variance_upper_bound); //can't be greater than the theoretical variance

        p + (((parent_plays as f32).ln()) * smaller_upper_bound / (self.plays as f32)).sqrt()
    }

    fn next_uct_child_index(&self) -> usize {
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
