/************************************************************************
 *                                                                      *
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
use patterns::LargePatternMatcher;
use patterns::SmallPatternMatcher;

use std::sync::Arc;

pub struct Prior {
    large_pattern_matched: bool,
    m: Move,
    plays: usize,
    wins: usize,
}

impl Prior {

    pub fn new(board: &Board, m: &Move, small_pattern_matcher: &Arc<SmallPatternMatcher>, large_pattern_matcher: &Arc<LargePatternMatcher>, config: Arc<Config>) -> Prior {
        let mut prior = Prior {
            large_pattern_matched: false,
            m: *m,
            plays: 0,
            wins: 0,
        };
        if !m.is_pass() {
            prior.calculate(board, m, small_pattern_matcher, large_pattern_matcher, &config);
        }
        prior
    }

    pub fn plays(&self) -> usize {
        self.plays
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    fn calculate(&mut self, board: &Board, m: &Move, small_pattern_matcher: &Arc<SmallPatternMatcher>, large_pattern_matcher: &Arc<LargePatternMatcher>, config: &Arc<Config>) {
        if !board.is_not_self_atari(m) {
            let value = config.priors.self_atari;
            self.record_negative_prior(value);
        }
        if config.priors.use_empty() {
            let distance = m.coord().distance_to_border(board.size());
            if distance <= 2 && self.in_empty_area(board, m) {
                let value = config.priors.empty;
                if distance <= 1 {
                    self.record_negative_prior(value);
                } else {
                    self.record_even_prior(value);
                }
            }
        }
        if config.priors.use_small_patterns() {
            let count = self.matching_patterns_count(board, m, small_pattern_matcher);
            let prior = count * config.priors.small_patterns;
            self.record_even_prior(prior);
        }
        if config.priors.use_large_patterns() {
            let probability = self.large_pattern_probability(board, m, large_pattern_matcher);
            self.large_pattern_matched = probability > 0.0;
            let prior = config.priors.large_pattern_factor * probability;
            self.record_even_prior(prior.round() as usize);
        }
    }

    fn in_empty_area(&self, board: &Board, m: &Move) -> bool {
        m.coord().manhattan_distance_three_neighbours(board.size())
            .iter()
            .all(|c| board.color(c) == Empty)
    }

    fn matching_patterns_count(&self, board: &Board, m: &Move, matcher: &Arc<SmallPatternMatcher>) -> usize {
        matcher.pattern_count(board, &m.coord())
    }

    fn large_pattern_probability(&self, board: &Board, m: &Move, matcher: &Arc<LargePatternMatcher>) -> f32 {
        matcher.pattern_probability(board, &m.coord())
    }

    fn record_priors(&mut self, plays: usize, wins: usize) {
        self.plays += plays;
        self.wins += wins;
    }

    fn record_even_prior(&mut self, prior: usize) {
        self.record_priors(prior, prior);
    }

    fn record_negative_prior(&mut self, prior: usize) {
        self.record_priors(prior, 0);
    }
}

pub fn calculate(board: Board, child_moves: Vec<Move>, small_pattern_matcher: &Arc<SmallPatternMatcher>, large_pattern_matcher: &Arc<LargePatternMatcher>, config: &Arc<Config>) -> Vec<Prior> {
    let mut priors: Vec<Prior> = child_moves.iter()
        .map(|m| Prior::new(&board, m, small_pattern_matcher, large_pattern_matcher, config.clone()))
        .collect();
    let color = board.next_player().opposite();
    let in_danger = board.chains().iter()
        .filter(|chain| {
            chain.color() == color && chain.coords().len() == 1 && chain.liberties().len() <= 2
        });
    for one_stone in in_danger {
        if let Some(solution) = board.capture_ladder(one_stone) {
            if let Some(prior) = priors.iter_mut().find(|c| c.m == solution) {
                prior.record_even_prior(config.priors.capture_one);
            }
        }
    }
    let in_danger = board.chains().iter()
        .filter(|chain| {
            chain.color() == color && chain.coords().len() > 1 && chain.liberties().len() <= 2
        });
    for many_stones in in_danger {
        if let Some(solution) = board.capture_ladder(many_stones) {
            if let Some(prior) = priors.iter_mut().find(|c| c.m == solution) {
                prior.record_even_prior(config.priors.capture_many);
            }
        }
    }
    priors
}
