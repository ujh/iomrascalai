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
use board::Coord;
use board::Empty;
use board::Move;
use board::NoMove;
use board::Pass;
use board::Play;
use config::Config;
use game::Game;
use patterns::Matcher;

use std::collections::HashMap;
use std::sync::Arc;
use std::usize;

mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    amaf_plays: usize,
    amaf_wins: usize,
    children: Vec<Node>,
    config: Arc<Config>,
    descendants: usize,
    m: Move,
    plays: usize,
    prior_plays: usize,
    prior_wins: usize,
    wins: usize,
}

impl Node {

    pub fn new(m: Move, config: Arc<Config>) -> Node {
        Node {
            amaf_plays: 0,
            amaf_wins: 0,
            children: vec!(),
            config: config.clone(),
            descendants: 0,
            m: m,
            plays: 0,
            prior_plays: config.tree.priors.neutral_plays,
            prior_wins: config.tree.priors.neutral_wins,
            wins: 0,
        }
    }

    pub fn root(game: &Game, color: Color, config: Arc<Config>) -> Node {
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
            Node::root(game, color, self.config.clone())
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
            match node.m() {
                Play(..) => if game.play(node.m()).is_err() {
                    to_remove.push(index);
                },
                Pass(_) => if self.config.play_out_aftermath && game.winner() != game.next_player() {
                    to_remove.push(index);
                },
                _ => unreachable!()
            }
        }
        to_remove.reverse();
        for &index in to_remove.iter() {
            self.descendants -= self.children[index].descendants;
            self.children.remove(index);
        }
    }

    pub fn find_leaf_and_expand(&mut self, game: &Game, matcher: Arc<Matcher>) -> (Vec<usize>, Vec<Move>, bool, usize) {
        let (path, moves, leaf) = self.find_leaf_and_mark(vec!(), vec!());
        let mut board = game.board();
        for &m in moves.iter() {
            board.play_legal_move(m);
        }
        let previous_desc = leaf.descendants;
        let not_terminal = leaf.expand(&board, matcher);
        if !not_terminal {
            let is_win = board.winner() == leaf.color();
            leaf.mark_as_terminal(is_win);
        }
        let new_desc = leaf.descendants - previous_desc;
        (path, moves, not_terminal, new_desc)
    }

    /// Finds the next leave to simulate. To make sure that different
    /// paths are taken through the tree (as we execute the
    /// simulations in parallel) we already increase the play count
    /// here instead of when recording the wins in the tree.
    pub fn find_leaf_and_mark(&mut self, mut path: Vec<usize>, mut moves: Vec<Move>) -> (Vec<usize>, Vec<Move>, &mut Node) {
        self.record_play();
        if self.is_leaf() {
            (path, moves, self)
        } else {
            let index = self.next_child_index();
            path.push(index);
            moves.push(self.children[index].m());
            self.children[index].find_leaf_and_mark(path, moves)
        }
    }

    fn expand_root(&mut self, game: &Game) {
        if !game.is_over() {
            self.children = game.legal_moves_without_eyes()
                .iter()
                .map(|&m| Node::new(m, self.config.clone()))
                .collect();
            let size = game.size() as usize;
            if self.children.len() <= (size * size / 10) {
                if !self.config.play_out_aftermath || game.winner() == game.next_player() {
                    //don't pass if we're losing on the board on CGOS, but otherwise it's OK
                    self.children.push(Node::new(Pass(game.next_player()), self.config.clone()));
                }
            }

            self.descendants = self.children.len();
        }
 }

    pub fn expand(&mut self, board: &Board, matcher: Arc<Matcher>) -> bool {
        let not_terminal = !board.is_game_over();
        if not_terminal && self.plays >= self.config.tree.expand_after {
            let mut children = board.legal_moves_without_eyes()
                .iter()
                .map(|m| self.new_leaf(board, m, matcher.clone()))
                .collect();

            self.priors(&mut children, board);
            self.children = children;
            let size = board.size() as usize;
            if self.children.len() <= (size * size / 10) {
                let player = board.next_player();
                if !self.config.play_out_aftermath || board.winner() == player {
                    //don't pass if we're losing on the board on CGOS, but otherwise it's OK
                    self.children.push(Node::new(Pass(player), self.config.clone()));
                }

            }
        }


        self.descendants = self.children.len();
        not_terminal
    }

    pub fn priors(&self, children: &mut Vec<Node>, board: &Board) {
        let color = board.next_player().opposite();

        let in_danger = board.chains().iter()
            .filter(|chain| chain.color() == color && chain.coords().len() == 1 && chain.liberties().len() <= 2);

        for one_stone in in_danger {
            if let Some(solution) = board.capture_ladder(one_stone) {
                if let Some(node) = children.iter_mut().find(|c| c.m() == solution) {
                    node.record_even_prior(self.config.tree.priors.capture_one);
                }
            }
        }

        let in_danger = board.chains().iter()
            .filter(|chain| chain.color() == color && chain.coords().len() > 1 && chain.liberties().len() <= 2);

        for many_stones in in_danger {
            if let Some(solution) = board.capture_ladder(many_stones) {
                if let Some(node) = children.iter_mut().find(|c| c.m() == solution) {
                    node.record_even_prior(self.config.tree.priors.capture_many);
                }
            }
        }
    }

    pub fn new_leaf(&self, board: &Board, m: &Move, matcher: Arc<Matcher>) -> Node {
        let mut node = Node::new(*m, self.config.clone());

        if !board.is_not_self_atari(m) {
            // That's a negative prior
            node.record_priors(self.config.tree.priors.self_atari, 0);
        }
        if self.config.tree.priors.use_empty {
            let distance = m.coord().distance_to_border(board.size());
            if distance <= 2 && self.in_empty_area(board, m) {
                if distance <= 1 {
                    // That's a negative prior
                    node.record_priors(self.config.tree.priors.empty, 0);
                } else {
                    node.record_even_prior(self.config.tree.priors.empty);
                }
            }
        }
        if self.config.tree.priors.use_patterns {
            let count = self.matching_patterns_count(board, m, matcher);
            let prior = count * self.config.tree.priors.patterns;
            node.record_even_prior(prior);
        }
        node
    }

    fn matching_patterns_count(&self, board: &Board, m: &Move, matcher: Arc<Matcher>) -> usize {
        matcher.pattern_count(board, &m.coord())
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

    pub fn record_on_path(&mut self, path: &[usize], winner: Color, new_nodes: usize, amaf: &HashMap<Coord, Color>) {
        if self.color() == winner {
            self.record_win();
        }
        // We need to switch the color as we see things from the
        // opponent's point of view now.
        let color = self.color().opposite();
        for child in self.children.iter_mut() {
            if !child.m.is_pass() {
                match amaf.get(&child.m.coord()) {
                    Some(&c) if c == color => {
                        child.record_amaf_play();
                        if color == winner {
                            child.record_amaf_win();
                        }
                    }
                    _ => {}
                }
            }
        }
        if path.len() > 0 {
            self.descendants += new_nodes;
            self.children[path[0]].record_on_path(&path[1..], winner, new_nodes, amaf);
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
        self.win_ratio_with_priors() < cutoff
    }

    pub fn record_win(&mut self) {
        self.wins += 1;
    }

    pub fn record_amaf_win(&mut self) {
        self.amaf_wins += 1;
    }

    pub fn record_play(&mut self) {
        self.plays += 1;
    }

    pub fn record_amaf_play(&mut self) {
        self.amaf_plays += 1;
    }

    pub fn record_priors(&mut self, prior_plays: usize, prior_wins: usize) {
        self.prior_plays += prior_plays;
        self.prior_wins += prior_wins;
    }

    pub fn record_even_prior(&mut self, prior: usize) {
        self.record_priors(prior, prior);
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
            None => Node::new(NoMove, self.config.clone()),
        }
    }

    fn uct_tuned_value(&self, parent_plays: usize) -> f32 {
        const MAX_BERNOULLI_VARIANCE: f32 = 0.25;
        let p = self.win_ratio_with_priors(); //bernoulli distribution parameter
        let variance = p * (1.0 - p);
        let variance_upper_bound = variance + ((2.0 * (parent_plays as f32).ln())/(self.plays as f32)).sqrt();
        let smaller_upper_bound = MAX_BERNOULLI_VARIANCE.min(variance_upper_bound); //can't be greater than the theoretical variance

        p + (((parent_plays as f32).ln()) * smaller_upper_bound / (self.plays as f32)).sqrt()
    }

    fn next_child_index(&self) -> usize {
        let mut index = 0;
        for i in 1..self.children.len() {
            if self.children[i].child_value(self.plays) > self.children[index].child_value(self.plays) {
                index = i;
            }
        }
        index
    }

    fn child_value(&self, parent_plays: usize) -> f32 {
        let uct = self.uct_tuned_value(parent_plays);
        if self.amaf_plays == 0 {
            uct
        } else {
            let aw = self.amaf_wins as f32;
            let ap = self.amaf_plays as f32;
            let p = self.plays as f32;
            let rave_equiv = self.config.tree.rave_equiv;
            let rave_winrate = aw / ap;
            let beta = ap / (ap + p + p * ap / rave_equiv);
            beta * rave_winrate + (1.0 - beta) * uct
        }
    }

    fn win_ratio_with_priors(&self) -> f32 {
        let p = self.plays + self.prior_plays;
        if p == 0 {
            0f32
        } else {
            let w = self.wins + self.prior_wins;
            w as f32 / p as f32
        }
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
