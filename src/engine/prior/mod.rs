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
use dcnn::DCNN;
use patterns::Matcher;

use rand::Rng;
use rand::XorShiftRng;
use std::sync::Arc;

pub struct Prior {
    m: Move,
    plays: f32,
    wins: f32,
}

impl Prior {

    pub fn new(board: &Board, m: &Move, matcher: &Arc<Matcher>, config: Arc<Config>, dcnn_prior: f32, rng: &mut XorShiftRng) -> Prior {
        let mut prior = Prior {
            m: *m,
            plays: dcnn_prior + rng.gen_range(0f32, 0.05),
            wins: dcnn_prior + rng.gen_range(0f32, 0.05),
        };
        if !m.is_pass() && board.size() != 19 {
            prior.calculate(board, m, matcher, &config);
        }
        prior
    }

    pub fn plays(&self) -> f32 {
        self.plays
    }

    pub fn wins(&self) -> f32 {
        self.wins
    }

    fn calculate(&mut self, board: &Board, m: &Move, matcher: &Arc<Matcher>, config: &Arc<Config>) {
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
        if config.priors.use_patterns() {
            let count = self.matching_patterns_count(board, m, matcher);
            let prior = count as f32 * config.priors.patterns;
            self.record_even_prior(prior);
        }
    }

    fn in_empty_area(&self, board: &Board, m: &Move) -> bool {
        m.coord().manhattan_distance_three_neighbours(board.size())
            .iter()
            .all(|c| board.color(c) == Empty)
    }

    fn matching_patterns_count(&self, board: &Board, m: &Move, matcher: &Arc<Matcher>) -> usize {
        matcher.pattern_count(board, &m.coord())
    }

    fn record_priors(&mut self, plays: f32, wins: f32) {
        self.plays += plays;
        self.wins += wins;
    }

    pub fn record_even_prior(&mut self, prior: f32) {
        self.record_priors(prior, prior);
    }

    fn record_negative_prior(&mut self, prior: f32) {
        self.record_priors(prior, 0.0);
    }
}

pub fn calculate(board: Board, child_moves: Vec<Move>, matcher: &Arc<Matcher>, config: &Arc<Config>, dcnn: &mut DCNN, rng: &mut XorShiftRng) -> Vec<Prior> {
    let dcnn_priors = calculate_dcnn_priors(&board, dcnn);
    let size = board.size();
    let mut priors: Vec<Prior> = child_moves.iter()
        .map(|m| {
            let dcnn_prior = if m.is_pass() {
                0.0
            } else {
                if board.size() == 19 {
                    let idx = m.coord().to_index(size);
                    dcnn_priors[idx]
                } else {
                    0.0
                }
            };
            Prior::new(&board, m, matcher, config.clone(), dcnn_prior, rng)
        })
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

fn calculate_dcnn_priors(board: &Board, dcnn: &mut DCNN) -> Vec<f32> {
    if board.size() == 19 {
        dcnn.calculate_priors(board)
    } else {
        vec![]
    }
}
