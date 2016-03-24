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
use board::Pass;
use config::Config;
use patterns::Matcher;

use std::sync::Arc;

pub struct Prior {
    m: Move,
    plays: usize,
    wins: usize,
}

impl Prior {

    fn new(m: Move) -> Prior {
        Prior {
            m: m,
            plays: 0,
            wins: 0
        }
    }

    pub fn m(&self) -> Move {
        self.m
    }

    pub fn plays(&self) -> usize {
        self.plays
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    fn record_priors(&mut self, plays: usize, wins: usize) {
        self.plays += plays;
        self.wins += wins;
    }

    fn record_even_prior(&mut self, prior: usize) {
        self.record_priors(prior, prior);
    }

}

pub fn calculate(board: &Board, matcher: &Arc<Matcher>, config: &Arc<Config>) -> Vec<Prior> {
    let mut priors: Vec<Prior> = board.legal_moves_without_eyes()
        .iter()
        .map(|m| Prior::new(*m))
        .collect();
    priors.push(Prior::new(Pass(board.next_player())));
    let color = board.next_player().opposite();

    let in_danger = board.chains().
        iter().
        filter(|chain| {
            chain.color() == color && chain.coords().len() == 1 && chain.liberties().len() <= 2
        });

    for one_stone in in_danger {
        if let Some(solution) = board.capture_ladder(one_stone) {
            if let Some(prior) = priors.iter_mut().find(|c| c.m() == solution) {
                prior.record_even_prior(config.priors.capture_one);
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
            if let Some(prior) = priors.iter_mut().find(|c| c.m() == solution) {
                prior.record_even_prior(config.priors.capture_many);
            }
        }
    }
    for prior in priors.iter_mut() {
        child_priors(prior, &board, &matcher, &config);
    }
    priors
}

fn child_priors(prior: &mut Prior, board: &Board, matcher: &Arc<Matcher>, config: &Arc<Config>) {
    let m = &prior.m();
    if m.is_pass() { return; }
    if !board.is_not_self_atari(m) {
        // That's a negative prior
        prior.record_priors(config.priors.self_atari, 0);
    }
    if use_empty(config) {
        let distance = m.coord().distance_to_border(board.size());
        if distance <= 2 && in_empty_area(board, m) {
            if distance <= 1 {
                // That's a negative prior
                prior.record_priors(config.priors.empty, 0);
            } else {
                prior.record_even_prior(config.priors.empty);
            }
        }
    }
    if use_patterns(config) {
        let count = matching_patterns_count(board, m, matcher);
        prior.record_even_prior(count * config.priors.patterns);
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
