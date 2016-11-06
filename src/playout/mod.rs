/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot, Igor Polyakov           *
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
use board::Coord;
use board::Move;
use board::Pass;
use board::Play;
use config::Config;
use patterns::LargePatternMatcher;
use patterns::SmallPatternMatcher;
use score::Score;

use rand::Rng;
use rand::XorShiftRng;
use std::cmp;
use std::collections::HashMap;
use std::sync::Arc;

mod test;

const ATARI_CUTOFF: usize = 7;

pub struct Playout {
    config: Arc<Config>,
    large_pattern_matcher: Arc<LargePatternMatcher>,
    small_pattern_matcher: Arc<SmallPatternMatcher>
}

impl Playout {

    pub fn new(config: Arc<Config>, large_pattern_matcher: Arc<LargePatternMatcher>, small_pattern_matcher: Arc<SmallPatternMatcher>) -> Playout {
        Playout {
            config: config,
            large_pattern_matcher: large_pattern_matcher,
            small_pattern_matcher: small_pattern_matcher,
        }
    }

    pub fn run(&self, board: &mut Board, initial_move: Option<&Move>, rng: &mut XorShiftRng) -> PlayoutResult {
        let mut played_moves = Vec::new();
        let mut amaf = HashMap::new();
        initial_move.map(|&m| {
            board.play_legal_move(m);
            played_moves.push(m);
            if !m.is_pass() && !amaf.contains_key(&m.coord()) {
                amaf.insert(m.coord(), *m.color());
            }
        });

        let max_moves = self.max_moves(board.size());
        while !board.is_game_over() && played_moves.len() < max_moves {
            let heuristic_set = self.heuristic_set(&played_moves, board, rng);
            let m = self.select_move(board, heuristic_set, rng);
            board.play_legal_move(m);
            played_moves.push(m);
            if !m.is_pass() && !amaf.contains_key(&m.coord()) {
                amaf.insert(m.coord(), *m.color());
            }
        }
        PlayoutResult::new(board.score(), amaf)
    }

    //don't self atari strings that will make an eye after dying, which is strings of 7+
    fn is_playable(&self, board: &Board, m: &Move) -> bool {
        !board.is_eye(&m.coord(), *m.color()) &&
            (board.is_not_self_atari(m) ||
             board.new_chain_length_less_than(*m, ATARI_CUTOFF)) //suicide for smaller groups is ok
    }

    fn max_moves(&self, size: u8) -> usize {
        size as usize * size as usize * 3
    }

    fn heuristic_set(&self, played_moves: &Vec<Move>, board: &Board, rng: &mut XorShiftRng) -> Vec<Coord> {
        let moves_to_consider = self.config.playout.last_moves_for_heuristics as isize;
        let idx = cmp::max(played_moves.len() as isize - moves_to_consider,0) as usize;
        let moves = &played_moves[idx..played_moves.len()];
        let mut coords = vec!();
        // The neighbours of the latest move should come first as we
        // select a matching move from the start of the vector and
        // these should take precedence.
        for i in (0..moves.len()).rev() {
            if !moves[i].is_pass() {
                let mut candidates : Vec<Coord> = board.neighbours(moves[i].coord()).iter().chain(board.diagonals(moves[i].coord())).cloned().collect();
                rng.shuffle(&mut candidates);
                for c in candidates {
                    if !coords.contains(&c) {
                        coords.push(c);
                    }
                }
            }
        }
        coords
    }

    fn select_move(&self, board: &Board, heuristic_set: Vec<Coord>, rng: &mut XorShiftRng) -> Move {
        let color = board.next_player();

        if self.check_for_atari(rng) {
            let possible_move = self.atari_move(color, board, rng);
            if possible_move.is_some() {
                return possible_move.unwrap();
            }
        }
        if self.use_patterns(rng) {
            let possible_move = self.small_pattern_move(color, &heuristic_set, board);
            if possible_move.is_some() {
                return possible_move.unwrap();
            }
        }
        // if self.use_large_patterns(rng) {
        //     let possible_move = self.large_pattern_move(color, &heuristic_set, board, rng);
        //     if possible_move.is_some() {
        //         return possible_move.unwrap();
        //     }
        // }
        if self.do_captures(rng) {
            if let Some(m) = self.capture_move(color, board, rng) {
                return m;
            }
        }
        self.random_move(color, board, rng)
    }

    fn capture_move(&self, color: Color, board: &Board, rng: &mut XorShiftRng) -> Option<Move> {
        let opposite = color.opposite();
        let captures: Vec<_> = board.chains()
            .iter()
            .filter(|chain| chain.color() == opposite && chain.liberties().len() == 1)
            .flat_map(|chain| chain.liberties().iter())
            .collect();
        if captures.len() == 0 {
            None
        } else {
            let index = rng.gen_range(0, captures.len());
            let coord = captures[index];
            Some(Play(color, coord.col, coord.row))
        }
    }

