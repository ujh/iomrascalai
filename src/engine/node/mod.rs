/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
 * Copyright 2016 Urban Hafner                                          *
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
use board::NoMove;
use board::Pass;
use board::Play;
use config::Config;
use game::Game;
use playout::PlayoutResult;
use score::Score;
use super::prior::Prior;

use std::f32;
use std::sync::Arc;

mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    amaf_plays: f32,
    amaf_wins: f32,
    children: Vec<Node>,
    config: Arc<Config>,
    descendants: usize,
    m: Move,
    playouts: usize,
    plays: f32,
    prior_plays: usize,
    prior_wins: usize,
    wins: f32,
}

impl Node {

    pub fn new(m: Move, config: Arc<Config>) -> Node {
        Node {
            amaf_plays: 0.0,
            amaf_wins: 0.0,
            children: vec!(),
            config: config.clone(),
            descendants: 0,
            m: m,
            playouts: 0,
            plays: 0.0,
            prior_plays: config.priors.neutral_plays,
            prior_wins: config.priors.neutral_wins,
            wins: 0.0,
        }
    }

    pub fn root(game: &Game, color: Color, config: Arc<Config>) -> Node {
        let mut root = Node::new(Pass(color), config);
        root.reset();
        // So that we don't get NaN on the first UCT calculation
        root.plays = 1.0;
        // Now that plays is 1, this needs to be one too to keep the
        // win ratio calculations correct.
        root.wins = 1.0;
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
        self.reset();
        // The root has to have the color of the player we want to
        // simulate. Otherwise the win statistics are for the wrong
        // player!
        self.m = Pass(color);
    }

    fn reset(&mut self) {
        self.amaf_plays = 0.0;
        self.amaf_wins = 0.0;
        self.playouts = 0;
        self.plays = 0.0;
        self.prior_plays = 0;
        self.prior_wins = 0;
        self.wins = 0.0;
    }

    pub fn remove_illegal_children(&mut self, game: &Game) {
        let mut to_remove = vec!();

        for (index, node) in self.children.iter().enumerate() {
            match node.m() {
                Play(..) => if game.play(node.m()).is_err() {
                    to_remove.push(index);
                },
                Pass(_) => if game.winner() != game.next_player() {
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

    pub fn find_leaf_and_expand(&mut self, game: &Game) -> (Vec<usize>, Vec<Move>, Vec<Move>) {
        let (path, moves, leaf) = self.find_leaf_and_mark(vec!(), vec!());
        let mut board = game.board();
        for &m in moves.iter() {
            board.play_legal_move(m);
        }
        let (not_terminal, child_moves) = leaf.expand(&board);
        if !not_terminal {
            let is_win = board.winner() == leaf.color();
            leaf.mark_as_terminal(is_win);
        }
        (path, moves, child_moves)
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
            self.children.push(Node::new(Pass(game.next_player()), self.config.clone()));
            self.descendants = self.children.len();
        }
 }

    pub fn expand(&mut self, board: &Board) -> (bool, Vec<Move>) {
        let not_terminal = !board.is_game_over();
        if not_terminal && self.playouts >= self.config.tree.expand_after {
            self.children = board.legal_moves_without_eyes()
                .iter()
                .map(|m| self.new_leaf(m))
                .collect();
            self.children.push(Node::new(Pass(board.next_player()), self.config.clone()));
        }
        self.descendants = self.children.len();
        let child_moves = self.children.iter().map(|n| n.m()).collect();
        (not_terminal, child_moves)
    }

    fn update_prior(&mut self, prior: &Prior) {
        self.prior_plays += prior.plays();
        self.prior_wins += prior.wins();
    }

    pub fn new_leaf(&self, m: &Move) -> Node {
        Node::new(*m, self.config.clone())
    }

    pub fn has_no_children(&self) -> bool {
        self.children.len() == 0
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn mark_as_terminal(&mut self, is_win: bool) {
        self.plays = f32::MAX;
        if is_win {
            self.wins = f32::MAX;
        } else {
            self.wins = 0.0;
        }
    }

    pub fn record_on_path(&mut self, path: &[usize], new_nodes: usize, playout_result: &PlayoutResult) {
        let winner = playout_result.winner();
        let amaf = playout_result.amaf();
        if self.color() == winner {
            self.record_win(playout_result.score());
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
                            child.record_amaf_win(playout_result.score());
                        }
                    }
                    _ => {}
                }
            }
        }
        if path.len() > 0 {
            self.descendants += new_nodes;
            self.children[path[0]].record_on_path(&path[1..], new_nodes, playout_result);
        }
    }

