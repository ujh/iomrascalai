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

use ruleset::CGOS;
use ruleset::KgsChinese;
use ruleset::Ruleset;
use version;

use getopts::Matches;
use getopts::Options;
use std::io::Write;
use std::io::stderr;

mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeConfig {
    pub end_of_game_cutoff: f32,
    pub expand_after: usize,
    pub fastplay20_thres: f32,
    pub fastplay5_thres: f32,
    pub rave_equiv: f32,
    pub reuse_subtree: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriorsConfig {
    pub best_move_factor: f32,
    pub capture_many: usize,
    pub capture_one: usize,
    pub empty: usize,
    pub neutral_plays: usize,
    pub neutral_wins: usize,
    pub patterns: usize,
    pub self_atari: usize,
    pub use_empty: bool,
    pub use_patterns: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimerConfig {
    pub c: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayoutConfig {
    pub atari_check: bool,
    pub ladder_check: bool,
    pub last_moves_for_heuristics: usize,
    pub no_self_atari_cutoff: usize,
    pub pattern_probability: f32,
    pub play_in_middle_of_eye: bool,
    pub use_patterns: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub log: bool,
    pub play_out_aftermath: bool,
    pub playout: PlayoutConfig,
    pub priors: PriorsConfig,
    pub ruleset: Ruleset,
    pub threads: usize,
    pub timer: TimerConfig,
    pub tree: TreeConfig,
}

impl Config {

    pub fn default() -> Config {
        let default_toml = include_str!("defaults.toml");
        Config {
            log: false,
            play_out_aftermath: false,
            playout: PlayoutConfig {
                atari_check: true,
                ladder_check: true,
                last_moves_for_heuristics: 2,
                no_self_atari_cutoff: 7,
                pattern_probability: 0.9,
                play_in_middle_of_eye: true,
                use_patterns: true,
            },
            priors: PriorsConfig {
                capture_many: 30,
                capture_one: 15,
                empty: 20,
                neutral_plays: 10,
                neutral_wins: 5,
                patterns: 10,
                self_atari: 10,
                use_empty: true,
                use_patterns: false,
            },
            ruleset: KgsChinese,
            threads: 1,
            timer: TimerConfig {
                c: 0.5
            },
            tree: TreeConfig {
                end_of_game_cutoff: 0.08,
                expand_after: 1,
                fastplay20_thres: 0.8,
                fastplay5_thres: 0.95,
                rave_equiv: 20.0,
                reuse_subtree: true,
            },
        }
    }

    pub fn setup(&self, opts: &mut Options) {
        opts.optflag("h", "help", "Print this help menu");
        opts.optflag("v", "version", "Print the version number");
    }

    pub fn set_from_opts(&mut self, matches: &Matches, opts: &Options, args: &Vec<String>) -> Result<Option<String>, String>{
        if matches.opt_present("h") {
            let brief = format!("Usage: {} [options]", args[0]);
            let s = format!("{}", opts.usage(brief.as_ref()));
            return Ok(Some(s));
        }
        if matches.opt_present("v") {
            let s = format!("Iomrascálaí {}", version::version());
            return Ok(Some(s));
        }
        self.set_ruleset_dependent_defaults();
        self.check()
    }

    pub fn log(&self, s: String) {
        if self.log {
            match stderr().write(format!("{}\n", s).as_bytes()) {
                Ok(_) => {},
                Err(x) => panic!("Unable to write to stderr: {}", x)
            }
        }
    }

    fn check(&self) -> Result<Option<String>, String> {
        if self.playout.ladder_check && !self.playout.atari_check {
            let s = String::from("'--use-ladder-check-in-playouts true' requires '--use-atari-check-in-playouts true'");
            Err(s)
        } else {
            Ok(None)
        }
    }

    fn set_ruleset_dependent_defaults(&mut self) {
        match self.ruleset {
            CGOS => {
                self.play_out_aftermath = true;
            }
            _ => {}
        }
    }

}
