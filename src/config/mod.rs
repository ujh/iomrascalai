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
use version;

use getopts::Matches;
use getopts::Options;

mod test;

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

macro_rules! opt {
    ($matches:expr, $longopt:expr, $key:expr) => {
        opt!($matches, "", $longopt, $key);
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
                    let s = format!("Unknown value ({}) as argument to {}", arg, strs.connect(" or "));
                    return Err(s);
                }
            }
        }
    };
}

macro_rules! flag {
    ($matches:expr, $longopt:expr, $key:expr) => {
        flag!($matches, "", $longopt, $key);
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
            debug: true,
            log: false,
            playout: PlayoutConfig {
                atari_check: true,
                ladder_check: true,
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

    pub fn setup(&self, opts: &mut Options) {
        opts.optflag("h", "help", "print this help menu");
        opts.optflag("l", "log", "log to stderr (defaults to false)");
        opts.optflag("v", "version", "print the version number");

        opts.optopt("", "empty-area-prior", format!("prior value for empty areas (defaults to {})", self.uct.priors.empty).as_ref(), "NUM");
        opts.optopt("", "reuse-subtree", "reuse the subtree from the previous search (defaults to true)", "true|false");
        opts.optopt("", "use-atari-check-in-playouts", format!("Check for atari in the playouts (defaults to {}", self.playout.ladder_check).as_ref(), "true|false");
        opts.optopt("", "use-empty-area-prior", format!("use a prior for empty areas on the board (defaults to {:?})", self.uct.priors.use_empty).as_ref(), "true|false");
        opts.optopt("", "use-ladder-check-in-playouts", format!("Check for ladders in the playouts (defaults to {}", self.playout.ladder_check).as_ref(), "true|false");
        opts.optopt("", "use-ucb1-tuned", format!("Use the UCB1tuned selection strategy (defaults to {})", self.uct.tuned).as_ref(), "true|false");
        opts.optopt("r", "ruleset", "select the ruleset (defaults to chinese)", "cgos|chinese|tromp-taylor|minimal");
        opts.optopt("t", "threads", "number of threads to use (defaults to 1)", "NUM");
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

        opt!(matches, "empty-area-prior", self.uct.priors.empty);
        opt!(matches, "r", "ruleset", self.ruleset);
        opt!(matches, "reuse-subtree", self.uct.reuse_subtree);
        opt!(matches, "t", "threads", self.threads);
        opt!(matches, "use-atari-check-in-playouts", self.playout.atari_check);
        opt!(matches, "use-empty-area-prior", self.uct.priors.use_empty);
        opt!(matches, "use-ladder-check-in-playouts", self.playout.ladder_check);
        opt!(matches, "use-ucb1-tuned", self.uct.tuned);

        flag!(matches, "l", "log", self.log);

        self.check()
    }

    fn check(&self) -> Result<Option<String>, String> {
        if self.playout.ladder_check && !self.playout.atari_check {
            let s = String::from_str("'--use-ladder-check-in-playouts true' requires '--use-atari-check-in-playouts true'");
            Err(s)
        } else {
            Ok(None)
        }
    }
}