    // If own group of more than one stone has one liberty, check if it can be captured
    fn atari_move(&self, color: Color, board: &Board, rng: &mut XorShiftRng) -> Option<Move> {
        let mut in_danger = board.chains().iter()
            .filter(|chain| {
                chain.color() == color && chain.coords().len() > 1 && chain.liberties().len() == 1
            });
        match in_danger.next() {
            Some(chain) => {
                let solutions = if self.check_for_ladders(rng) {
                    board.save_group(chain)
                } else {
                    board.fix_atari_no_ladder_check(chain)
                };
                if solutions.len() > 0 { //if we can actually save it
                    let random = rng.gen::<usize>() % solutions.len();
                    Some(solutions[random])
                } else {
                    None
                }
            },
            None => None
        }
    }

    fn small_pattern_move(&self, color: Color, coords: &Vec<Coord>, board: &Board) -> Option<Move> {
        // This works as coords is randomly ordered, so taking the
        // first we find is OK.
        coords.iter()
            .map(|c| Play(color, c.col, c.row))
            .find(|&m| {
                board.is_legal(m).is_ok() && self.small_pattern_matches(board, &m)
            })
    }

    fn large_pattern_move(&self, color: Color, coords: &Vec<Coord>, board: &Board, rng: &mut XorShiftRng) -> Option<Move> {
        // This works as coords is randomly ordered, so taking the
        // first we find is OK.
        coords.iter()
            .map(|c| Play(color, c.col, c.row))
            .find(|&m| {
                board.is_legal(m).is_ok() && self.large_pattern_matches(board, &m, rng)
            })
    }

    fn large_pattern_matches(&self, board: &Board, m: &Move, rng: &mut XorShiftRng) -> bool {
        let p = self.large_pattern_matcher.pattern_probability(board, &m.coord()) *
            self.config.playout.large_pattern_factor;
        rng.gen_range(0f32, 1f32) <= p
    }

    fn small_pattern_matches(&self, board: &Board, m: &Move) -> bool {
        self.small_pattern_matcher.pattern_count(board, &m.coord()) > 0
    }

    fn random_move(&self, color: Color, board: &Board, rng: &mut XorShiftRng) -> Move {
        let vacant = board.vacant();
        let playable_move = vacant
            .iter()
            .map(|c| Play(color, c.col, c.row))
            .position(|m| board.is_legal(m).is_ok() && self.is_playable(board, &m));
        if let Some(first) = playable_move {
            let mut include_pass = 0;
            loop {
                let r = first + rng.gen::<usize>() % (vacant.len() - first + include_pass);

                if r == vacant.len() {
                    return Pass(color);
                }
                let c = vacant[r];
                let m = Play(color, c.col, c.row);
                if board.is_legal(m).is_ok() && self.is_playable(board, &m) {
                    if include_pass == 0 && !board.is_not_self_atari(&m) {
                        include_pass = 1; //try to pass in a seki sometimes
                    } else {
                        return if self.play_in_middle_of_eye(rng) {
                            board.play_in_middle_of_eye(m).unwrap_or(m)
                        } else {
                            m
                        };
                    }
                }
            }
        } else {
            Pass(color)
        }
    }

    fn check_for_ladders(&self, rng: &mut XorShiftRng) -> bool {
        rng.gen_range(0f32, 1f32) <= self.config.playout.ladder_check
    }

    fn check_for_atari(&self, rng: &mut XorShiftRng) -> bool {
        rng.gen_range(0f32, 1f32) <= self.config.playout.atari_check
    }

    fn use_patterns(&self, rng: &mut XorShiftRng) -> bool {
        rng.gen_range(0f32, 1f32) <= self.config.playout.pattern_probability
    }

    fn do_captures(&self, rng: &mut XorShiftRng) -> bool {
        rng.gen_range(0f32, 1f32) <= self.config.playout.captures_probability
    }

    fn use_large_patterns(&self, rng: &mut XorShiftRng) -> bool {
        rng.gen_range(0f32, 1f32) <= self.config.playout.large_pattern_probability
    }

    fn play_in_middle_of_eye(&self, rng: &mut XorShiftRng) -> bool {
        rng.gen_range(0f32, 1f32) <= self.config.playout.play_in_middle_of_eye
    }

}

pub struct PlayoutResult {
    amaf: HashMap<Coord,Color>,
    score: Score,
}

impl PlayoutResult {

    pub fn new(score: Score, amaf: HashMap<Coord,Color>) -> PlayoutResult {
        PlayoutResult {
            amaf: amaf,
            score: score,
        }
    }

    pub fn winner(&self) -> Color {
        self.score.color()
    }

    pub fn score(&self) -> &Score {
        &self.score
    }

    pub fn amaf(&self) -> &HashMap<Coord,Color> {
        &self.amaf
    }
}
