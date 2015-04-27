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

use ruleset::KgsChinese;
use ruleset::Ruleset;

use getopts::Matches;
use std::ascii::OwnedAsciiExt;

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
    pub atari_check: bool,
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

macro_rules! set_if_present {
    ($matches:expr, $opt:expr, $key:expr) => {
        if $matches.opt_present($opt) {
            let arg = $matches.opt_str($opt).unwrap();
            $key = match arg.parse() {
                Ok(v) => v,
                Err(_) => {
                    let s = format!("Unknown value ({}) as argument to --{}", arg, $opt);
                    return Err(s);
                }
            }
        }
    }
}

impl Config {

    pub fn default() -> Config {
        Config {
            debug: true,
            log: false,
            playout: PlayoutConfig {
                atari_check: true,
                ladder_check: false,
                no_self_atari_cutoff: 7,
            },
            ruleset: KgsChinese,
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

    pub fn set_from_opts(&mut self, matches: &Matches) -> Result<(), String>{
        set_if_present!(matches, "empty-area-prior", self.uct.priors.empty);
        set_if_present!(matches, "use-atari-check-in-playouts", self.playout.atari_check);
        set_if_present!(matches, "use-empty-area-prior", self.uct.priors.use_empty);
        set_if_present!(matches, "use-ladder-check-in-playouts", self.playout.ladder_check);
        set_if_present!(matches, "reuse-subtree", self.uct.reuse_subtree);
        let log = matches.opt_present("l");

        let threads = match matches.opt_str("t") {
            Some(s) => {
                match s.parse() {
                    Ok(n)  => n,
                    Err(_) => 1
                }
            },
            None => 1
        };
        let rules_arg = matches.opt_str("r").map(|s| s.into_ascii_lowercase());
        let ruleset = match rules_arg {
            Some(r) => Ruleset::from_string(r),
            None    => KgsChinese
        };

        let policy = matches.opt_str("P").map(|s| s.into_ascii_lowercase());
        self.log = log;
        self.ruleset = ruleset;
        self.threads = threads;
        self.uct.tuned = match policy {
            Some(str) => if str == "ucb1" { false } else { true},
            _ => true
        };
        Ok(())
    }

}
