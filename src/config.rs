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

#[derive(Debug, Copy, Clone)]
pub struct UctConfig {
    pub end_of_game_cutoff: f32,
    pub expand_after: usize,
    pub reuse_subtree: bool,
    pub tuned: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct TimerConfig {
    pub c: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct PlayoutConfig {
    pub no_self_atari_cutoff: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Config {
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
            log: false,
            playout: PlayoutConfig {
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
                reuse_subtree: true,
                tuned: true,
            },
        }
    }

}