    pub fn record_priors(&mut self, path: &[usize], priors: Vec<Prior>) {
        if path.len() == 0 {
            for (index, prior) in priors.iter().enumerate() {
                self.children[index].update_prior(prior);
            }
        } else {
            self.children[path[0]].record_priors(&path[1..], priors);
        }
    }

    pub fn best(&self) -> (&Node, &Node) {
        let mut best = &self.children[0];
        let mut pass = &self.children[0];
        for n in self.children.iter() {
            if n.m().is_pass() {
                pass = n;
            } else if n.plays > best.plays {
                best = n;
            }
        }
        (best, pass)
    }

    pub fn best2_plays(&self) -> (f32, f32) {
        let mut most_plays = 0.0;
        let mut second_most_plays = 0.0;
        for n in &self.children {
            if n.plays > most_plays {
                most_plays = n.plays;
            } else if n.plays > second_most_plays {
                second_most_plays = n.plays;
            }
        }
        (most_plays, second_most_plays)
    }

    fn weighted_win(&self, score: &Score) -> f32 {
        let weight = self.config.tree.score_weight;
        (weight * score.adjusted()) + (1.0 - weight)
    }

    fn record_win(&mut self, score: &Score) {
        self.wins += self.weighted_win(score);
    }

    fn record_amaf_win(&mut self, score: &Score) {
        self.amaf_wins += self.weighted_win(score);
    }

    fn record_play(&mut self) {
        self.playouts += 1;
        self.plays += 1.0;
    }

    fn record_amaf_play(&mut self) {
        self.amaf_plays += 1.0;
    }

    fn plays_with_prior_factor(&self) -> f32 {
        self.plays as f32 + (self.prior_plays as f32 * self.config.priors.best_move_factor)
    }

    fn wins_with_prior_factor(&self) -> f32 {
        self.wins as f32 + (self.prior_wins as f32 * self.config.priors.best_move_factor)
    }

    pub fn m(&self) -> Move {
        self.m
    }

    pub fn playouts(&self) -> usize {
        self.playouts
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

    fn uct_tuned_value(&self, parent_plays: f32) -> f32 {
        const MAX_BERNOULLI_VARIANCE: f32 = 0.25;
        let p = self.win_ratio_with_priors(); //bernoulli distribution parameter
        let variance = p * (1.0 - p);
        let variance_upper_bound = variance + ((2.0 * parent_plays.ln())/self.plays_with_prior_factor()).sqrt();
        let smaller_upper_bound = MAX_BERNOULLI_VARIANCE.min(variance_upper_bound); //can't be greater than the theoretical variance

        p + ((parent_plays.ln()) * smaller_upper_bound / self.plays_with_prior_factor()).sqrt()
    }

    fn next_child_index(&self) -> usize {
        let mut best = 0;
        let plays = self.plays_with_prior_factor();
        for i in 1..self.children.len() {
            if self.children[i].child_value(plays) > self.children[best].child_value(plays) {
                best = i;
            }
        }
        best
    }

    fn child_value(&self, parent_plays: f32) -> f32 {
        let uct = self.uct_tuned_value(parent_plays);
        if self.amaf_plays == 0.0 {
            uct
        } else {
            let aw = self.amaf_wins;
            let ap = self.amaf_plays;
            let p = self.plays_with_prior_factor();
            let rave_equiv = self.config.tree.rave_equiv;
            let rave_winrate = aw / ap;
            let beta = ap / (ap + p + p * ap / rave_equiv);
            beta * rave_winrate + (1.0 - beta) * uct
        }
    }

    fn win_ratio_with_priors(&self) -> f32 {
        let p = self.plays_with_prior_factor();
        if p == 0.0 {
            0.0
        } else {
            let w = self.wins_with_prior_factor();
            w / p
        }
    }

    pub fn win_ratio(&self) -> f32 {
        if self.plays == 0.0 {
            0f32
        } else {
            self.wins / self.plays
        }
    }

    pub fn color(&self) -> Color {
        *self.m().color()
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

}
