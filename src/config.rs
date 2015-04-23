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

use ruleset::Minimal;
use ruleset::Ruleset;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UctConfig {
    pub end_of_game_cutoff: f32,
    pub expand_after: usize,
    pub priors: UctPriorsConfig,
    pub reuse_subtree: bool,
    pub tuned: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UctPriorsConfig {
    pub capture_many: usize,
    pub capture_one: usize,
    pub empty: usize,
    pub neutral_plays: usize,
    pub neutral_wins: usize,
    pub self_atari: usize,
    pub use_empty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimerConfig {
    pub c: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayoutConfig {
    pub ladder_check: bool,
    pub no_self_atari_cutoff: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Config {
    pub debug: bool,
    pub log: bool,
    pub playout: PlayoutConfig,
    pub ruleset: Ruleset,
    pub threads: usize,
    pub timer: TimerConfig,
    pub uct: UctConfig,
}

impl Config {

    pub fn default() -> Config {
        Config {
            debug: true,
            log: false,
            playout: PlayoutConfig {
                ladder_check: false,
                no_self_atari_cutoff: 7,
            },
            ruleset: Minimal,
            threads: 1,
            timer: TimerConfig {
                c: 0.5
            },
            uct: UctConfig {
                end_of_game_cutoff: 0.01,
                expand_after: 1,
                priors: UctPriorsConfig {
                    capture_many: 30,
                    capture_one: 15,
                    empty: 10,
                    neutral_plays: 10,
                    neutral_wins: 5,
                    self_atari: 10,
                    use_empty: false,
                },
                reuse_subtree: true,
                tuned: true,
            },
        }
    }

}
