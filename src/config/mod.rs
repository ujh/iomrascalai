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

use core::fmt::Display;
use getopts::Matches;
use getopts::Options;
use std::io::Write;
use std::io::stderr;

mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct UctConfig {
    pub end_of_game_cutoff: f32,
    pub expand_after: usize,
    pub priors: UctPriorsConfig,
    pub rave_equiv: usize,
    pub reuse_subtree: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UctPriorsConfig {
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
    pub ruleset: Ruleset,
    pub threads: usize,
    pub timer: TimerConfig,
    pub uct: UctConfig,
}

macro_rules! set_from_opt {
    ($matches:expr, $longopt:expr, $key:expr) => {
        set_from_opt!($matches, "", $longopt, $key);
    };
    ($matches:expr, $shortopt:expr, $longopt:expr, $key:expr) => {
        if $matches.opt_present($longopt) {
            let arg = $matches.opt_str($longopt).unwrap();
            $key = match arg.parse() {
                Ok(v) => v,
                Err(_) => {
                    let strs: Vec<String> = [format!("--{}", $longopt), format!("-{}", $shortopt)].iter()
                        .filter(|&s| s != "")
                        .cloned()
                        .collect();
                    let s = format!("Unknown value ({}) as argument to {}", arg, strs.join(" or "));
                    return Err(s);
                }
            }
        }
    };
}

macro_rules! set_from_flag {
    ($matches:expr, $longopt:expr, $key:expr) => {
        set_from_flag!($matches, "", $longopt, $key);
    };
    ($matches:expr, $shortopt:expr, $longopt:expr, $key:expr) => {
        // Do it with an if so as to not override the default
        if $matches.opt_present($longopt) {
            $key = true;
        }
    };
}

impl Config {

    pub fn default() -> Config {
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
            ruleset: KgsChinese,
            threads: 1,
            timer: TimerConfig {
                c: 0.5
            },
            uct: UctConfig {
                end_of_game_cutoff: 0.08,
                expand_after: 1,
                priors: UctPriorsConfig {
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
                rave_equiv: 2000,
                reuse_subtree: true,
            },
        }
    }

    pub fn setup(&self, opts: &mut Options) {
        opts.optflag("h", "help", "Print this help menu");
        opts.optflag("v", "version", "Print the version number");

        self.flag(opts, "l", "log", "Log to stderr", self.log);

        self.opt(opts, "empty-area-prior", "Prior value for empty areas", self.uct.priors.empty);
        self.opt(opts, "play-out-aftermath", "Keep playing after the result of the game is decided", self.play_out_aftermath);
        self.opt(opts, "play-in-middle-of-eye", "Try playing in the middle of a large eye", self.playout.play_in_middle_of_eye);
        self.opt(opts, "reuse-subtree", "Reuse the subtree from the previous search", self.uct.reuse_subtree);
        self.opt(opts, "use-atari-check-in-playouts", "Check for atari in the playouts", self.playout.ladder_check);
        self.opt(opts, "use-empty-area-prior", "Use a prior for empty areas on the board", self.uct.priors.use_empty);
        self.opt(opts, "use-ladder-check-in-playouts", "Check for ladders in the playouts", self.playout.ladder_check);
        self.opt(opts, "use-patterns-prior", "Use a prior to prioritize 3x3 patterns", self.uct.priors.use_patterns);
        self.opt(opts, "use-patterns-in-playouts", "Use 3x3 patterns in the playouts", self.playout.use_patterns);
        self.optopt(opts, "r", "ruleset", "Select the ruleset", self.ruleset);
        self.optopt(opts, "t", "threads", "Number of threads to use", self.threads);
        self.opt(opts, "rave-equiv", "Weighting between RAVE and UCT term. Set to 0 for pure UCT", self.uct.rave_equiv);
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
        set_from_opt!(matches, "r", "ruleset", self.ruleset);
        self.set_ruleset_dependent_defaults();

        set_from_opt!(matches, "empty-area-prior", self.uct.priors.empty);
        set_from_opt!(matches, "play-out-aftermath", self.play_out_aftermath);
        set_from_opt!(matches, "play-in-middle-of-eye", self.playout.play_in_middle_of_eye);
        set_from_opt!(matches, "reuse-subtree", self.uct.reuse_subtree);
        set_from_opt!(matches, "t", "threads", self.threads);
        set_from_opt!(matches, "use-atari-check-in-playouts", self.playout.atari_check);
        set_from_opt!(matches, "use-empty-area-prior", self.uct.priors.use_empty);
        set_from_opt!(matches, "use-ladder-check-in-playouts", self.playout.ladder_check);
        set_from_opt!(matches, "use-patterns-prior", self.uct.priors.use_patterns);
        set_from_opt!(matches, "use-patterns-in-playouts", self.playout.use_patterns);
        set_from_opt!(matches, "rave-equiv", self.uct.rave_equiv);

        set_from_flag!(matches, "l", "log", self.log);

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

    fn optopt<T: Display + Hint>(&self, opts: &mut Options, shortname: &'static str, name: &'static str, descr: &'static str, default: T) {
        opts.optopt(shortname, name, format!("{} (defaults to {})", descr, default).as_ref(), default.hint_str());
    }

    fn opt<T: Display + Hint>(&self, opts: &mut Options, name: &'static str, descr: &'static str, default: T) {
        self.optopt(opts, "", name, descr, default);
    }

    fn flag(&self, opts: &mut Options, shortname: &'static str, name: &'static str, descr: &'static str, default: bool) {
        opts.optflag(shortname, name, format!("{} (defaults to {})", descr, default).as_ref());
    }
}

pub trait Hint {

    fn hint_str(&self) -> &'static str;

}

impl Hint for bool {

    fn hint_str(&self) -> &'static str {
        "true|false"
    }
}


impl Hint for usize {

    fn hint_str(&self) -> &'static str {
        "NUM"
    }

}
