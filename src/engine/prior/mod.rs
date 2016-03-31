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
use patterns::Matcher;

use std::sync::Arc;

pub struct Prior {
    config: Arc<Config>,
    plays: usize,
    wins: usize,
}

impl Prior {

    pub fn new(board: &Board, m: &Move, matcher: Arc<Matcher>, config: Arc<Config>) -> Prior {
        let mut prior = Prior {
            config: config,
            plays: 0,
            wins: 0,
        };
        prior.calculate(board, m, matcher);
        prior
    }

    pub fn plays(&self) -> usize {
        self.plays
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    fn calculate(&mut self, board: &Board, m: &Move, matcher: Arc<Matcher>) {
        if !board.is_not_self_atari(m) {
            let value = self.config.priors.self_atari;
            self.record_negative_prior(value);
        }
        if self.use_empty() {
            let distance = m.coord().distance_to_border(board.size());
            if distance <= 2 && self.in_empty_area(board, m) {
                let value = self.config.priors.empty;
                if distance <= 1 {
                    self.record_negative_prior(value);
                } else {
                    self.record_even_prior(value);
                }
            }
        }
        if self.use_patterns() {
            let count = self.matching_patterns_count(board, m, matcher);
            let prior = count * self.config.priors.patterns;
            self.record_even_prior(prior);
        }
    }

    fn use_empty(&self) -> bool {
        self.config.priors.empty > 0
    }

    fn in_empty_area(&self, board: &Board, m: &Move) -> bool {
        m.coord().manhattan_distance_three_neighbours(board.size())
            .iter()
            .all(|c| board.color(c) == Empty)
    }

    fn use_patterns(&self) -> bool {
        self.config.priors.patterns > 0
    }

    fn matching_patterns_count(&self, board: &Board, m: &Move, matcher: Arc<Matcher>) -> usize {
        matcher.pattern_count(board, &m.coord())
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
