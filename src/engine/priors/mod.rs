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
use board::Empty;
use board::Move;
use config::Config;
use game::Game;
use patterns::Matcher;
use super::Node;

use std::sync::Arc;

pub fn calculate(mut nodes: Vec<Node>, game: &Game, moves: &Vec<Move>, matcher: Arc<Matcher>, config: Arc<Config>) -> Vec<Node> {
    let mut board = game.board();
    for &m in moves.iter() {
        board.play_legal_move(m);
    }
    let color = board.next_player().opposite();

    let in_danger = board.chains().
        iter().
        filter(|chain| {
            chain.color() == color && chain.coords().len() == 1 && chain.liberties().len() <= 2
        });

    for one_stone in in_danger {
        if let Some(solution) = board.capture_ladder(one_stone) {
            if let Some(node) = nodes.iter_mut().find(|c| c.m() == solution) {
                node.record_even_prior(config.priors.capture_one);
            }
        }
    }

    let in_danger = board.chains().
        iter().
        filter(|chain| {
            chain.color() == color && chain.coords().len() > 1 && chain.liberties().len() <= 2
        });

    for many_stones in in_danger {
        if let Some(solution) = board.capture_ladder(many_stones) {
            if let Some(node) = nodes.iter_mut().find(|c| c.m() == solution) {
                node.record_even_prior(config.priors.capture_many);
            }
        }
    }
    for n in nodes.iter_mut() {
        child_priors(n, &board, &matcher, &config);
    }
    nodes
}

fn child_priors(node: &mut Node, board: &Board, matcher: &Arc<Matcher>, config: &Arc<Config>) {
    let m = &node.m();
    if m.is_pass() { return; }
    if !board.is_not_self_atari(m) {
        // That's a negative prior
        node.record_priors(config.priors.self_atari, 0);
    }
    if use_empty(config) {
        let distance = m.coord().distance_to_border(board.size());
        if distance <= 2 && in_empty_area(board, m) {
            if distance <= 1 {
                // That's a negative prior
                node.record_priors(config.priors.empty, 0);
            } else {
                node.record_even_prior(config.priors.empty);
            }
        }
    }
    if use_patterns(config) {
        let count = matching_patterns_count(board, m, matcher);
        let prior = count * config.priors.patterns;
        node.record_even_prior(prior);
    }
}

fn use_empty(config: &Arc<Config>) -> bool {
    config.priors.empty > 0
}

fn in_empty_area(board: &Board, m: &Move) -> bool {
    m.coord().manhattan_distance_three_neighbours(board.size())
        .iter()
        .all(|c| board.color(c) == Empty)
}

fn use_patterns(config: &Arc<Config>) -> bool {
    config.priors.patterns > 0
}

fn matching_patterns_count(board: &Board, m: &Move, matcher: &Arc<Matcher>) -> usize {
    matcher.pattern_count(board, &m.coord())
}
